use crate::models::candle::Candle;

pub struct ATR;

impl ATR {
    pub fn new() -> Self {
        ATR
    }

    pub fn default() -> Self {
        Self::new()
    }

    pub fn calculate(&mut self, candles: &[Candle], period: usize) -> Vec<Option<f64>> {
        let len = candles.len();
        if len == 0 || period == 0 || period > len {
            return vec![None; len];
        }

        let mut trs = Vec::with_capacity(len);
        for (i, c) in candles.iter().enumerate() {
            if i == 0 {
                trs.push(c.high - c.low);
            } else {
                let prev_close = candles[i - 1].close;
                let hl = c.high - c.low;
                let hc = (c.high - prev_close).abs();
                let lc = (c.low - prev_close).abs();
                let tr = hl.max(hc).max(lc);
                trs.push(tr);
            }
        }

        let mut atr = vec![None; len];

        let mut sum = 0.0;
        for i in 0..period {
            sum += trs[i];
        }
        let mut prev_atr = sum / period as f64;
        atr[period - 1] = Some(prev_atr);

        for i in period..len {
            let tr = trs[i];
            prev_atr = (prev_atr * (period as f64 - 1.0) + tr) / period as f64;
            atr[i] = Some(prev_atr);
        }

        atr
    }
}

