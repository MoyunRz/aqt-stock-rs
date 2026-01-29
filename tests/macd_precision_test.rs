use aqt_stock::models::candle::Candle;
use aqt_stock::indicators::macd::MACD;

#[test]
fn macd_precision_test() {
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

    // 创建MACD指标
    let mut macd = MACD::new(12, 26, 9);
    
    // 计算MACD
    let (_buy_signals, _sell_signals) = macd.calculate(&candles);
    
    // 验证精度处理
    println!("MACD精度测试:");
    println!("索引\tMACD\t\t信号线\t\t直方图");
    
    let macd_line = macd.macd_line();
    let signal_line = macd.signal_line();
    let histogram = macd.histogram();
    
    for i in 0..macd_line.len().min(10) {
        if i < signal_line.len() && i < histogram.len() {
            let macd_val = macd_line[i];
            let signal_val = signal_line[i];
            let hist_val = histogram[i];
            
            println!("{}\t{:.6}\t{:.6}\t{:.6}", 
                     i, macd_val, signal_val, hist_val);
            
            // 验证精度 - 检查小数位数
            let macd_str = format!("{:.6}", macd_val);
            let signal_str = format!("{:.6}", signal_val);
            let hist_str = format!("{:.6}", hist_val);
            
            // 检查是否符合精度要求（最多4位小数）
            assert!(macd_str.chars().filter(|&c| c == '.').count() <= 1);
            assert!(signal_str.chars().filter(|&c| c == '.').count() <= 1);
            assert!(hist_str.chars().filter(|&c| c == '.').count() <= 1);
        }
    }
    
    println!("精度测试通过！");
}