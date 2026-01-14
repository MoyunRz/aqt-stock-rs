use crate::models::candle::Candle;

pub struct SMA;

impl SMA {
    pub fn new() -> Self {
        SMA
    }

    pub fn default() -> Self {
        Self::new()
    }

    pub fn calculate(&mut self, candles: &[Candle], period: usize) -> Vec<Option<f64>> {
        let len = candles.len();
        if len == 0 || period == 0 || period > len {
            return vec![None; len];
        }

        let mut result = vec![None; len];
        let mut sum = 0.0;

        for i in 0..len {
            sum += candles[i].close;
            if i >= period {
                sum -= candles[i - period].close;
            }
            if i + 1 >= period {
                result[i] = Some(sum / period as f64);
            }
        }

        result
    }
}

