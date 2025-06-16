use crate::calculates::base_calculate::BaseCalculate;
use crate::indicators::candle::Candle;
use crate::indicators::kdj::KDJ;

pub struct KdjCalculate {
    pub(crate) candles: Vec<Candle>
}

impl BaseCalculate for KdjCalculate {
    fn calculate(&self) -> i64 {
        let mut kdj = KDJ::default();
        kdj.calculate(&self.candles);
        // 检查交易信号
        if kdj.is_golden_cross() {
            return 1;
        }
        if kdj.is_oversold(20.0) {
            return 1;
        }
        if kdj.is_death_cross() {
            return -1;
        }
        if kdj.is_overbought(80.0) {
            return -1;
        }
        0
    }

    fn get_name(&self) -> String {
        "KDJ".to_string()
    }
    fn get_description(&self) -> String {
        "KDJ指标".to_string()
    }
}
