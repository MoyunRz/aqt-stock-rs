use crate::calculates::base_calculate::BaseCalculate;
use crate::indicators::candle::Candle;
use crate::indicators::schaff_trend_cycle::SchaffTrendCycle;

pub struct STCCalculate {
   pub candles: Vec<Candle>,
}

impl BaseCalculate for STCCalculate {
    fn calculate(&self) -> i64 {
        // 创建STC指标
        let mut stc = SchaffTrendCycle::new(12, 26, 50);
        let signals = stc.calculate(&self.candles.clone());
        let last_signal = signals.last();
        if last_signal.is_none() {
            return 0;
        }
        // 直接使用最后一个信号进行判断
        let (stc_value, red_signal, green_signal) = last_signal.unwrap();
        if *red_signal && stc_value > &75.0 {
            return -1;
        } 
        if *green_signal  && stc_value < &25.0 {
            return 1;
        }
        0
    }

    fn get_name(&self) -> String {
        "STC".to_string()
    }
    fn get_description(&self) -> String {
        "STC指标".to_string()
    }
}
