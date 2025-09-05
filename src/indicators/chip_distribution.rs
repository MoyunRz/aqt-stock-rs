use std::collections::HashMap;
use crate::models::candle::Candle;

pub struct ChipDistribution {}

// 筹码分布的结构体
#[derive(Clone)]
pub struct ChipLevel {
    pub price: f64,
    pub volume: f64, // 累积成交量（筹码量）
}

impl ChipDistribution {
    pub fn new() -> Self {
        ChipDistribution {}
    }

    pub fn default() -> Self {
        Self::new()
    }

    pub fn calculate(&mut self, candles: &[Candle]) -> Vec<ChipLevel> {
        let (precision, decay_factor) = ChipDistribution::calculate_parameters(candles);

        let mut chip_map = HashMap::new();

        for (i, candle) in candles.iter().enumerate() {
            // 四舍五入到指定精度
            let rounded_price = (candle.close / precision).round() * precision;
            let price_key = (rounded_price * 100.0) as i64;

            // 时间衰减：近期K线权重更高
            let weight = if decay_factor > 0.0 {
                (1.0 - decay_factor).powi(i as i32)
            } else {
                1.0
            };

            // 累加成交量
            let entry = chip_map.entry(price_key).or_insert(0.0);
            *entry += candle.volume * weight;
        }

        // 转换为ChipLevel向量并排序
        let mut levels: Vec<ChipLevel> = chip_map
            .into_iter()
            .map(|(price, volume)| ChipLevel {
                price: price as f64 / 100.0,
                volume,
            })
            .collect();

        // 按筹码量降序排序
        levels.sort_by(|a, b| b.volume.partial_cmp(&a.volume).unwrap_or(std::cmp::Ordering::Equal));
        println!("Chip Distribution:"); 
        for level in levels.clone() {
            println!("Price: {:.2}, Volume: {:.2}", level.price, level.volume);
        }
        
        levels
    }


    // 计算平均涨跌幅度均值
    pub fn calculate_avg_price_change(candles: &[Candle]) -> f64 {
        if candles.is_empty() {
            return 0.0;
        }

        let mut total_change = 0.0;
        let mut valid_count = 0;

        for candle in candles {
            if candle.open > 0.0 && (candle.close - candle.open) != 0.0 { // 避免除以0
                let change = (candle.close - candle.open).abs() / candle.open;
                total_change += change;
                valid_count += 1;
            }
        }

        if valid_count > 0 {
            total_change / valid_count as f64
        } else {
            0.0
        }
    }

    // 动态计算precision和decay_factor
    fn calculate_parameters(candles: &[Candle]) -> (f64, f64) {
        let price_range = candles.iter().map(|c| c.close).fold(f64::MIN, f64::max) -
            candles.iter().map(|c| c.close).fold(f64::MAX, f64::min);
        let precision = price_range / 100.0; // 根据价格范围动态设置精度
        let decay_factor = if candles.len() > 100 { 0.01 } else { 0.005 }; // 根据数据长度调整衰减
        (precision.max(0.01), decay_factor)
    }
}

