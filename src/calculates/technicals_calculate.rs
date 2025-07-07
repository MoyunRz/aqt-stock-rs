use crate::calculates::base_calculate::BaseCalculate;
use crate::indicators::tradingview_technicals::TradingTechnicals;

pub struct TechnicalsCalculate {
    pub technicals: TradingTechnicals
}

impl BaseCalculate for TechnicalsCalculate {
    fn calculate(&self) -> i64 {
        0
    }

    fn get_name(&self) -> String {
        "Tradingview Technicals".to_string()
    }
    fn get_description(&self) -> String {
        "Tradingview Technicals 指标".to_string()
    }
}
