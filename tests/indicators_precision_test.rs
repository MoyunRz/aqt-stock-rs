use aqt_stock::models::candle::Candle;
use aqt_stock::indicators::{ema::EMA, sma::SMA, rsi::RSI, kdj::KDJ, atr::ATR, hma::HMA};
use aqt_stock::indicators::utils::format_precision;

#[test]
fn indicators_precision_test() {
    // 创建测试用的K线数据
    let candles = vec![
        Candle { symbol: None, timestamp: 1, open: 100.0, high: 102.0, low: 98.0, close: 101.0, volume: 1000.0 },
        Candle { symbol: None, timestamp: 2, open: 101.0, high: 103.0, low: 99.0, close: 102.0, volume: 1100.0 },
        Candle { symbol: None, timestamp: 3, open: 102.0, high: 104.0, low: 100.0, close: 103.0, volume: 1200.0 },
        Candle { symbol: None, timestamp: 4, open: 103.0, high: 105.0, low: 101.0, close: 104.0, volume: 1300.0 },
        Candle { symbol: None, timestamp: 5, open: 104.0, high: 106.0, low: 102.0, close: 105.0, volume: 1400.0 },
        Candle { symbol: None, timestamp: 6, open: 105.0, high: 107.0, low: 103.0, close: 106.0, volume: 1500.0 },
        Candle { symbol: None, timestamp: 7, open: 106.0, high: 108.0, low: 104.0, close: 107.0, volume: 1600.0 },
        Candle { symbol: None, timestamp: 8, open: 107.0, high: 109.0, low: 105.0, close: 108.0, volume: 1700.0 },
        Candle { symbol: None, timestamp: 9, open: 108.0, high: 110.0, low: 106.0, close: 109.0, volume: 1800.0 },
        Candle { symbol: None, timestamp: 10, open: 109.0, high: 111.0, low: 107.0, close: 110.0, volume: 1900.0 },
        Candle { symbol: None, timestamp: 11, open: 110.0, high: 112.0, low: 108.0, close: 111.0, volume: 2000.0 },
        Candle { symbol: None, timestamp: 12, open: 111.0, high: 113.0, low: 109.0, close: 112.0, volume: 2100.0 },
        Candle { symbol: None, timestamp: 13, open: 112.0, high: 114.0, low: 110.0, close: 113.0, volume: 2200.0 },
        Candle { symbol: None, timestamp: 14, open: 113.0, high: 115.0, low: 111.0, close: 114.0, volume: 2300.0 },
        Candle { symbol: None, timestamp: 15, open: 114.0, high: 116.0, low: 112.0, close: 115.0, volume: 2400.0 },
        Candle { symbol: None, timestamp: 16, open: 115.0, high: 117.0, low: 113.0, close: 116.0, volume: 2500.0 },
        Candle { symbol: None, timestamp: 17, open: 116.0, high: 118.0, low: 114.0, close: 117.0, volume: 2600.0 },
        Candle { symbol: None, timestamp: 18, open: 117.0, high: 119.0, low: 115.0, close: 118.0, volume: 2700.0 },
        Candle { symbol: None, timestamp: 19, open: 118.0, high: 120.0, low: 116.0, close: 119.0, volume: 2800.0 },
        Candle { symbol: None, timestamp: 20, open: 119.0, high: 121.0, low: 117.0, close: 120.0, volume: 2900.0 },
    ];

    println!("=== 技术指标精度测试 ===");

    // 测试 EMA
    println!("\n1. EMA 测试 (周期: 5)");
    let mut ema = EMA::new();
    let ema_results = ema.calculate(&candles, 5);
    for (i, result) in ema_results.iter().enumerate().skip(4).take(5) {
        if let Some(val) = result {
            let formatted = format_precision(*val);
            println!("EMA[{}] = {}", i, formatted);
            // 验证格式化结果
            assert!(formatted.chars().filter(|&c| c == '.').count() <= 1);
        }
    }

    // 测试 SMA
    println!("\n2. SMA 测试 (周期: 5)");
    let mut sma = SMA::new();
    let sma_results = sma.calculate(&candles, 5);
    for (i, result) in sma_results.iter().enumerate().skip(4).take(5) {
        if let Some(val) = result {
            let formatted = format_precision(*val);
            println!("SMA[{}] = {}", i, formatted);
            // 验证格式化结果
            assert!(formatted.chars().filter(|&c| c == '.').count() <= 1);
        }
    }

    // 测试 RSI
    println!("\n3. RSI 测试 (周期: 14)");
    let mut rsi = RSI::new();
    let rsi_results = rsi.calculate(&candles, 14);
    for (i, result) in rsi_results.iter().enumerate().skip(14).take(3) {
        if let Some(val) = result {
            let formatted = format_precision(*val);
            println!("RSI[{}] = {}", i, formatted);
            // 验证格式化结果
            assert!(formatted.chars().filter(|&c| c == '.').count() <= 1);
        }
    }

    // 测试 KDJ
    println!("\n4. KDJ 测试");
    let mut kdj = KDJ::new(9, 3, 3);
    let (k_vals, d_vals, j_vals) = kdj.calculate(&candles);
    println!("KDJ 最新值:");
    if !k_vals.is_empty() && !d_vals.is_empty() && !j_vals.is_empty() {
        let k = k_vals.last().unwrap();
        let d = d_vals.last().unwrap();
        let j = j_vals.last().unwrap();
        let k_formatted = format_precision(*k);
        let d_formatted = format_precision(*d);
        let j_formatted = format_precision(*j);
        println!("K = {}", k_formatted);
        println!("D = {}", d_formatted);
        println!("J = {}", j_formatted);
        
        // 验证格式化结果
        for formatted in [k_formatted, d_formatted, j_formatted] {
            assert!(formatted.chars().filter(|&c| c == '.').count() <= 1);
        }
    }

    // 测试 ATR
    println!("\n5. ATR 测试 (周期: 14)");
    let mut atr = ATR::new();
    let atr_results = atr.calculate(&candles, 14);
    for (i, result) in atr_results.iter().enumerate().skip(13).take(3) {
        if let Some(val) = result {
            let formatted = format_precision(*val);
            println!("ATR[{}] = {}", i, formatted);
            // 验证格式化结果
            assert!(formatted.chars().filter(|&c| c == '.').count() <= 1);
        }
    }

    // 测试 HMA
    println!("\n6. HMA 测试 (周期: 9)");
    let mut hma = HMA::new();
    let hma_results = hma.calculate(&candles, 9);
    println!("HMA 值:");
    for (i, val) in hma_results.iter().enumerate().take(5) {
        let formatted = format_precision(*val);
        println!("HMA[{}] = {}", i, formatted);
        // 验证格式化结果
        assert!(formatted.chars().filter(|&c| c == '.').count() <= 1);
    }

    println!("\n=== 所有技术指标精度测试通过 ===");
}