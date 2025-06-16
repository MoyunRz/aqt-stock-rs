use aqt_stock::indicators::candle::Candle;
use aqt_stock::indicators::kdj::KDJ;

#[test]
fn kdj_test() {
    // 假设我们有一些K线数据
    let candles = vec![
        Candle { timestamp: 1, open: 10.0, high: 12.0, low: 9.0, close: 11.0, volume: 1000.0 },
        Candle { timestamp: 2, open: 11.0, high: 14.0, low: 10.0, close: 13.0, volume: 1500.0 },
        Candle { timestamp: 3, open: 13.0, high: 15.0, low: 12.5, close: 14.0, volume: 1200.0 },
        Candle { timestamp: 4, open: 14.0, high: 14.5, low: 13.0, close: 13.5, volume: 900.0 },
        Candle { timestamp: 5, open: 13.5, high: 13.8, low: 12.0, close: 12.5, volume: 1100.0 },
        Candle { timestamp: 6, open: 12.5, high: 13.0, low: 11.5, close: 12.0, volume: 800.0 },
        Candle { timestamp: 7, open: 12.0, high: 12.8, low: 11.8, close: 12.5, volume: 950.0 },
        Candle { timestamp: 8, open: 12.5, high: 13.5, low: 12.2, close: 13.2, volume: 1300.0 },
        Candle { timestamp: 9, open: 13.2, high: 14.0, low: 13.0, close: 13.8, volume: 1400.0 },
        Candle { timestamp: 10, open: 13.8, high: 14.5, low: 13.5, close: 14.2, volume: 1600.0 },
        Candle { timestamp: 11, open: 14.2, high: 15.0, low: 14.0, close: 14.8, volume: 1800.0 },
        Candle { timestamp: 12, open: 14.8, high: 15.2, low: 14.5, close: 15.0, volume: 2000.0 },
        Candle { timestamp: 13, open: 15.0, high: 15.5, low: 14.8, close: 15.3, volume: 1900.0 },
        Candle { timestamp: 14, open: 15.3, high: 15.4, low: 14.0, close: 14.2, volume: 2200.0 }, // 大幅下跌
        Candle { timestamp: 15, open: 14.2, high: 14.3, low: 13.5, close: 13.8, volume: 1700.0 },
        Candle { timestamp: 16, open: 13.8, high: 14.0, low: 13.0, close: 13.2, volume: 1500.0 },
        Candle { timestamp: 17, open: 13.2, high: 13.5, low: 12.8, close: 13.0, volume: 1400.0 },
        Candle { timestamp: 18, open: 13.0, high: 13.2, low: 12.0, close: 12.3, volume: 1300.0 },
        Candle { timestamp: 19, open: 12.3, high: 12.5, low: 11.5, close: 12.0, volume: 1100.0 },
        Candle { timestamp: 20, open: 12.0, high: 13.0, low: 11.8, close: 12.8, volume: 1400.0 }, // 反弹
    ];

    // 创建KDJ指标并计算
    let mut kdj = KDJ::default();
    let (k_values, d_values, j_values) = kdj.calculate(&candles);

    // 检查交易信号
    if kdj.is_golden_cross() {
        println!("发现金叉信号 - 可能的买入机会");
    }

    if kdj.is_oversold(20.0) {
        println!("KDJ超卖 - 可能的买入机会");
    }

    if kdj.is_death_cross() {
        println!("发现死叉信号 - 可能的卖出机会");
    }

    if kdj.is_overbought(80.0) {
        println!("KDJ超买 - 可能的卖出机会");
    }

    // 获取最新的KDJ值
    if let Some((k, d, j)) = kdj.latest() {
        println!("最新KDJ值: K={:.2}, D={:.2}, J={:.2}", k, d, j);
    }
}