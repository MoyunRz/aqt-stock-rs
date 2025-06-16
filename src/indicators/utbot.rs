use crate::indicators::candle::Candle;

/// UT Bot 指标实现
///
/// 这是一个基于 ATR 的跟踪止损系统，用于生成交易信号
/// 原始指标来源于 TradingView 的 Pine Script
pub struct UTBot {
    // 配置参数
    key_value: f64,      // 灵敏度系数
    atr_period: usize,   // ATR 计算周期
    use_heikin_ashi: bool, // 是否使用平均K线

    // 内部状态
    atr_values: Vec<f64>,              // ATR 值
    trailing_stop: Vec<f64>,           // 跟踪止损线
    position: Vec<i32>,                // 仓位状态: 1=多头, -1=空头, 0=无仓位
    source_values: Vec<f64>,           // 价格源数据
    ema_values: Vec<f64>,              // EMA 值

    // 信号
    buy_signals: Vec<bool>,            // 买入信号
    sell_signals: Vec<bool>,           // 卖出信号
}

impl UTBot {
    /// 创建新的 UT Bot 指标
    ///
    /// # 参数
    /// * `key_value` - 灵敏度系数，默认为1.0
    /// * `atr_period` - ATR计算周期，默认为10
    /// * `use_heikin_ashi` - 是否使用平均K线，默认为false
    pub fn new(key_value: f64, atr_period: usize, use_heikin_ashi: bool) -> Self {
        UTBot {
            key_value,
            atr_period,
            use_heikin_ashi,
            atr_values: Vec::new(),
            trailing_stop: Vec::new(),
            position: Vec::new(),
            source_values: Vec::new(),
            ema_values: Vec::new(),
            buy_signals: Vec::new(),
            sell_signals: Vec::new(),
        }
    }

    /// 使用默认参数创建 UT Bot 指标
    pub fn default() -> Self {
        Self::new(1.0, 10, false)
    }

    /// 计算一组K线的 UT Bot 指标
    pub fn calculate(&mut self, candles: &[Candle]) -> (&[bool], &[bool]) {
        // 重置状态
        self.reset();

        if candles.len() < self.atr_period {
            return (&self.buy_signals, &self.sell_signals);
        }

        // 准备源数据
        self.prepare_source_data(candles);

        // 计算 ATR
        self.calculate_atr(candles);

        // 计算 EMA
        self.calculate_ema();

        // 计算跟踪止损线和信号
        for i in 0..candles.len() {
            self.update_trailing_stop(i);
            self.update_position(i);
            self.generate_signals(i);
        }

        (&self.buy_signals, &self.sell_signals)
    }

    /// 准备源数据 (普通收盘价或平均K线收盘价)
    fn prepare_source_data(&mut self, candles: &[Candle]) {
        if self.use_heikin_ashi {
            // 计算平均K线
            for i in 0..candles.len() {
                let ha_close = if i == 0 {
                    (candles[i].open + candles[i].high + candles[i].low + candles[i].close) / 4.0
                } else {
                    (self.source_values[i-1] +
                        (candles[i].high + candles[i].low + candles[i].close) / 3.0) / 2.0
                };
                self.source_values.push(ha_close);
            }
        } else {
            // 使用普通收盘价
            self.source_values = candles.iter().map(|c| c.close).collect();
        }
    }

    /// 计算 ATR 值
    fn calculate_atr(&mut self, candles: &[Candle]) {
        // 简单实现 ATR 计算
        for i in 0..candles.len() {
            if i == 0 {
                // 第一个值使用真实范围
                let tr = candles[i].high - candles[i].low;
                self.atr_values.push(tr);
                continue;
            }

            // 计算真实范围
            let tr1 = candles[i].high - candles[i].low;
            let tr2 = (candles[i].high - candles[i-1].close).abs();
            let tr3 = (candles[i].low - candles[i-1].close).abs();
            let tr = tr1.max(tr2).max(tr3);

            // 计算ATR
            if i < self.atr_period {
                // 简单平均
                let sum: f64 = self.atr_values.iter().sum::<f64>() + tr;
                self.atr_values.push(sum / (i as f64 + 1.0));
            } else {
                // 移动平均
                let prev_atr = self.atr_values[i-1];
                let atr = (prev_atr * (self.atr_period as f64 - 1.0) + tr) / self.atr_period as f64;
                self.atr_values.push(atr);
            }
        }
    }

    /// 计算 EMA(1) 值 (实际上就是原始价格)
    fn calculate_ema(&mut self) {
        self.ema_values = self.source_values.clone();
    }

    /// 更新跟踪止损线
    fn update_trailing_stop(&mut self, index: usize) {
        let src = self.source_values[index];
        let n_loss = self.key_value * self.atr_values[index];

        let prev_stop = if index > 0 {
            self.trailing_stop.get(index - 1).cloned().unwrap_or(0.0)
        } else {
            0.0
        };

        let prev_src = if index > 0 {
            self.source_values[index - 1]
        } else {
            src
        };

        let new_stop = if src > prev_stop && prev_src > prev_stop {
            // 上升趋势，止损线上移
            prev_stop.max(src - n_loss)
        } else if src < prev_stop && prev_src < prev_stop {
            // 下降趋势，止损线下移
            prev_stop.min(src + n_loss)
        } else if src > prev_stop {
            // 突破上轨，建立新的上升止损
            src - n_loss
        } else {
            // 突破下轨，建立新的下降止损
            src + n_loss
        };

        self.trailing_stop.push(new_stop);
    }

    /// 更新仓位状态
    fn update_position(&mut self, index: usize) {
        if index == 0 {
            self.position.push(0);
            return;
        }

        let src = self.source_values[index];
        let prev_src = self.source_values[index - 1];
        let stop = self.trailing_stop[index - 1];
        let prev_pos = self.position[index - 1];

        let new_pos = if prev_src < stop && src > stop {
            // 价格从下方突破止损线，转为多头
            1
        } else if prev_src > stop && src < stop {
            // 价格从上方跌破止损线，转为空头
            -1
        } else {
            // 保持原有仓位
            prev_pos
        };

        self.position.push(new_pos);
    }

    /// 生成交易信号
    fn generate_signals(&mut self, index: usize) {
        if index == 0 {
            self.buy_signals.push(false);
            self.sell_signals.push(false);
            return;
        }

        let src = self.source_values[index];
        let stop = self.trailing_stop[index];
        let ema = self.ema_values[index];
        let prev_ema = self.ema_values[index - 1];
        let prev_stop = self.trailing_stop[index - 1];

        // 检查EMA是否穿过止损线
        let above = prev_ema <= prev_stop && ema > stop;
        let below = prev_ema >= prev_stop && ema < stop;

        // 生成买入信号
        let buy = src > stop && above;
        self.buy_signals.push(buy);

        // 生成卖出信号
        let sell = src < stop && below;
        self.sell_signals.push(sell);
    }

    /// 重置指标状态
    pub fn reset(&mut self) {
        self.atr_values.clear();
        self.trailing_stop.clear();
        self.position.clear();
        self.source_values.clear();
        self.ema_values.clear();
        self.buy_signals.clear();
        self.sell_signals.clear();
    }

    /// 获取最新的跟踪止损值
    pub fn latest_stop(&self) -> Option<f64> {
        self.trailing_stop.last().cloned()
    }

    /// 获取最新的仓位状态
    pub fn latest_position(&self) -> Option<i32> {
        self.position.last().cloned()
    }

    /// 获取所有跟踪止损值
    pub fn trailing_stops(&self) -> &[f64] {
        &self.trailing_stop
    }

    /// 获取所有仓位状态
    pub fn positions(&self) -> &[i32] {
        &self.position
    }

    /// 获取所有买入信号
    pub fn buy_signals(&self) -> &[bool] {
        &self.buy_signals
    }

    /// 获取所有卖出信号
    pub fn sell_signals(&self) -> &[bool] {
        &self.sell_signals
    }

    /// 检查当前是否为多头状态 (价格在止损线上方)
    pub fn is_long(&self) -> bool {
        if let (Some(src), Some(stop)) = (self.source_values.last(), self.trailing_stop.last()) {
            src > stop
        } else {
            false
        }
    }

    /// 检查当前是否为空头状态 (价格在止损线下方)
    pub fn is_short(&self) -> bool {
        if let (Some(src), Some(stop)) = (self.source_values.last(), self.trailing_stop.last()) {
            src < stop
        } else {
            false
        }
    }
}

// 为 UTBot 实现 Debug trait 以便于打印调试信息
impl std::fmt::Debug for UTBot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UTBot(key={}, atr_period={}, heikin_ashi={})",
               self.key_value, self.atr_period, self.use_heikin_ashi)
    }
}
