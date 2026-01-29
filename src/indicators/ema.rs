use crate::models::candle::Candle;
use crate::indicators::utils::round_precision;

pub struct EMA {}

impl EMA {
    pub fn new() -> Self {
        EMA {}
    }

    pub fn default() -> Self {
        Self::new()
    }

    /// 计算EMA（指数移动平均线）
    ///
    /// # 参数
    /// - prices: 价格序列
    /// - period: EMA周期
    ///
    /// # 返回
    /// 包含EMA值的Vec<f64>，长度与输入prices相同，前period-1个值为None的等效表示
    pub fn calculate(&mut self, candles: &[Candle], period: usize) -> Vec<Option<f64>> {
        if candles.is_empty() || period == 0 || period > candles.len() {
            return vec![None; candles.len()];
        }

        let mut ema = vec![None; candles.len()];
        let multiplier = 2.0 / (period as f64 + 1.0);

        // 第一个EMA值使用简单移动平均（SMA）作为起点
        let mut sma = 0.0;
        for i in 0..period {
            sma += candles[i].close;
        }
        sma /= period as f64;
        ema[period - 1] = Some(sma);

        // 计算后续EMA值并应用精度处理
        for i in period..candles.len() {
            let prev_ema = ema[i - 1].unwrap();
            let current_ema = (candles[i].close * multiplier) + (prev_ema * (1.0 - multiplier));
            ema[i] = Some(round_precision(current_ema));
        }
        ema
    }
}
