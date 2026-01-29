use crate::models::candle::Candle;
use crate::indicators::utils::round_precision;

/// HMA 指标计算器
pub struct HMA {
    pub values: Vec<f64>, // 存储计算出的 HMA 值
}

impl HMA {
    /// 创建新的 HMA 实例
    pub fn new() -> Self {
        HMA { values: Vec::new() }
    }

    /// 计算加权移动平均 (WMA)
    fn wma(prices: &[f64], period: usize) -> Vec<f64> {
        let mut result = Vec::new();
        if prices.len() < period {
            return result;
        }

        for i in period - 1..prices.len() {
            let mut sum = 0.0;
            let mut weight_sum = 0.0;

            for j in 0..period {
                let w = (period - j) as f64; // 直接计算权重
                sum += prices[i - j] * w;
                weight_sum += w;
            }

            result.push(round_precision(sum / weight_sum));
        }
        result
    }

    /// 计算 HMA 值
    pub fn calculate(&mut self, candles: &[Candle], period: usize) -> &Vec<f64> {
        // 清空之前的 HMA 值
        self.values.clear();

        // 输入验证
        if candles.len() < period || period < 2 {
            return &self.values;
        }

        // 提取收盘价
        let prices: Vec<f64> = candles.iter().map(|candle| candle.close).collect();

        // Step 1: 计算 WMA(n/2) 和 WMA(n)
        let half_period = (period as f64 / 2.0).floor() as usize;
        let wma_half = Self::wma(&prices, half_period);
        let wma_full = Self::wma(&prices, period);

        // Step 2: 计算 2 * WMA(n/2) - WMA(n)
        let mut wma_diff = Vec::new();
        for i in 0..wma_half.len().min(wma_full.len()) {
            wma_diff.push(round_precision(2.0 * wma_half[i] - wma_full[i]));
        }

        // Step 3: 对结果计算周期为 sqrt(n) 的 WMA
        let sqrt_period = (period as f64).sqrt().floor() as usize;
        let raw_values = Self::wma(&wma_diff, sqrt_period);
        // 对最终结果应用精度处理
        self.values = raw_values.into_iter().map(round_precision).collect();

        // 返回 HMA 值
        &self.values
    }
}