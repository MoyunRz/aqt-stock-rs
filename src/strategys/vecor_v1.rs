use crate::calculates::cyc_calculate::CycCalculate;
use crate::calculates::kdj_calculate::KdjCalculate;
use crate::calculates::macd_calculate::MacdCalculate;
use crate::calculates::stc_calculate::STCCalculate;
use crate::calculates::utbot_calculate::UTBotCalculate;
use crate::computes::calculate::Calculate;
use crate::computes::defult_rules::{CulRules, DefultRules};
use crate::config::config;
use crate::config::config::SymbolConfig;
use crate::indicators::candle::Candle;
use crate::indicators::tradingview_technicals::TradingTechnicals;
use crate::models::market::MarketData;
use crate::services::service::Service;
use crate::strategys::strategy::Strategy;
use log::{info, warn};
use longport::quote::{Candlestick};
use longport::trade::{Order, OrderSide, OrderStatus, StockPosition, StockPositionChannel};
use longport::{decimal, Decimal, QuoteContext, TradeContext};
use std::env;
use std::error::Error;
use std::sync::Arc;
use time::OffsetDateTime;
use crate::calculates::technicals_calculate::TechnicalsCalculate;
use crate::strategys::gconsts::Environment;
use crate::strategys::gconsts::Environment::{DEV, LOCAL, PROD};

/// VecorStrategy 结构体实现了 Strategy trait，用于执行具体的交易策略
pub struct VecorStrategy {
    /// 服务实例，用于访问各种交易和行情数据
    service: Service,
    /// 股票配置映射，存储每个股票的配置信息
    sym_config: Vec<SymbolConfig>,
    env: Environment,
}


impl Strategy for VecorStrategy {
    /// 创建一个新的 VecorStrategy 实例
    fn new(quote_ctx: Arc<QuoteContext>, trade_ctx: Arc<TradeContext>) -> Self {
        let cfgs = config::Configs::load();
        let sym_config = cfgs.unwrap().symbols;
        let mut env = LOCAL;
        // 获取 APP_ENV 配置
          match env::var("APP_ENV") {
            Ok(value) => {
               if value == "dev" {
                   info!("当前环境为开发环境");
                   env = DEV;
               }
               if value == "prod"{
                   info!("当前环境为生产环境");
                   env = PROD;
               }
            },
            Err(e) => {
                warn!("获取APP_ENV配置错误:{}", e);
            },
        }

        VecorStrategy {
            service: Service::new(quote_ctx, trade_ctx),
            sym_config,
            env,
        }
    }

    /// 异步运行策略逻辑
    async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        info!("vecor v1 策略程序开始执行");
        Ok(())
    }

    /// 异步执行策略逻辑，处理传入的市场数据
    async fn execute(&mut self, event: &MarketData) -> Result<(), Box<dyn Error>> {
        // 判断当前的数据时间
        let ts = event.ts.unix_timestamp();
        let market_px = event.price.clone();
        // 只处理收尾的K线
        if ts % 3 <= 3 && !market_px.clone().is_zero() {
            // 获取币种信息
            let sym = VecorStrategy::get_sym_info(self.sym_config.clone(), event.symbol.clone());
            let candles = self
                .service
                .get_candlesticks(event.symbol.clone(), sym.clone().period)
                .await;
            // info!("获取{}股票K线数据", event.symbol.clone());
            // 防止为空
            if candles.clone().is_empty() {
                return Ok(());
            }
            let candles_list = VecorStrategy::handle_candles(event.symbol.clone(), candles.clone());
            // 防止为空
            if candles_list.clone().is_empty() {
                return Ok(());
            }

            // 下单
            // 获取用户的持仓
            let positions = self.service.stock_positions().await;
            let sym_position = VecorStrategy::handler_positions(positions, event.symbol.clone());

            // TODO 判断是否达到收益预期 进行回撤、仓位判断 决定是否抛售
            if VecorStrategy::handler_close_position(
                sym.clone(),
                candles,
                sym_position.clone(),
            ) {
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
                return Ok(());
            }
            // 获取 APP_ENV 配置
            if self.env != DEV {
                // 计算是不是新k线
                let cs = candles_list.clone();
                let le = candles_list.clone().len();
                let pts = cs[le-2].timestamp - cs[le-3].timestamp;
                let lts = cs[le-1].timestamp - cs[le-2].timestamp;
                // 新的k线
                if pts - lts > 10 {
                    return Ok(());
                }
            }
            // TODO 聚合技术判断
            let inds = VecorStrategy::handler_indicators(candles_list,sym.clone()).await;
            info!("对{}进行技术指标聚合判断:{}", event.symbol.clone(), inds);
            if inds == OrderSide::Buy
                && !sym_position.cost_price.is_zero()
                && sym_position.cost_price * decimal!(0.99) <= market_px.clone()
            {
                return Ok(());
            }
            if inds == OrderSide::Sell
                && !sym_position.cost_price.is_zero()
                && sym_position.cost_price >= market_px.clone() * decimal!(0.99)
            {
                return Ok(());
            }
            // TODO 指标指出可以买卖
            if inds != OrderSide::Unknown {
                // info!("获取用户的资金");
                // 获取用户的资金
                let balance = self.service.account_balance().await;
                if balance.is_empty() {
                    return Ok(());
                }
                info!("获取用户的资金{:?}", balance);
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
                let orders = self
                    .service
                    .get_today_orders(event.symbol.clone().as_str())
                    .await;
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
                    return Ok(());
                }

                // 获取订单状态，是否可以下单
                let order_status =
                    VecorStrategy::handler_orders(&self.service, orders, event.symbol.clone())
                        .await;
                if order_status {
                    let resp = self
                        .service
                        .submit_order(event.symbol.clone(), inds, market_px.clone(), quantity)
                        .await;
                    info!("{:?}", resp);
                }
            }
        }
        Ok(())
    }

    /// 停止策略执行
    fn stop(&mut self) -> Result<(), Box<dyn Error>> {
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
                timestamp: c.timestamp.unix_timestamp() as u64,
                open: f64::try_from(c.open).unwrap(),
                high: f64::try_from(c.high).unwrap(),
                low: f64::try_from(c.low).unwrap(),
                close: f64::try_from(c.close).unwrap(),
                volume: c.volume as f64,
            })
            .collect::<Vec<_>>();
        cs
    }

    /// handler_orders 处理订单
    /// - 判断是不是2个小时内下过单
    /// - 判断订单状态是否合适继续下单
    pub async fn handler_orders(service: &Service, orders: Vec<Order>, symbol: String) -> bool {
        if orders.is_empty() {
            return true;
        }
        // let ts = 1_000;
        let h2ts = 2 * 60 * 60;
        // 判断是不是2个小时内下过单
        for o in orders {
            if o.symbol == symbol {
                // 检查订单提交时间是否在最近两小时内
                info!("submitted_at   {:?}",o.submitted_at.clone().unix_timestamp());
                info!("OffsetDateTime {:?}",OffsetDateTime::now_utc().unix_timestamp());
                if o.submitted_at.unix_timestamp()  > OffsetDateTime::now_utc().unix_timestamp()  - h2ts
                {
                    return false; // 若在两小时内返回false，避免频繁下单
                }
                // 判断订单状态是否为新订单、等待提交或部分成交
                if o.status == OrderStatus::New
                    && o.status == OrderStatus::WaitToNew
                    && o.status == OrderStatus::PartialFilled
                    && o.status == OrderStatus::NotReported
                {
                    // 取消订单
                    let _ = service.cancel_order(o.order_id).await;
                    return false; // 如果满足条件则返回false，防止重复操作
                }
            }
        }
        true
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

    pub async fn handler_indicators(
        candles: Vec<Candle>,
        symbol: SymbolConfig,
    ) -> OrderSide {
        // 首先处理异步调用，避免在同步代码中混合异步调用
        let mut sym_str = symbol.symbol;
        sym_str = sym_str.replace(".US", "");
        sym_str = format!("{}:{}", symbol.symbol_type, sym_str);
        let technicals = TradingTechnicals::new(sym_str.as_str()).await;

        let defult_rules = DefultRules {};
        let rules = defult_rules.create();
        let mut calculate = Calculate::new(Box::new(rules));
        let kdj = Box::new(KdjCalculate {
            candles: candles.clone(),
        });
        let macd = Box::new(MacdCalculate {
            candles: candles.clone(),
        });
        let ut_bot = Box::new(UTBotCalculate {
            candles: candles.clone(),
        });
        let stc = Box::new(STCCalculate {
            candles: candles.clone(),
        });
        let cyc = Box::new(CycCalculate {
            candles: candles.clone(),
        });
        let techs = Box::new(TechnicalsCalculate{
            technicals: technicals.clone(),
        });
        
        calculate.add_calculator(kdj);
        calculate.add_calculator(macd);
        calculate.add_calculator(stc);
        calculate.add_calculator(ut_bot);
        calculate.add_calculator(cyc);
        calculate.add_calculator(techs);
        
        let res = calculate.execute_rules();
        if res > 0 {
            return OrderSide::Buy;
        }
        if res < 0 {
            return OrderSide::Sell;
        }
        OrderSide::Unknown
    }

    // 持仓是否达到止盈条件
    pub fn handler_close_position(
        sym: SymbolConfig,
        candle: Vec<Candlestick>,
        stock: StockPosition,
    ) -> bool {
        if candle.len() < 3 || stock.available_quantity.is_zero() {
            return false;
        }
        let cur_price = candle.last().unwrap().close;
        let cost_price = stock.cost_price;
        let tp_ratio = decimal!(sym.tp_ratio) * decimal!(0.01) + decimal!(1);
        if tp_ratio * cost_price < cur_price {
            let prev_price = candle.get(candle.len() - 2).unwrap().close;
            if (prev_price - cur_price) / prev_price > decimal!(0.001) {
                return true;
            }
        }
        false
    }
}
