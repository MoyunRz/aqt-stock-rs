use crate::calculates::base_calculate::BaseCalculate;
use crate::indicators::tradingview_technicals::TradingTechnicals;

pub struct TechnicalsCalculate {
    pub technicals: TradingTechnicals
}

impl BaseCalculate for TechnicalsCalculate {
    fn calculate(&self) -> i64 {
        let (summary_signal, ma_signal, osc_signal) = self.technicals.calculate();
        if summary_signal >= 1.0 || ma_signal >= 1.0 || osc_signal >= 1.0 {
            3
        }
        else if summary_signal <= -1.0 || ma_signal <= -1.0 || osc_signal <= -1.0 {
            -3
        }
        else {
            0
        }
    }

    fn get_name(&self) -> String {
        "Tradingview Technicals".to_string()
    }
    fn get_description(&self) -> String {
        "Tradingview Technicals 指标".to_string()
    }
}
