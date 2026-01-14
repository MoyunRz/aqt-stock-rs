use crate::models::candle::Candle;

pub struct RSI;

impl RSI {
    pub fn new() -> Self {
        RSI
    }

    pub fn default() -> Self {
        Self::new()
    }

    pub fn calculate(&mut self, candles: &[Candle], period: usize) -> Vec<Option<f64>> {
        let len = candles.len();
        if len == 0 || period == 0 || len <= period {
            return vec![None; len];
        }

        let mut rsi = vec![None; len];

        let mut gains = Vec::with_capacity(len);
        let mut losses = Vec::with_capacity(len);
        gains.push(0.0);
        losses.push(0.0);

        for i in 1..len {
            let diff = candles[i].close - candles[i - 1].close;
            if diff > 0.0 {
                gains.push(diff);
                losses.push(0.0);
            } else {
                gains.push(0.0);
                losses.push(-diff);
            }
        }

        let mut avg_gain = 0.0;
        let mut avg_loss = 0.0;
        for i in 1..=period {
            avg_gain += gains[i];
            avg_loss += losses[i];
        }
        avg_gain /= period as f64;
        avg_loss /= period as f64;

        if avg_loss == 0.0 {
            rsi[period] = Some(100.0);
        } else {
            let rs = avg_gain / avg_loss;
            rsi[period] = Some(100.0 - 100.0 / (1.0 + rs));
        }

        for i in (period + 1)..len {
            avg_gain = (avg_gain * (period as f64 - 1.0) + gains[i]) / period as f64;
            avg_loss = (avg_loss * (period as f64 - 1.0) + losses[i]) / period as f64;

            if avg_loss == 0.0 {
                rsi[i] = Some(100.0);
            } else {
                let rs = avg_gain / avg_loss;
                rsi[i] = Some(100.0 - 100.0 / (1.0 + rs));
            }
        }

        rsi
    }
}

