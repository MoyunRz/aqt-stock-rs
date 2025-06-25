use log::info;
use crate::calculates::base_calculate::BaseCalculate;
use crate::indicators::candle::Candle;
use crate::indicators::cyc::CYC;

pub struct CycCalculate {
    pub candles: Vec<Candle>
}

impl BaseCalculate for CycCalculate {
    fn calculate(&self) -> i64 {
        let mut cyc = CYC::default();
        let (short_cyc, mid_cyc, long_cyc) = cyc.calculate(&self.candles.clone());
        // 获取当前价格
        let current_price = self.candles.last().unwrap().close;
        let threshold = 5.0; // 超买/超卖阈值 5%
        // 生成信号
        let signal = cyc.generate_signal(current_price, threshold);
        info!("短期 CYC: {:.2}, 中期 CYC: {:.2}, 长期 CYC: {:.2}", short_cyc.last().unwrap(), mid_cyc.last().unwrap(), long_cyc.last().unwrap());
        info!("当前价格: {:.2}, 信号: {:?}", current_price, signal);
        signal
    }

    fn get_name(&self) -> String {
        "KDJ".to_string()
    }
    fn get_description(&self) -> String {
        "KDJ指标".to_string()
    }
}
