use log::{debug, info};
use crate::calculates::base_calculate::BaseCalculate;
use crate::models::candle::Candle;
use crate::indicators::cyc::CYC;
use crate::indicators::fibonacci::Fibonacci;

pub struct FibonacciCalculate {
    pub candles: Vec<Candle>,
    pub high: f64,
    pub low: f64,
}

impl BaseCalculate for FibonacciCalculate {
    
    fn calculate(&self) -> i64 {

        let mut fib  = Fibonacci::new();
        // 计算60天的斐波那契数列
        let mut h = self.high;
        let mut l =  self.low;
        let mut candles =  self.candles.clone();
        let pre_close = candles.clone().get(candles.len()-3).unwrap().close;
        let pre_open = candles.clone().get(candles.len()-3).unwrap().open;
        let mark_px = candles.clone().last().unwrap().close;
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
                debug!("前收盘价 {:?} ",pre_close.clone());
                debug!("{:?}",levels.clone());
                debug!("--------------------------------------------------");
                // 获取当前价格处于第几序列之间
                for (i, level) in fib_levels.iter().enumerate() {

                    if i == fib_levels.len() - 1 {
                        return 0;
                    }
                    if mark_px >= *level && mark_px < levels[i + 1] {

                        // 查看之前的k线是不是在前一个序列之前
                        if i > 3  && pre_close <= levels[i+1]  {
                            // 建仓加仓
                            return 1;
                        }
                        if i == fib_levels.len() - 2  && pre_close > pre_open {
                            // 建仓加仓 在最底部，判断是否进行了止跌
                            return 2;
                        }
                        // 查看之前的k线是不是在前一个序列之前
                        if (pre_close < pre_open && pre_open > levels[i]) || (pre_close > pre_open && pre_close > levels[i]) {
                            // 清仓
                            return -1;
                        }
                    }
                }
                0
            },
            Err(e) => 0,
        }
    }

    fn get_name(&self) -> String {
        "CYC".to_string()
    }
    fn get_description(&self) -> String {
        "CYC 指标".to_string()
    }
}
