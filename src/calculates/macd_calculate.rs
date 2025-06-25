use crate::calculates::base_calculate::BaseCalculate;
use crate::indicators::candle::Candle;
use crate::indicators::macd::MACD;
use ta::indicators::MovingAverageConvergenceDivergence as Macd;
pub struct MacdCalculate {
    pub(crate) candles: Vec<Candle>,
}

impl BaseCalculate for MacdCalculate {
    fn calculate(&self) -> i64 {
        // 创建MACD指标
        let mut macd = MACD::new(5, 10, 5);
        // 设置显示选项
        macd.set_display_options(true, true, true, true, true);

        // 计算MACD
        let (buy_signals, sell_signals) = macd.calculate(&self.candles.clone());

        let l = self.candles.len();
        if  l < 3 {
            return 0;
        }
        for i in (l - 3)..l {
            if i < macd.macd_line().len() && i < macd.signal_line().len() && i < macd.histogram().len() {
                if buy_signals[i] {
                    return 1;
                }
                if sell_signals[i] {
                    return -1;
                }

                if macd.macd_line()[i-1] > macd.signal_line()[i-1] && macd.macd_line()[i] > macd.signal_line()[i]{
                    return 1;
                }

                if macd.macd_line()[i-1] < macd.signal_line()[i-1] && macd.macd_line()[i] < macd.signal_line()[i]{
                    return -1;
                }
            }
        }
        0
    }

    fn get_name(&self) -> String {
        "MACD".to_string()
    }
    fn get_description(&self) -> String {
        "MACD指标".to_string()
    }
}
