use std::vec::Vec;
use crate::indicators::candle::Candle;

/// CYC 成本均线指标结构体
pub struct CYC {
    // 配置参数
    short_period: usize,  // 短期 CYC 周期（如 5）
    mid_period: usize,    // 中期 CYC 周期（如 13）
    long_period: usize,   // 长期 CYC 周期（如 34）

    // 内部状态
    price_volume_sums: Vec<f64>, // 价格*成交量之和
    volume_sums: Vec<f64>,       // 成交量之和

    // 计算结果
    short_cyc: Vec<f64>,  // 短期 CYC 值
    mid_cyc: Vec<f64>,    // 中期 CYC 值
    long_cyc: Vec<f64>,   // 长期 CYC 值
}

impl CYC {
    /// 创建新的 CYC 指标
    ///
    /// # 参数
    /// * `short_period` - 短期 CYC 计算周期（如 5）
    /// * `mid_period` - 中期 CYC 计算周期（如 13）
    /// * `long_period` - 长期 CYC 计算周期（如 34）
    pub fn new(short_period: usize, mid_period: usize, long_period: usize) -> Self {
        CYC {
            short_period,
            mid_period,
            long_period,
            price_volume_sums: Vec::new(),
            volume_sums: Vec::new(),
            short_cyc: Vec::new(),
            mid_cyc: Vec::new(),
            long_cyc: Vec::new(),
        }
    }

    /// 使用默认参数创建 CYC 指标 (5, 13, 34)
    pub fn default() -> Self {
        Self::new(5, 13, 34)
    }

    /// 计算一组K线的 CYC 指标
    pub fn calculate(&mut self, candles: &[Candle]) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        // 重置状态
        self.reset();

        if candles.len() < self.long_period {
            return (vec![], vec![], vec![]);
        }

        // 计算每个周期的 CYC
        for i in 0..candles.len() {
            self.update_cyc(candles, i);
        }

        // 返回计算结果的克隆
        (
            self.short_cyc.clone(),
            self.mid_cyc.clone(),
            self.long_cyc.clone(),
        )
    }

    /// 更新指定位置的 CYC 值
    fn update_cyc(&mut self, candles: &[Candle], index: usize) {
        // 计算价格*成交量和成交量
        let price = candles[index].close; // 可用 (high + low + close) / 3 替代
        let volume = candles[index].volume;
        self.price_volume_sums.push(price * volume);
        self.volume_sums.push(volume);

        // 计算短期 CYC
        if index >= self.short_period - 1 {
            let start = index + 1 - self.short_period;
            let pv_sum: f64 = self.price_volume_sums[start..=index].iter().sum();
            let v_sum: f64 = self.volume_sums[start..=index].iter().sum();
            let short_cyc = if v_sum.abs() < f64::EPSILON { 0.0 } else { pv_sum / v_sum };
            self.short_cyc.push(short_cyc);
        } else {
            self.short_cyc.push(0.0);
        }

        // 计算中期 CYC
        if index >= self.mid_period - 1 {
            let start = index + 1 - self.mid_period;
            let pv_sum: f64 = self.price_volume_sums[start..=index].iter().sum();
            let v_sum: f64 = self.volume_sums[start..=index].iter().sum();
            let mid_cyc = if v_sum.abs() < f64::EPSILON { 0.0 } else { pv_sum / v_sum };
            self.mid_cyc.push(mid_cyc);
        } else {
            self.mid_cyc.push(0.0);
        }

        // 计算长期 CYC
        if index >= self.long_period - 1 {
            let start = index + 1 - self.long_period;
            let pv_sum: f64 = self.price_volume_sums[start..=index].iter().sum();
            let v_sum: f64 = self.volume_sums[start..=index].iter().sum();
            let long_cyc = if v_sum.abs() < f64::EPSILON { 0.0 } else { pv_sum / v_sum };
            self.long_cyc.push(long_cyc);
        } else {
            self.long_cyc.push(0.0);
        }
    }

    /// 重置 CYC 计算器状态
    pub fn reset(&mut self) {
        self.price_volume_sums.clear();
        self.volume_sums.clear();
        self.short_cyc.clear();
        self.mid_cyc.clear();
        self.long_cyc.clear();
    }

    /// 获取最新的 CYC 值
    pub fn latest(&self) -> Option<(f64, f64, f64)> {
        if self.short_cyc.is_empty() || self.mid_cyc.is_empty() || self.long_cyc.is_empty() {
            None
        } else {
            Some((
                *self.short_cyc.last().unwrap(),
                *self.mid_cyc.last().unwrap(),
                *self.long_cyc.last().unwrap(),
            ))
        }
    }

    /// 获取短期 CYC 值序列
    pub fn short_cyc(&self) -> &[f64] {
        &self.short_cyc
    }

    /// 获取中期 CYC 值序列
    pub fn mid_cyc(&self) -> &[f64] {
        &self.mid_cyc
    }

    /// 获取长期 CYC 值序列
    pub fn long_cyc(&self) -> &[f64] {
        &self.long_cyc
    }

    /// 检查是否有超买信号（价格显著高于长期 CYC）
    pub fn is_overbought(&self, price: f64, threshold: f64) -> bool {
        if let Some((_, _, long_cyc)) = self.latest() {
            price > long_cyc * (1.0 + threshold / 100.0) // 价格高于长期 CYC 的 threshold%
        } else {
            false
        }
    }

    /// 检查是否有超卖信号（价格显著低于长期 CYC）
    pub fn is_oversold(&self, price: f64, threshold: f64) -> bool {
        if let Some((_, _, long_cyc)) = self.latest() {
            price < long_cyc * (1.0 - threshold / 100.0) // 价格低于长期 CYC 的 threshold%
        } else {
            false
        }
    }

    /// 检查是否有金叉信号（短期 CYC 上穿长期 CYC）
    pub fn is_golden_cross(&self) -> bool {
        if self.short_cyc.len() < 2 || self.long_cyc.len() < 2 {
            return false;
        }

        let last_short = *self.short_cyc.last().unwrap();
        let last_long = *self.long_cyc.last().unwrap();
        let prev_short = self.short_cyc[self.short_cyc.len() - 2];
        let prev_long = self.long_cyc[self.long_cyc.len() - 2];

        prev_short <= prev_long && last_short > last_long
    }

    /// 检查是否有死叉信号（短期 CYC 下穿长期 CYC）
    pub fn is_death_cross(&self) -> bool {
        if self.short_cyc.len() < 2 || self.long_cyc.len() < 2 {
            return false;
        }

        let last_short = *self.short_cyc.last().unwrap();
        let last_long = *self.long_cyc.last().unwrap();
        let prev_short = self.short_cyc[self.short_cyc.len() - 2];
        let prev_long = self.long_cyc[self.long_cyc.len() - 2];

        prev_short >= prev_long && last_short < last_long
    }
    /// 生成交易信号
    pub fn generate_signal(&self, price: f64, threshold: f64) -> i64 {
        if self.is_golden_cross() || (self.is_oversold(price, threshold) && self.short_cyc.last().unwrap() > &self.short_cyc[self.short_cyc.len() - 2]) {
            1
        } else if self.is_death_cross() || (self.is_overbought(price, threshold) && self.short_cyc.last().unwrap() < &self.short_cyc[self.short_cyc.len() - 2]) {
            -1
        } else {
            0
        }
    }
}

// 为 CYC 实现 Debug trait 以便于打印调试信息
impl std::fmt::Debug for CYC {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CYC({}, {}, {})",
            self.short_period, self.mid_period, self.long_period
        )
    }
}