use std::vec;
use crate::indicators::candle::Candle;

/// KDJ 指标结构体
pub struct KDJ {
    // 配置参数
    k_period: usize,     // K值计算周期
    d_period: usize,     // D值计算周期
    j_period: usize,     // J值计算周期

    // 内部状态
    highest_high: Vec<f64>,  // 周期内最高价
    lowest_low: Vec<f64>,    // 周期内最低价
    rsv_values: Vec<f64>,    // RSV值

    // 计算结果
    k_values: Vec<f64>,      // K值
    d_values: Vec<f64>,      // D值
    j_values: Vec<f64>,      // J值
}

impl KDJ {
    /// 创建新的 KDJ 指标
    ///
    /// # 参数
    /// * `k_period` - RSV计算周期，通常为9
    /// * `d_period` - D值平滑系数，通常为3
    /// * `j_period` - J值计算系数，通常为3
    pub fn new(k_period: usize, d_period: usize, j_period: usize) -> Self {
        KDJ {
            k_period,
            d_period,
            j_period,
            highest_high: Vec::new(),
            lowest_low: Vec::new(),
            rsv_values: Vec::new(),
            k_values: Vec::new(),
            d_values: Vec::new(),
            j_values: Vec::new(),
        }
    }

    /// 使用默认参数创建 KDJ 指标 (9, 3, 3)
    pub fn default() -> Self {
        Self::new(9, 3, 3)
    }

    /// 计算一组K线的 KDJ 指标
    pub fn calculate(&mut self, candles: &[Candle]) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        // 重置状态
        self.reset();

        if candles.len() < self.k_period {
            return (vec![], vec![], vec![]);
        }

        // 计算每个周期的KDJ
        for i in 0..candles.len() {
            self.update_kdj(candles, i);
        }

        // 返回计算结果的克隆
        (
            self.k_values.clone(),
            self.d_values.clone(),
            self.j_values.clone()
        )
    }

    /// 更新指定位置的KDJ值
    fn update_kdj(&mut self, candles: &[Candle], index: usize) {
        // 计算RSV值
        let rsv = self.calculate_rsv(candles, index);
        self.rsv_values.push(rsv);

        // 计算K值 (第一个K值使用50作为初始值，后续使用SMA平滑)
        let k = if self.k_values.is_empty() {
            (rsv + 2.0 * 50.0) / 3.0
        } else {
            (rsv + (self.d_period as f64 - 1.0) * self.k_values.last().unwrap()) / self.d_period as f64
        };
        self.k_values.push(k);

        // 计算D值 (第一个D值使用50作为初始值，后续使用SMA平滑)
        let d = if self.d_values.is_empty() {
            (k + 2.0 * 50.0) / 3.0
        } else {
            (k + (self.d_period as f64 - 1.0) * self.d_values.last().unwrap()) / self.d_period as f64
        };
        self.d_values.push(d);

        // 计算J值
        let j = 3.0 * k - 2.0 * d;
        self.j_values.push(j);
    }

    /// 计算RSV (Raw Stochastic Value)
    fn calculate_rsv(&mut self, candles: &[Candle], index: usize) -> f64 {
        if index < self.k_period - 1 {
            return 50.0; // 数据不足时返回中间值
        }

        // 获取周期内的最高价和最低价
        let start = index + 1 - self.k_period;
        let period_candles = &candles[start..=index];

        let highest = period_candles.iter().map(|c| c.high).fold(f64::NEG_INFINITY, f64::max);
        let lowest = period_candles.iter().map(|c| c.low).fold(f64::INFINITY, f64::min);

        self.highest_high.push(highest);
        self.lowest_low.push(lowest);

        // 计算RSV
        if (highest - lowest).abs() < f64::EPSILON {
            50.0 // 避免除以零
        } else {
            100.0 * (candles[index].close - lowest) / (highest - lowest)
        }
    }

    /// 重置 KDJ 计算器状态
    pub fn reset(&mut self) {
        self.highest_high.clear();
        self.lowest_low.clear();
        self.rsv_values.clear();
        self.k_values.clear();
        self.d_values.clear();
        self.j_values.clear();
    }

    /// 获取最新的 KDJ 值
    pub fn latest(&self) -> Option<(f64, f64, f64)> {
        if self.k_values.is_empty() || self.d_values.is_empty() || self.j_values.is_empty() {
            None
        } else {
            Some((
                *self.k_values.last().unwrap(),
                *self.d_values.last().unwrap(),
                *self.j_values.last().unwrap()
            ))
        }
    }

    /// 获取 K 值序列
    pub fn k_values(&self) -> &[f64] {
        &self.k_values
    }

    /// 获取 D 值序列
    pub fn d_values(&self) -> &[f64] {
        &self.d_values
    }

    /// 获取 J 值序列
    pub fn j_values(&self) -> &[f64] {
        &self.j_values
    }

    /// 检查是否有超买信号 (K值和D值都大于80)
    pub fn is_overbought(&self, threshold: f64) -> bool {
        if let Some((k, d, _)) = self.latest() {
            k > threshold && d > threshold
        } else {
            false
        }
    }

    /// 检查是否有超卖信号 (K值和D值都小于20)
    pub fn is_oversold(&self, threshold: f64) -> bool {
        if let Some((k, d, _)) = self.latest() {
            k < threshold && d < threshold
        } else {
            false
        }
    }

    /// 检查是否有金叉信号 (K线从下方穿过D线)
    pub fn is_golden_cross(&self) -> bool {
        if self.k_values.len() < 2 || self.d_values.len() < 2 {
            return false;
        }

        let last_k = *self.k_values.last().unwrap();
        let last_d = *self.d_values.last().unwrap();
        let prev_k = self.k_values[self.k_values.len() - 2];
        let prev_d = self.d_values[self.d_values.len() - 2];

        prev_k < prev_d && last_k > last_d
    }

    /// 检查是否有死叉信号 (K线从上方穿过D线)
    pub fn is_death_cross(&self) -> bool {
        if self.k_values.len() < 2 || self.d_values.len() < 2 {
            return false;
        }

        let last_k = *self.k_values.last().unwrap();
        let last_d = *self.d_values.last().unwrap();
        let prev_k = self.k_values[self.k_values.len() - 2];
        let prev_d = self.d_values[self.d_values.len() - 2];

        prev_k > prev_d && last_k < last_d
    }
}

// 为 KDJ 实现 Debug trait 以便于打印调试信息
impl std::fmt::Debug for KDJ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "KDJ({}, {}, {})", self.k_period, self.d_period, self.j_period)
    }
}
