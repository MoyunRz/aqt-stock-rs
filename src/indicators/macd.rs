use crate::indicators::candle::Candle;

/// MACD 指标结构体
pub struct MACD {
    // 配置参数
    fast_length: usize,  // 快线周期
    slow_length: usize,  // 慢线周期
    signal_length: usize, // 信号线周期

    // 内部状态
    fast_ema: ExponentialMovingAverage,
    slow_ema: ExponentialMovingAverage,
    signal_ema: ExponentialMovingAverage,

    // 计算结果
    macd_line: Vec<f64>,
    signal_line: Vec<f64>,
    histogram: Vec<f64>,
}

/// 指数移动平均线实现
pub struct ExponentialMovingAverage {
    length: usize,
    multiplier: f64,
    last_ema: Option<f64>,
}

impl ExponentialMovingAverage {
    pub fn new(length: usize) -> Self {
        ExponentialMovingAverage {
            length,
            multiplier: 2.0 / (length as f64 + 1.0),
            last_ema: None,
        }
    }

    pub fn update(&mut self, value: f64) -> f64 {
        match self.last_ema {
            Some(last) => {
                let ema = (value - last) * self.multiplier + last;
                self.last_ema = Some(ema);
                ema
            },
            None => {
                // 首次计算时，直接使用当前值作为 EMA
                self.last_ema = Some(value);
                value
            }
        }
    }

    pub fn reset(&mut self) {
        self.last_ema = None;
    }
}

impl MACD {
    /// 创建新的 MACD 指标
    ///
    /// # 参数
    /// * `fast_length` - 快线 EMA 周期，通常为 12
    /// * `slow_length` - 慢线 EMA 周期，通常为 26
    /// * `signal_length` - 信号线 EMA 周期，通常为 9
    pub fn new(fast_length: usize, slow_length: usize, signal_length: usize) -> Self {
        MACD {
            fast_length,
            slow_length,
            signal_length,
            fast_ema: ExponentialMovingAverage::new(fast_length),
            slow_ema: ExponentialMovingAverage::new(slow_length),
            signal_ema: ExponentialMovingAverage::new(signal_length),
            macd_line: Vec::new(),
            signal_line: Vec::new(),
            histogram: Vec::new(),
        }
    }

    /// 使用默认参数创建 MACD 指标 (12, 26, 9)
    pub fn default() -> Self {
        Self::new(12, 26, 9)
    }

    /// 更新 MACD 指标
    ///
    /// # 参数
    /// * `price` - 当前价格
    ///
    /// # 返回值
    /// 返回包含 MACD 线、信号线和柱状图的元组
    pub fn update(&mut self, price: f64) -> (f64, f64, f64) {
        // 更新快速和慢速 EMA
        let fast_ema = self.fast_ema.update(price);
        let slow_ema = self.slow_ema.update(price);

        // 计算 MACD 线
        let macd_line = fast_ema - slow_ema;
        self.macd_line.push(macd_line);

        // 更新信号线 (MACD 线的 EMA)
        let signal_line = self.signal_ema.update(macd_line);
        self.signal_line.push(signal_line);

        // 计算柱状图
        let histogram = macd_line - signal_line;
        self.histogram.push(histogram);

        (macd_line, signal_line, histogram)
    }

    /// 计算一组价格的 MACD
    pub fn calculate(&mut self, candles: Vec<Candle>) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        // 重置状态
        self.reset();
     
        let prices: Vec<f64> = candles.iter().map(|c| c.close).collect();
        // 处理每个价格
        for price in prices {
            self.update(price);
        }
        // 返回计算结果的克隆
        (
            self.macd_line.clone(),
            self.signal_line.clone(),
            self.histogram.clone()
        )
    }

    /// 重置 MACD 计算器状态
    pub fn reset(&mut self) {
        self.fast_ema.reset();
        self.slow_ema.reset();
        self.signal_ema.reset();
        self.macd_line.clear();
        self.signal_line.clear();
        self.histogram.clear();
    }

    /// 获取最新的 MACD 值
    pub fn latest(&self) -> Option<(f64, f64, f64)> {
        if self.macd_line.is_empty() || self.signal_line.is_empty() || self.histogram.is_empty() {
            None
        } else {
            Some((
                *self.macd_line.last().unwrap(),
                *self.signal_line.last().unwrap(),
                *self.histogram.last().unwrap()
            ))
        }
    }

    /// 获取 MACD 线
    pub fn macd_line(&self) -> &[f64] {
        &self.macd_line
    }

    /// 获取信号线
    pub fn signal_line(&self) -> &[f64] {
        &self.signal_line
    }

    /// 获取柱状图
    pub fn histogram(&self) -> &[f64] {
        &self.histogram
    }

    /// 检查是否有买入信号 (柱状图从负转正)
    pub fn is_buy_signal(&self) -> bool {
        if self.histogram.len() < 2 {
            return false;
        }

        let last = *self.histogram.last().unwrap();
        let previous = self.histogram[self.histogram.len() - 2];

        previous < 0.0 && last > 0.0
    }

    /// 检查是否有卖出信号 (柱状图从正转负)
    pub fn is_sell_signal(&self) -> bool {
        if self.histogram.len() < 2 {
            return false;
        }

        let last = *self.histogram.last().unwrap();
        let previous = self.histogram[self.histogram.len() - 2];

        previous > 0.0 && last < 0.0
    }

}

// 为 MACD 实现 Debug trait 以便于打印调试信息
impl std::fmt::Debug for MACD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MACD({}, {}, {})", self.fast_length, self.slow_length, self.signal_length)
    }
}