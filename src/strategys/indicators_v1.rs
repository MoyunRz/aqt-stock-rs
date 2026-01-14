use log::debug;
use longport::trade::OrderSide;
use crate::calculates::cyc_calculate::CycCalculate;
use crate::calculates::kdj_calculate::KdjCalculate;
use crate::calculates::macd_calculate::MacdCalculate;
use crate::calculates::stc_calculate::STCCalculate;
use crate::calculates::technicals_calculate::TechnicalsCalculate;
// use crate::calculates::utbot_calculate::UTBotCalculate;
use crate::computes::calculate::Calculate;
use crate::computes::defult_rules::{CulRules, DefultRules};
use crate::config::config::SymbolConfig;
use crate::indicators::ema::EMA;
use crate::indicators::fibonacci::Fibonacci;
use crate::indicators::hma::HMA;
use crate::indicators::tradingview_technicals::TradingTechnicals;
use crate::models::candle::Candle;
use crate::services::service::Service;
use crate::strategys::ai::{ai_prediction,indicator};

pub struct IndicatorsV1 {}

impl IndicatorsV1 {

    // #[warn(dead_code)]
    pub fn fibonacci(candles: Vec<Candle>) -> f64 {
        let mut fib  = Fibonacci::new();
        // 计算周期内的斐波那契数列
        let pre_close = candles.clone().get(candles.len()-2).unwrap().close;
        let pre_open = candles.clone().get(candles.len()-2).unwrap().open;
        let mark_px = candles.clone().last().unwrap().close;
        let mut h = mark_px.clone();
        let mut l =  mark_px.clone();
        for candle in candles {
            // 看看有没有更高的high
            if candle.high > h {
                h = candle.high;
            }
            if candle.low < l{
                l = candle.low;
            }
        }
        let h_float = h;
        let l_float = l;
        let res = fib.calculate(h_float, l_float);
        match res {
            Ok(res) => {
                // 获取 fibonacci 数列
                // 查看当前价格处于第几序列之间
                let fib_levels = res;
                let levels = fib_levels.clone();
                debug!("------------------- 斐波那契数列 -------------------");
                debug!("市场价格 {:?} ",mark_px.clone());
                debug!("{:?}",levels.clone());
                debug!("--------------------------------------------------");
                // 获取当前价格处于第几序列之间
                for (i, level) in fib_levels.iter().enumerate() {

                    if i == fib_levels.len() - 1 {
                        return 0.0;
                    }
                    if mark_px >= *level && mark_px < levels[i + 1] {

                        // 查看之前的k线是不是在前一个序列之前
                        if i > 3  && pre_close <= levels[i+1]  {
                            // 建仓加仓
                            return 1.0 + i as f64;
                        }
                        if i == fib_levels.len() - 2  && pre_close > pre_open {
                            // 建仓加仓 在最底部，判断是否进行了止跌
                            return 1.0;
                        }
                        // 查看之前的k线是不是在前一个序列之前
                        if (pre_close < pre_open && pre_open > levels[i]) || (pre_close > pre_open && pre_close > levels[i]) {
                            // 清仓
                            return -1.0;
                        }
                    }
                }
                0.0
            },
            Err(e) => panic!("{}", e),
        }
    }


    pub async fn handler_indicators(candles: Vec<Candle>, symbol: SymbolConfig) -> OrderSide {
        // 首先处理异步调用，避免在同步代码中混合异步调用
        let mut sym_str = symbol.clone().symbol;
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
        let cyc = Box::new(CycCalculate {
            candles: candles.clone(),
        });
        let techs = Box::new(TechnicalsCalculate {
            technicals: technicals.clone(),
        });
        let stc = Box::new(STCCalculate {
            candles: candles.clone(),
        });
        // let chip = Box::new(ChipCalculate {
        //     candles: candles.clone(),
        // });
        // let ut_bot = Box::new(UTBotCalculate {
        //     candles: candles.clone(),
        // });
        calculate.add_calculator(kdj);
        calculate.add_calculator(macd);
        calculate.add_calculator(stc);
        // calculate.add_calculator(ut_bot);
        calculate.add_calculator(cyc);
        calculate.add_calculator(techs);
        // calculate.add_calculator(chip);
        let res = calculate.execute_rules();
        if res > 0 {
            return OrderSide::Buy;
        }
        if res < 0 {
            return OrderSide::Sell;
        }
        OrderSide::Unknown
    }

    pub async fn handler_ai_indicators(service: &Service, symbol: &str) -> OrderSide {
        let inobj = indicator::build_inobj(service, symbol).await.unwrap();
        let chat_content = ai_prediction::ds_prediction(symbol, &inobj).await;

        match chat_content {
            Some(content) => {
                if content.action == "BUY" {
                    return OrderSide::Buy;
                }
                if content.action == "SELL" {
                    return OrderSide::Sell;
                }
            },
            None => {}
        }

        OrderSide::Unknown
    }

    pub fn is_close_ema(candles: Vec<Candle>, period: usize)-> bool {
        let mut make_ema  = EMA::new();
        let mut make_hma  = HMA::new();
        let emas = make_ema.calculate(&candles.clone(),period);
        let hmas = make_hma.calculate(&candles.clone(),period);
        if emas.len() > 0 || hmas.len() > 0{
            // 获取candles 前一个 close
            let pre_close = candles.clone().get(candles.len()-2).unwrap().close;
            let close = candles.last().unwrap().close;
            let ema = emas.last().unwrap();
            let hma = hmas.last().unwrap();

            if pre_close > hma.clone() && close < hma.clone() {
                return true;
            }

            if pre_close > ema.unwrap() && close < ema.unwrap() {
                return true;
            }
        }
        false
    }
}