use aqt_stock::indicators::candle::Candle;
use aqt_stock::indicators::macd::MACD;

#[test]
fn macd_test() {
    // 假设我们有一些K线数据
    let candles = vec![
        Candle { symbol: None,timestamp: 1, open: 10.0, high: 12.0, low: 9.0, close: 11.0, volume: 1000.0 },
        Candle { symbol: None,timestamp: 2, open: 11.0, high: 14.0, low: 10.0, close: 13.0, volume: 1500.0 },
        Candle { symbol: None,timestamp: 3, open: 13.0, high: 15.0, low: 12.5, close: 14.0, volume: 1200.0 },
        Candle { symbol: None,timestamp: 4, open: 14.0, high: 14.5, low: 13.0, close: 13.5, volume: 900.0 },
        Candle { symbol: None,timestamp: 5, open: 13.5, high: 13.8, low: 12.0, close: 12.5, volume: 1100.0 },
        Candle { symbol: None,timestamp: 6, open: 12.5, high: 13.0, low: 11.5, close: 12.0, volume: 800.0 },
        Candle { symbol: None,timestamp: 7, open: 12.0, high: 12.8, low: 11.8, close: 12.5, volume: 950.0 },
        Candle { symbol: None,timestamp: 8, open: 12.5, high: 13.5, low: 12.2, close: 13.2, volume: 1300.0 },
        Candle { symbol: None,timestamp: 9, open: 13.2, high: 14.0, low: 13.0, close: 13.8, volume: 1400.0 },
        Candle { symbol: None,timestamp: 10, open: 13.8, high: 14.5, low: 13.5, close: 14.2, volume: 1600.0 },
        Candle { symbol: None,timestamp: 11, open: 14.2, high: 15.0, low: 14.0, close: 14.8, volume: 1800.0 },
        Candle { symbol: None,timestamp: 12, open: 14.8, high: 15.2, low: 14.5, close: 15.0, volume: 2000.0 },
        Candle { symbol: None,timestamp: 13, open: 15.0, high: 15.5, low: 14.8, close: 15.3, volume: 1900.0 },
        Candle { symbol: None,timestamp: 14, open: 15.3, high: 15.4, low: 14.0, close: 14.2, volume: 2200.0 }, // 大幅下跌
        Candle { symbol: None,timestamp: 15, open: 14.2, high: 14.3, low: 13.5, close: 13.8, volume: 1700.0 },
        Candle { symbol: None,timestamp: 16, open: 13.8, high: 14.0, low: 13.0, close: 13.2, volume: 1500.0 },
        Candle { symbol: None,timestamp: 17, open: 13.2, high: 13.5, low: 12.8, close: 13.0, volume: 1400.0 },
        Candle { symbol: None,timestamp: 18, open: 13.0, high: 13.2, low: 12.0, close: 12.3, volume: 1300.0 },
        Candle { symbol: None,timestamp: 19, open: 12.3, high: 12.5, low: 11.5, close: 12.0, volume: 1100.0 },
        Candle { symbol: None,timestamp: 20, open: 12.0, high: 13.0, low: 11.8, close: 12.8, volume: 1400.0 }, // 反弹
    ];
    

    // 创建MACD指标
    let mut macd = MACD::new(12, 26, 9);

    // 设置显示选项
    macd.set_display_options(true, true, true, true, true);

    // 计算MACD
    let (buy_signals, sell_signals) = macd.calculate(&candles);

    // 打印结果
    println!("\nMACD分析:");
    println!("索引\t收盘价\tMACD\t信号线\t直方图\t买入\t卖出");

    for i in 0..candles.len() {
        if i < macd.macd_line().len() && i < macd.signal_line().len() && i < macd.histogram().len() {
            println!("{}\t{:.2}\t{:.4}\t{:.4}\t{:.4}\t{}\t{}",
                     i+1,
                     candles[i].close,
                     macd.macd_line()[i],
                     macd.signal_line()[i],
                     macd.histogram()[i],
                     buy_signals[i],
                     sell_signals[i]);
        }
    }
}