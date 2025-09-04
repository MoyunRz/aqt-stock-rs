use log::{debug, info};
use crate::calculates::base_calculate::BaseCalculate;
use crate::indicators::chip_distribution::ChipDistribution;
use crate::models::candle::Candle;
use crate::indicators::cyc::CYC;
use crate::indicators::fibonacci::Fibonacci;

pub struct ChipCalculate {
    pub candles: Vec<Candle>,
}

impl BaseCalculate for ChipCalculate {
    
    fn calculate(&self) -> i64 {

        let mut distribution  = ChipDistribution::new();
        let mark_px = self.candles.clone().last().unwrap().close;
        let leves = distribution.calculate(&self.candles.clone());
        // 当前价格在筹码最多的位置还是最少的位置？
        if leves.len() > 0 {
            let price = leves.get(0).unwrap().price;
            if price > mark_px {
                // 卖出
                return -1;
            }
            if price < mark_px {
                // 卖出
                return 1;
            }
        }
        0
    }

    fn get_name(&self) -> String {
        "CYC".to_string()
    }
    fn get_description(&self) -> String {
        "CYC 指标".to_string()
    }
}
