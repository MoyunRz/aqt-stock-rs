use std::collections::HashMap;
use crate::config::config;
use crate::config::config::SymbolConfig;
use crate::models::candle::Candle;
use crate::indicators::tradingview_technicals::TradingTechnicals;
use crate::models::market::MarketData;
use crate::services::service::Service;
use crate::strategys::strategy::Strategy;
use log::{info, warn};
use longport::quote::{Candlestick};
use longport::trade::{ OrderSide, StockPosition, StockPositionChannel};
use longport::{decimal, Decimal, QuoteContext, TradeContext};
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;
use async_trait::async_trait;
use tokio::sync::Mutex;
use tokio::time::sleep;
use crate::strategys::indicators_v1::IndicatorsV1;
use crate::utils::helpers;

/// VecorStrategy 结构体实现了 Strategy trait，用于执行具体的交易策略
pub struct VecorStrategy {
    /// 服务实例，用于访问各种交易和行情数据
    service: Service,
    /// 股票配置映射，存储每个股票的配置信息
    sym_config: Vec<SymbolConfig>,
    last_order_time: Mutex<HashMap<String, i64>>, // symbol -> timestamp
}

#[async_trait]
impl Strategy for VecorStrategy {
    /// 创建一个新的 VecorStrategy 实例
    fn new(quote_ctx: Arc<QuoteContext>, trade_ctx: Arc<TradeContext>) -> Self {
        let cfgs = config::Configs::load();
        let sym_config = cfgs.unwrap().symbols;

        VecorStrategy {
            service: Service::new(quote_ctx, trade_ctx),
            sym_config,
            last_order_time: Mutex::new(HashMap::new()),
        }
    }

    /// 异步运行策略逻辑
    async fn run(&mut self) -> Result<(), Box<dyn Error + Send + Sync>>{
        info!("vecor v1 策略程序开始执行");
        Ok(())
    }

    /// 异步执行策略逻辑，处理传入的市场数据
    async fn execute(&mut self, event: &MarketData) -> Result<(), Box<dyn Error + Send + Sync>>{
        let is_run_time = helpers::do_run_time();
        if !is_run_time {
            tokio::time::sleep(Duration::from_secs(60)).await;
            return Ok(());
        }
        // 获取信息
        let sym = VecorStrategy::get_sym_info(self.sym_config.clone(), event.symbol.clone());
        // 判断当前的数据时间
        let pd = 60*60;
        let ts = event.ts.clone().unix_timestamp();
        let market_px = event.price.clone();
        // 只处理收尾的K线
        // 判断当前价格不为零
        if !market_px.is_zero() && ts % pd <= 3 {
            let lock_delay = Duration::from_secs(5); // 锁延迟释放时间，例如 2 秒
            let mut last_orders = self.last_order_time.lock().await;
            let now_ts = event.ts.unix_timestamp();
            if let Some(last_ts) = last_orders.get(&event.symbol) {
                if now_ts - last_ts < 3600 * 4 { // 1小时内不重复下单
                    sleep(lock_delay).await;
                    return Ok(());
                }
            }
            let candles = self
                .service
                .get_candlesticks(event.symbol.clone(), sym.clone().period)
                .await;
            info!("获取{}股票K线数据", event.symbol.clone());
            // 防止为空
            if candles.clone().is_empty() {
                sleep(lock_delay).await;
                return Ok(());
            }
            let candles_list = VecorStrategy::handle_candles(event.symbol.clone(), candles.clone());
            // 防止为空
            if candles_list.clone().is_empty() {
                sleep(lock_delay).await;
                return Ok(());
            }
            // 获取用户的持仓
            let (positions,ok) = self.service.stock_positions().await;
            if !ok {
                sleep(lock_delay).await;
                return Ok(());
            }
            let sym_position = VecorStrategy::handler_positions(positions, event.symbol.clone());
            // TODO 判断是否达到收益预期 进行回撤、仓位判断 决定是否抛售
            let can_close = VecorStrategy::handler_close_position(sym.clone(), candles, sym_position.clone()).await;
            if can_close {
                info!("{:?}", market_px.clone());
                let resp = self
                    .service
                    .submit_order(
                        event.symbol.clone(),
                        OrderSide::Sell,
                        market_px.clone(),
                        sym_position.available_quantity,
                    )
                    .await;
                info!("{:?}", resp);
                // 允许下单，更新时间
                last_orders.insert(event.symbol.clone(), now_ts);
                sleep(lock_delay).await;
                return Ok(());
            }

            // TODO 聚合技术判断
            let inds = IndicatorsV1::handler_ai_indicators(&self.service, &event.symbol.clone()).await;
            info!("对{} 进行技术指标聚合判断:{}", event.symbol.clone(), inds);
            if inds == OrderSide::Buy
                && !sym_position.cost_price.is_zero()
                && sym_position.cost_price * decimal!(0.99) <= market_px.clone()
            {
                info!("{} 持仓价格:{:?}市场价格:{:?}", event.symbol.clone(),sym_position.cost_price * decimal!(0.99),  market_px.clone());
                return Ok(());
            }
            if inds == OrderSide::Sell
                && !sym_position.cost_price.is_zero()
                && sym_position.cost_price >= market_px.clone() * decimal!(0.99)
            {
                sleep(lock_delay).await;
                info!("{} 持仓价格:{:?}市场价格:{:?}", event.symbol.clone(),sym_position.cost_price * decimal!(0.99),  market_px.clone());
                return Ok(());
            }
            // TODO 指标指出可以买卖
            if inds != OrderSide::Unknown {
                // info!("获取用户的资金");
                // 获取用户的资金
                let balance = self.service.account_balance().await;
                if balance.is_empty() {
                    sleep(lock_delay).await;
                    return Ok(());
                }
                // info!("获取用户的资金{:?}", balance);
                // 循环balance获取美元金额
                let mut usd_bal = Decimal::new(0, 3);
                let mut total_cash = Decimal::new(0, 3);
                for b in balance {
                    for cash_info in b.cash_infos {
                        if cash_info.currency == "USD" {
                            usd_bal = cash_info.withdraw_cash;
                            total_cash = cash_info.available_cash;
                        }
                    }
                }

                // 获取用户的订单
                let orders = self.
                    service.
                    get_today_orders(
                        event.symbol.clone().as_str()
                    ).await;

                if orders.len() > 0 {
                    sleep(lock_delay).await;
                    return Ok(());
                }
                let mut quantity = decimal!(0.0);
                // 根据总资产进行下单
                if usd_bal > decimal!(0.0) && inds == OrderSide::Buy {
                    let volume = sym.volume;
                    let cash = total_cash.checked_mul(decimal!(volume)).unwrap();
                    if usd_bal >= cash * decimal!(1.05) {
                        quantity = (cash / decimal!(market_px.clone())).ceil();
                    }
                }
                if !sym_position.available_quantity.is_zero() && inds == OrderSide::Sell {
                    quantity = sym_position.available_quantity;
                }

                // 数量为0直接返回
                if quantity.is_zero() {
                    sleep(lock_delay).await;
                    return Ok(());
                }

                let resp = self
                    .service
                    .submit_order(event.symbol.clone(), inds, market_px.clone(), quantity)
                    .await;
                // 允许下单，更新时间
                last_orders.insert(event.symbol.clone(), now_ts);
                sleep(lock_delay).await;
                info!("下单成功！！！ {:?}", resp);
            }
        }
        // 在锁释放前休眠
        Ok(())
    }

    /// 停止策略执行
    fn stop(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        warn!("vecor v1 策略程序停止");
        Ok(())
    }
}

// 增加额外的函数
impl VecorStrategy {
    pub fn get_sym_info(sym_config: Vec<SymbolConfig>, symbol: String) -> SymbolConfig {
        let mut sym = SymbolConfig::new();
        for cfg in sym_config.iter() {
            if cfg.symbol == symbol {
                sym = SymbolConfig {
                    symbol: cfg.symbol.clone(),
                    symbol_type: cfg.symbol_type.clone(),
                    volume: cfg.volume.clone(),
                    period: cfg.period.clone(),
                    tp_ratio: cfg.tp_ratio.clone(),
                    sl_ratio: cfg.sl_ratio.clone(),
                };
            }
        }
        sym
    }

    pub fn handle_candles(symbol: String, candles: Vec<Candlestick>) -> Vec<Candle> {
        let candles = candles.clone();
        let cs = candles
            .iter()
            .map(|c| Candle {
                symbol: Option::from(symbol.clone()),
                timestamp: c.timestamp.to_utc().unix_timestamp() as u64,
                open: f64::try_from(c.open).unwrap(),
                high: f64::try_from(c.high).unwrap(),
                low: f64::try_from(c.low).unwrap(),
                close: f64::try_from(c.close).unwrap(),
                volume: c.volume as f64,
            })
            .collect::<Vec<_>>();
        cs
    }

    pub fn handler_positions(
        positions: Vec<StockPositionChannel>,
        symbol: String,
    ) -> StockPosition {
        let mut sym_position = StockPosition {
            symbol: "".to_string(),
            symbol_name: "".to_string(),
            quantity: Default::default(),
            available_quantity: Default::default(),
            currency: "".to_string(),
            cost_price: Default::default(),
            market: Default::default(),
            init_quantity: None,
        };

        if !positions.is_empty() {
            // 持仓是否存在
            for pchannel in positions {
                for p in pchannel.positions {
                    if p.symbol == symbol {
                        sym_position = p;
                    }
                }
            }
        }
        sym_position
    }

    // 持仓是否达到止盈条件
    pub async fn handler_close_position(
        sym: SymbolConfig,
        candle: Vec<Candlestick>,
        stock: StockPosition,
    ) -> bool {
        // 检查蜡烛图数据是否足够且有持仓
        if candle.len() < 3 || stock.available_quantity.is_zero() {
            return false;
        }

        // 获取当前价格和持仓成本价
        let cur_price = candle.last().unwrap().close;
        let cost_price = stock.cost_price;
        // 计算止盈价格（基于配置的止盈比例）10 * 0.01 + 1 =1.01
        let tp_ratio = decimal!(sym.tp_ratio) * decimal!(0.01) + decimal!(1);
        // 如果当前价格高于止盈价格，并且前一个价格出现回落，则触发止盈条件
        // 平均开盘价格 * 收益率 1.01  < 市场价格
        if tp_ratio * cost_price < cur_price {
            let mut sym_str = sym.symbol;
            sym_str = sym_str.replace(".US", "");
            sym_str = format!("{}:{}", sym.symbol_type, sym_str);
            let technicals = TradingTechnicals::new(sym_str.as_str()).await;
            let (summary_signal,_,_) = technicals.clone().calculate();
            if summary_signal < 0f64 {
                return true;
            }
        }
        false
    }
}
