use std::sync::Arc;
use longport::{decimal, Decimal, QuoteContext, TradeContext};
use longport::quote::{Candlestick, MarketTemperature};
use longport::trade::{Order, OrderSide, OrderStatus, StockPosition, StockPositionChannel};
use time::OffsetDateTime;
use crate::config::config;
use crate::config::config::SymbolConfig;
use crate::models::market::MarketData;
use crate::calculates::kdj_calculate::KdjCalculate;
use crate::calculates::macd_calculate::MacdCalculate;
use crate::calculates::market_calculate::MarketCalculate;
use crate::calculates::stc_calculate::STCCalculate;
use crate::calculates::utbot_calculate::UTBotCalculate;
use crate::computes::calculate::Calculate;
use crate::computes::defult_rules::{CulRules, DefultRules};
use crate::indicators::candle::Candle;
use crate::services::service::Service;
use crate::strategys::strategy::{Strategy};

// VecorStrategy 结构体实现了 Strategy trait，用于执行具体的交易策略
pub struct VecorStrategy {
    service: Service, // 服务实例，用于访问各种交易和行情数据
    sym_config: Vec<SymbolConfig>, // 股票配置映射，存储每个股票的配置信息
}

impl Strategy for VecorStrategy {
    // 创建一个新的 VecorStrategy 实例
    fn new(
        quote_ctx: Arc<QuoteContext>,
        trade_ctx: Arc<TradeContext>,
    ) -> Self {
        let cfgs = config::Configs::load();
        let sym_config = cfgs.unwrap().symbols;
        VecorStrategy {
            service: Service::new(quote_ctx, trade_ctx),
            sym_config: sym_config,
        }
    }

    // 异步运行策略逻辑
    async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>>{
        // 模拟运行逻辑
        Ok(())
    }

    // 异步执行策略逻辑，处理传入的市场数据
    async fn execute(&mut self, event: &MarketData) -> Result<(), Box<dyn std::error::Error>> {
        // 判断当前的数据时间
        let ts = event.ts.unix_timestamp();
        let now_ts = OffsetDateTime::now_utc().unix_timestamp();
        // 只处理收尾的K线
        if ts % 300 <= 10 {
            let mut sym = &SymbolConfig { symbol: "".parse().unwrap(), volume: 0.0, period: "".parse().unwrap(), tp_ratio: 0, sl_ratio:0 };
            for cfg in self.sym_config.iter() {
                if cfg.symbol == event.symbol {
                    sym = cfg;
                }
            }
            let candles = self.service.get_candlesticks(sym.symbol.clone(), sym.period.clone()).await;
            // 防止为空
            if candles.clone().is_empty() {
                return Ok(());
            }
            let candles_list = VecorStrategy::handle_candles(candles.clone());
            // 防止为空
            if candles_list.clone().is_empty() {
                return Ok(());
            }
            let candles_last = candles.last().unwrap();
            if candles_last.timestamp.unix_timestamp() > now_ts-5 && candles_last.timestamp.unix_timestamp() < now_ts+5{
                return Ok(());
            }
            // 获取用户的资金
            let balance = self.service.account_balance().await;
            if balance.is_empty() {
                return Ok(());
            }
            // 循环balance获取美元金额
            let mut usd_bal = Decimal::new(0, 3);
            let mut total_cash = Decimal::new(0, 3);
            for b in balance {
                if b.currency == "USD" {
                    usd_bal = b.buy_power;
                    total_cash = b.total_cash;
                }
            }
            // 获取市场热度
            let temperature = self.service.get_market_temperature().await;
            // 聚合判断
            let inds = VecorStrategy::handler_indicators(candles_list, temperature);
            // 下单
            if inds != OrderSide::Unknown {
                // 获取用户的持仓
                let positions = self.service.stock_positions().await;
                let sym_position = VecorStrategy::handler_positions(positions, event.symbol.clone());

                if inds == OrderSide::Buy && sym_position.cost_price < event.close {
                    return Ok(());
                }
                if inds == OrderSide::Sell && sym_position.cost_price > event.close {
                    return Ok(());
                }

                // 获取用户的订单
                let orders = self.service.get_today_orders(event.symbol.clone().as_str()).await;
                let mut quantity = decimal!(0.0);
                // 根据总资产进行下单
                if usd_bal > decimal!(0.0) {
                    let volume = sym.volume;
                    let cash = total_cash.checked_mul(decimal!(volume)).unwrap();
                    if usd_bal >= cash * decimal!(1.05) {
                        quantity = cash / decimal!(event.close);
                    }
                }
                if !sym_position.quantity.is_zero() && inds == OrderSide::Sell {
                    quantity = sym_position.quantity;
                }
                // 获取订单状态，是否可以下单
                if VecorStrategy::handler_orders(orders, event.symbol.clone()) {
                    let _ = self.service.submit_order(event.symbol.clone(), inds, quantity);
                }
            }
        }
        Ok(())
    }

    // 停止策略执行
    fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

// 增加额外的函数
impl VecorStrategy {
    pub fn handle_candles(candles: Vec<Candlestick>) -> Vec<Candle> {
        let candles = candles.clone();
        let cs = candles.iter().map(|c| {
            Candle {
                timestamp: c.timestamp.unix_timestamp() as u64,
                open: f64::try_from(c.open).unwrap(),
                high: f64::try_from(c.high).unwrap(),
                low: f64::try_from(c.low).unwrap(),
                close: f64::try_from(c.close).unwrap(),
                volume: c.volume as f64,
            }
        }).collect::<Vec<_>>();
        cs
    }

    pub fn handler_orders(orders: Vec<Order>, symbol: String) -> bool {
        if orders.is_empty() {
            return false;
        }
        let ts = 1_000_000;
        let h2ts = 2 * 60 * 60 * 1000;
        // 判断是不是2个小时内下过单
        for o in orders {
            if o.symbol == symbol {
                if o.submitted_at.unix_timestamp_nanos() / ts > OffsetDateTime::now_utc().unix_timestamp_nanos() / ts - h2ts {
                    return false;
                }
                if o.status == OrderStatus::New && o.status == OrderStatus::WaitToNew && o.status == OrderStatus::PartialFilled {
                    return false;
                }
            }
        }
        true
    }
    pub fn handler_positions(positions: Vec<StockPositionChannel>, symbol: String) -> StockPosition {
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

    pub fn handler_indicators(candles: Vec<Candle>, market: MarketTemperature) -> OrderSide {
        let defult_rules = DefultRules {};
        let rules = defult_rules.create();
        let mut calculate = Calculate::new(Box::new(rules));
        let mark = Box::new(MarketCalculate {
            market,
        });
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

        calculate.add_calculator(kdj);
        calculate.add_calculator(macd);
        calculate.add_calculator(stc);
        calculate.add_calculator(ut_bot);
        calculate.add_calculator(mark);
        let res = calculate.execute_rules();
        if res > 0 {
            return OrderSide::Buy;
        }
        if res < 0 {
            return OrderSide::Sell;
        }
        OrderSide::Unknown
    }
}

