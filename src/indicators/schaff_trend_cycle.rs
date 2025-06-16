use env_logger::fmt::Color;
use crate::indicators::candle::Candle;
pub struct SchaffTrendCycle {
    length: usize,
    fast_length: usize,
    slow_length: usize,
    factor: f64,
    stc_values: Vec<f64>,
}

impl SchaffTrendCycle {
    pub fn new(length: usize, fast_length: usize, slow_length: usize) -> Self {
        SchaffTrendCycle {
            length,
            fast_length,
            slow_length,
            factor: 0.5, // 默认平滑因子
            stc_values: Vec::new(),
        }
    }

    pub fn calculate(&mut self, candles: &[Candle]) -> Vec<(f64, bool, bool)> {
        // 确保至少有足够的数据来计算指标
        let min_required = self.length.max(self.fast_length).max(self.slow_length);
        if candles.len() < min_required {
            // 如果数据不足，返回空结果
            return Vec::new();
        }

        // 提取收盘价
        let prices: Vec<f64> = candles.iter().map(|c| c.close).collect();

        // 计算MACD值
        let macd_values = self.calculate_macd(&prices);

        // 初始化存储结果的向量
        self.stc_values = Vec::with_capacity(candles.len());
        let mut signals = Vec::with_capacity(candles.len());

        // 第一阶段随机计算的变量
        let mut stoch1_values = Vec::with_capacity(candles.len());
        let mut stoch1_smooth = Vec::with_capacity(candles.len());

        // 第二阶段随机计算的变量
        let mut stoch2_values = Vec::with_capacity(candles.len());
        let mut stoch2_smooth = Vec::with_capacity(candles.len());

        // 为每个价格点计算STC值
        for i in 0..candles.len() {
            // 确保有足够的数据进行计算
            if i < self.length {
                self.stc_values.push(50.0); // 默认值
                signals.push((50.0, false, false));
                stoch1_values.push(50.0);
                stoch1_smooth.push(50.0);
                stoch2_values.push(50.0);
                stoch2_smooth.push(50.0);
                continue;
            }

            // 计算第一阶段随机值 - 修复这里的索引计算
            let start_idx = if i >= self.length { i - self.length + 1 } else { 0 };
            let lowest_macd = macd_values[start_idx..=i].iter().fold(f64::INFINITY, |a, &b| a.min(b));
            let highest_macd = macd_values[start_idx..=i].iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
            let macd_range = highest_macd - lowest_macd;

            let stoch1 = if macd_range > 0.0 {
                (macd_values[i] - lowest_macd) / macd_range * 100.0
            } else {
                if i > 0 { stoch1_values[i-1] } else { 50.0 }
            };
            stoch1_values.push(stoch1);

            // 应用第一阶段平滑
            let stoch1_ema = if i > 0 {
                stoch1_smooth[i-1] + self.factor * (stoch1 - stoch1_smooth[i-1])
            } else {
                stoch1
            };
            stoch1_smooth.push(stoch1_ema);

            // 计算第二阶段随机值 - 同样修复索引计算
            let lowest_stoch1 = stoch1_smooth[start_idx..=i].iter().fold(f64::INFINITY, |a, &b| a.min(b));
            let highest_stoch1 = stoch1_smooth[start_idx..=i].iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
            let stoch1_range = highest_stoch1 - lowest_stoch1;

            let stoch2 = if stoch1_range > 0.0 {
                (stoch1_ema - lowest_stoch1) / stoch1_range * 100.0
            } else {
                if i > 0 { stoch2_values[i-1] } else { 50.0 }
            };
            stoch2_values.push(stoch2);

            // 应用第二阶段平滑 (最终STC值)
            let stc = if i > 0 {
                stoch2_smooth[i-1] + self.factor * (stoch2 - stoch2_smooth[i-1])
            } else {
                stoch2
            };
            stoch2_smooth.push(stc);
            self.stc_values.push(stc);

            // 生成信号 - 确保有足够的历史数据
            let red_signal = i >= 3 &&
                self.stc_values[i-3] <= self.stc_values[i-2] &&
                self.stc_values[i-2] > self.stc_values[i-1] &&
                stc > 75.0;

            let green_signal = i >= 3 &&
                self.stc_values[i-3] >= self.stc_values[i-2] &&
                self.stc_values[i-2] < self.stc_values[i-1] &&
                stc < 25.0;

            signals.push((stc, red_signal, green_signal));
        }

        signals
    }

    // 获取STC值
    pub fn stc_values(&self) -> &[f64] {
        &self.stc_values
    }

    // 获取STC颜色
    pub fn get_colors(&self) -> Vec<Color> {
        let mut colors = Vec::with_capacity(self.stc_values.len());

        for i in 0..self.stc_values.len() {
            if i == 0 {
                colors.push(Color::Green); // 默认第一个颜色
            } else {
                if self.stc_values[i] > self.stc_values[i-1] {
                    colors.push(Color::Green);
                } else {
                    colors.push(Color::Red);
                }
            }
        }

        colors
    }

    // 辅助函数：计算MACD值
    fn calculate_macd(&self, prices: &[f64]) -> Vec<f64> {
        // 计算快速EMA
        let fast_ema = self.calculate_ema(prices, self.fast_length);

        // 计算慢速EMA
        let slow_ema = self.calculate_ema(prices, self.slow_length);

        // 计算MACD值 (快速EMA - 慢速EMA)
        fast_ema.iter().zip(slow_ema.iter())
            .map(|(fast, slow)| fast - slow)
            .collect()
    }

    // 辅助函数：计算EMA
    fn calculate_ema(&self, prices: &[f64], period: usize) -> Vec<f64> {
        let mut ema = Vec::with_capacity(prices.len());

        // 确保period不为0，避免除以零错误
        let period = if period == 0 { 1 } else { period };
        let multiplier = 2.0 / (period as f64 + 1.0);

        // 初始化EMA为前period个价格的平均值
        let mut sum = 0.0;
        let actual_period = period.min(prices.len());

        // 避免除以零
        if actual_period == 0 {
            return Vec::new();
        }

        for i in 0..actual_period {
            sum += prices[i];
        }

        let first_ema = sum / actual_period as f64;
        ema.push(first_ema);

        // 计算剩余的EMA
        for i in 1..prices.len() {
            let prev_ema = ema[i-1];
            let current_ema = (prices[i] - prev_ema) * multiplier + prev_ema;
            ema.push(current_ema);
        }

        ema
    }
}
