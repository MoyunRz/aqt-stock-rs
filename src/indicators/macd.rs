use crate::indicators::candle::Candle;
pub struct MACD {
    fast_length: usize,
    slow_length: usize,
    signal_length: usize,

    // 存储计算结果
    macd_line: Vec<f64>,
    signal_line: Vec<f64>,
    histogram: Vec<f64>,

    // 配置选项
    show_macd_signal: bool,
    show_dots: bool,
    show_histogram: bool,
    macd_color_change: bool,
    hist_color_change: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MacdColor {
    Lime,
    Red,
    Yellow,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HistColor {
    Aqua,    // histA_IsUp
    Blue,    // histA_IsDown
    Red,     // histB_IsDown
    Maroon,  // histB_IsUp
    Yellow,  // 默认
    Gray,    // 不变色
}

impl MACD {
    pub fn new(fast_length: usize, slow_length: usize, signal_length: usize) -> Self {
        MACD {
            fast_length,
            slow_length,
            signal_length,
            macd_line: Vec::new(),
            signal_line: Vec::new(),
            histogram: Vec::new(),
            show_macd_signal: true,
            show_dots: true,
            show_histogram: true,
            macd_color_change: true,
            hist_color_change: true,
        }
    }

    // 设置显示选项
    pub fn set_display_options(&mut self, show_macd_signal: bool, show_dots: bool,
                               show_histogram: bool, macd_color_change: bool,
                               hist_color_change: bool) {
        self.show_macd_signal = show_macd_signal;
        self.show_dots = show_dots;
        self.show_histogram = show_histogram;
        self.macd_color_change = macd_color_change;
        self.hist_color_change = hist_color_change;
    }

    // 计算MACD指标
    pub fn calculate(&mut self, candles: &[Candle]) -> (Vec<bool>, Vec<bool>) {
        if candles.is_empty() {
            return (Vec::new(), Vec::new());
        }

        // 提取收盘价
        let prices: Vec<f64> = candles.iter().map(|c| c.close).collect();

        // 计算EMA
        let fast_ma = Self::calculate_ema(&prices, self.fast_length);
        let slow_ma = Self::calculate_ema(&prices, self.slow_length);

        // 计算MACD线
        self.macd_line = fast_ma.iter().zip(slow_ma.iter())
            .map(|(fast, slow)| fast - slow)
            .collect();

        // 计算信号线 (使用SMA)
        self.signal_line = Self::calculate_sma(&self.macd_line, self.signal_length);

        // 计算直方图
        self.histogram = self.macd_line.iter().zip(self.signal_line.iter())
            .map(|(macd, signal)| macd - signal)
            .collect();

        // 生成交叉信号
        let mut buy_signals = vec![false; candles.len()];
        let mut sell_signals = vec![false; candles.len()];

        // 从第二个点开始检查交叉
        for i in 1..self.macd_line.len() {
            // MACD线从下方穿过信号线 - 买入信号
            if self.macd_line[i-1] < self.signal_line[i-1] &&
                self.macd_line[i] >= self.signal_line[i] {
                buy_signals[i] = true;
            }

            // MACD线从上方穿过信号线 - 卖出信号
            if self.macd_line[i-1] > self.signal_line[i-1] &&
                self.macd_line[i] <= self.signal_line[i] {
                sell_signals[i] = true;
            }
        }

        (buy_signals, sell_signals)
    }

    // 获取MACD线
    pub fn macd_line(&self) -> &[f64] {
        &self.macd_line
    }

    // 获取信号线
    pub fn signal_line(&self) -> &[f64] {
        &self.signal_line
    }

    // 获取直方图
    pub fn histogram(&self) -> &[f64] {
        &self.histogram
    }

    // 获取MACD线颜色
    pub fn macd_colors(&self) -> Vec<MacdColor> {
        if !self.macd_color_change {
            return vec![MacdColor::Red; self.macd_line.len()];
        }

        self.macd_line.iter().zip(self.signal_line.iter())
            .map(|(macd, signal)| {
                if macd >= signal {
                    MacdColor::Lime
                } else {
                    MacdColor::Red
                }
            })
            .collect()
    }

    // 获取直方图颜色
    pub fn histogram_colors(&self) -> Vec<HistColor> {
        if !self.hist_color_change {
            return vec![HistColor::Gray; self.histogram.len()];
        }

        let mut colors = Vec::with_capacity(self.histogram.len());

        for i in 0..self.histogram.len() {
            let hist = self.histogram[i];
            let prev_hist = if i > 0 { self.histogram[i-1] } else { 0.0 };

            let color = if hist > 0.0 {
                if hist > prev_hist {
                    HistColor::Aqua  // histA_IsUp
                } else {
                    HistColor::Blue  // histA_IsDown
                }
            } else {
                if hist < prev_hist {
                    HistColor::Red   // histB_IsDown
                } else {
                    HistColor::Maroon // histB_IsUp
                }
            };

            colors.push(color);
        }

        colors
    }

    // 获取交叉点
    pub fn cross_points(&self) -> Vec<bool> {
        if !self.show_dots {
            return vec![false; self.macd_line.len()];
        }

        let mut crosses = vec![false; self.macd_line.len()];

        for i in 1..self.macd_line.len() {
            let prev_macd = self.macd_line[i-1];
            let prev_signal = self.signal_line[i-1];
            let curr_macd = self.macd_line[i];
            let curr_signal = self.signal_line[i];

            // 检查是否有交叉
            if (prev_macd > prev_signal && curr_macd <= curr_signal) ||
                (prev_macd < prev_signal && curr_macd >= curr_signal) {
                crosses[i] = true;
            }
        }

        crosses
    }

    // 辅助函数：计算EMA
    fn calculate_ema(prices: &[f64], period: usize) -> Vec<f64> {
        if prices.is_empty() || period == 0 {
            return Vec::new();
        }

        let mut ema = Vec::with_capacity(prices.len());
        let multiplier = 2.0 / (period as f64 + 1.0);

        // 初始化EMA为前period个价格的平均值
        let mut sum = 0.0;
        for i in 0..period.min(prices.len()) {
            sum += prices[i];
        }

        let first_ema = sum / period as f64;
        ema.push(first_ema);

        // 计算剩余的EMA
        for i in 1..prices.len() {
            let prev_ema = ema[i-1];
            let current_ema = (prices[i] - prev_ema) * multiplier + prev_ema;
            ema.push(current_ema);
        }

        ema
    }

    // 辅助函数：计算SMA
    fn calculate_sma(values: &[f64], period: usize) -> Vec<f64> {
        if values.is_empty() || period == 0 {
            return Vec::new();
        }

        let mut sma = Vec::with_capacity(values.len());

        // 填充前period-1个位置为0
        for _ in 0..period-1.min(values.len()) {
            sma.push(0.0);
        }

        // 计算移动平均
        for i in period-1..values.len() {
            let mut sum = 0.0;
            for j in 0..period {
                sum += values[i - j];
            }
            sma.push(sum / period as f64);
        }

        sma
    }
}
