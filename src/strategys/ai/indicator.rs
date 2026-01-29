use crate::indicators::atr::ATR;
use crate::indicators::ema::EMA;
use crate::indicators::macd::MACD;
use crate::indicators::rsi::RSI;
use crate::indicators::sma::SMA;
use crate::models::candle::Candle;
use crate::services::service::Service;
use crate::strategys::ai::model::{AtrData, Candlestick as AICandlestick, InObj, Indicators, MacdData};
use crate::strategys::vecor_v1::VecorStrategy;
use serde_json;
use tokio;

async fn fetch_candles(service: &Service, symbol: &str, period: &str) -> Vec<Candle> {
    let raw = service
        .get_candlesticks(symbol.to_string(), period.to_string())
        .await;
    if raw.is_empty() {
        return Vec::new();
    }
    VecorStrategy::handle_candles(symbol.to_string(), raw)
}

fn build_k_data(candles: &[Candle]) -> String {
    let data: Vec<AICandlestick> = candles
        .iter()
        .map(|c| AICandlestick {
            t: Some(c.timestamp as i64),
            v: Some(c.volume as i64),
            c: Some(c.close.to_string()),
            h: Some(c.high.to_string()),
            l: Some(c.low.to_string()),
            o: Some(c.open.to_string()),
            sum: None,
        })
        .collect();
    serde_json::to_string(&data).unwrap_or_else(|_| "[]".to_string())
}

fn build_atr_data(candles: &[Candle], period: usize) -> Vec<AtrData> {
    if candles.is_empty() || period == 0 || period > candles.len() {
        return Vec::new();
    }
    let mut trs = Vec::with_capacity(candles.len());
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
    let mut atr_calc = ATR::default();
    let atr_vals = atr_calc.calculate(candles, period);
    let mut result = Vec::new();
    for (i, atr_opt) in atr_vals.iter().enumerate() {
        if let Some(atr) = atr_opt {
            result.push(AtrData {
                tr: trs[i],
                atr: *atr,
            });
        }
    }
    result
}

fn build_indicators(candles: &[Candle]) -> Option<Indicators> {
    let len = candles.len();
    if len == 0 {
        return None;
    }
    let candles_tail = candles;
    let k_data = build_k_data(candles_tail);
    let volumes: Vec<f64> = candles.iter().map(|c| c.volume).collect();
    let mut macd = MACD::new(12, 26, 9);
    let _ = macd.calculate(candles);
    let macd_line = macd.macd_line();
    let signal_line = macd.signal_line();
    let hist = macd.histogram();
    let mut macd_vec = Vec::new();
    for i in 0..macd_line.len() {
        let dif = macd_line[i];
        let dea = signal_line.get(i).cloned().unwrap_or(0.0);
        let m = hist.get(i).cloned().unwrap_or(0.0);
        macd_vec.push(MacdData { dif, dea, macd: m });
    }
    let mut rsi7_calc = RSI::default();
    let rsi7_raw = rsi7_calc.calculate(candles, 7);
    let rsi7: Vec<f64> = rsi7_raw
        .iter()
        .filter_map(|v| *v)
        .collect();
    let mut rsi14_calc = RSI::default();
    let rsi14_raw = rsi14_calc.calculate(candles, 14);
    let rsi14: Vec<f64> = rsi14_raw
        .iter()
        .filter_map(|v| *v)
        .collect();
    let mut ema_calc = EMA::default();
    let ema_raw = ema_calc.calculate(candles, 5);
    let ema: Vec<f64> = ema_raw
        .iter()
        .filter_map(|v| *v)
        .collect();
    let mut sma_calc = SMA::default();
    let sma_raw = sma_calc.calculate(candles, 5);
    let sma: Vec<f64> = sma_raw
        .iter()
        .filter_map(|v| *v)
        .collect();
    let atr3 = build_atr_data(candles, 3);
    let atr14 = build_atr_data(candles, 14);
    
    Some(Indicators {
        k_data: if k_data.len() > 32 {
            k_data[k_data.len() - 32..].to_string()
        } else {
            k_data
        },
        macd: macd_vec,
        kdj: Vec::new(),
        rsi7,
        rsi14,
        ema,
        sma,
        vol: volumes,
        atr3,
        atr14,
    })
}

fn build_inobj_from_candles(candles_4h: Vec<Candle>, candles_15m: Vec<Candle>) -> Option<InObj> {
    if candles_4h.is_empty() || candles_15m.is_empty() {
        return None;
    }
    let ind4 = build_indicators(&candles_4h.as_slice())?;
    let ind3 = build_indicators(&candles_15m.as_slice())?;
    let mark_px = candles_15m.last().map(|c| c.close).unwrap_or(0.0);
    Some(InObj {
        mark_px,
        ind3: Some(ind3),
        ind4: Some(ind4),
    })
}

pub async fn build_inobj(service: &Service, symbol: &str) -> Option<InObj> {
    let (candles_4h, candles_15m) = tokio::join!(
        fetch_candles(service, symbol, "1d"),
        fetch_candles(service, symbol, "2h")
    );
    build_inobj_from_candles(candles_4h, candles_15m)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use longport::{Config, QuoteContext, TradeContext};
    use crate::config::config::Configs;

    fn sample_candles(len: usize) -> Vec<Candle> {
        let mut v = Vec::with_capacity(len);
        for i in 0..len {
            v.push(Candle {
                symbol: None,
                timestamp: i as u64,
                open: 100.0 + i as f64,
                high: 101.0 + i as f64,
                low: 99.0 + i as f64,
                close: 100.5 + i as f64,
                volume: 1000.0 + i as f64,
            });
        }
        v
    }

    #[test]
    fn test_build_k_data_outputs_json() {
        let candles = sample_candles(3);
        let json = build_k_data(&candles);
        assert!(json.starts_with('['));
        assert!(json.contains("\"t\""));
        assert!(json.contains("\"c\""));
    }

    #[test]
    fn test_build_indicators_has_values() {
        let candles = sample_candles(20);
        let ind = build_indicators(&candles).expect("indicators should be built");
        assert!(!ind.macd.is_empty());
        assert!(!ind.rsi7.is_empty());
        assert!(!ind.rsi14.is_empty());
        assert!(!ind.ema.is_empty());
        assert!(!ind.sma.is_empty());
        assert!(!ind.atr3.is_empty());
        assert!(!ind.atr14.is_empty());
        assert_eq!(ind.vol.len(), candles.len());
    }

    #[test]
    fn test_build_indicators_returns_none_when_empty() {
        let candles = Vec::new();
        let ind = build_indicators(&candles);
        assert!(ind.is_none());
    }

    #[test]
    fn test_build_inobj_from_candles_empty_returns_none() {
        let candles_4h = Vec::new();
        let candles_15m = sample_candles(20);
        let res = build_inobj_from_candles(candles_4h, candles_15m);
        assert!(res.is_none());
    }

    #[test]
    fn test_build_inobj_from_candles_non_empty_returns_some() {
        let candles_4h = sample_candles(20);
        let candles_15m = sample_candles(20);
        let last_close = candles_15m.last().unwrap().close;
        let res = build_inobj_from_candles(candles_4h, candles_15m).expect("should be some");
        assert_eq!(res.mark_px, last_close);
        assert!(res.ind3.is_some());
        assert!(res.ind4.is_some());
    }

    #[tokio::test]
    async fn test_build_inobj_smoke() {
        let cfg = Arc::new(Config::from_env().unwrap());
        let quote_res = QuoteContext::try_new(cfg.clone()).await;
        let quote_ctx = match quote_res {
            Ok((ctx, _)) => Arc::new(ctx),
            Err(_) => return,
        };
        let trade_res = TradeContext::try_new(cfg.clone()).await;
        let trade_ctx = match trade_res {
            Ok((ctx, _)) => Arc::new(ctx),
            Err(_) => return,
        };
        let service = Service::new(quote_ctx, trade_ctx);
        let configs = Configs::load().unwrap();
        let symbol = configs
            .symbols
            .first()
            .map(|s| s.symbol.as_str())
            .unwrap_or("AAPL.US");
        let s = build_inobj(&service, symbol).await;
        // 打印
        println!("{:?}", s);
    }
}


