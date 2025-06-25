use longport::quote::MarketTemperature;
use crate::calculates::base_calculate::BaseCalculate;

pub struct MarketCalculate {
    pub market: MarketTemperature
}

impl BaseCalculate for MarketCalculate {
    fn calculate(&self) -> i64 {

        if self.market.sentiment > 70 {
            return -2;
        }
        if self.market.sentiment < 24 {
            return 2;
        }
        if self.market.temperature < 30 {
            return 2;
        }
        if self.market.temperature > 55 {
            return -2;
        }
        if self.market.valuation > 50 {
            return -2;
        }
        if self.market.valuation < 30 {
            return 2;
        }
        0
    }

    fn get_name(&self) -> String {
        "Market Temperature".to_string()
    }
    fn get_description(&self) -> String {
        "Market Temperature指标".to_string()
    }
}
