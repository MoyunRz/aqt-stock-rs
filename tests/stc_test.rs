use aqt_stock::indicators::candle::Candle;
use aqt_stock::indicators::schaff_trend_cycle::SchaffTrendCycle;

// 测试函数
#[test]
fn stc_test() {
    // 创建模拟K线数据
    let mut candles = Vec::new();
    let prices = vec![
        100.0, 102.0, 104.0, 103.0, 105.0, 107.0, 109.0, 108.0, 106.0, 105.0,
        103.0, 101.0, 99.0, 98.0, 96.0, 95.0, 97.0, 99.0, 101.0, 103.0,
        105.0, 107.0, 109.0, 111.0, 113.0, 115.0, 114.0, 112.0, 110.0, 108.0,
        106.0, 104.0, 102.0, 100.0, 98.0, 96.0, 94.0, 92.0, 90.0, 92.0,
        94.0, 96.0, 98.0, 100.0, 102.0, 104.0, 106.0, 108.0, 110.0, 112.0
    ];

    // 生成模拟K线
    for (i, &price) in prices.iter().enumerate() {
        let open = if i > 0 { prices[i-1] } else { price * 0.99 };
        let high = price * 1.01;
        let low: f64 = f64::min(open, price) * 0.99;
        candles.push(Candle {
            timestamp: (i as u64) * 60000, // 假设1分钟K线
            open,
            high,
            low,
            close: price,
            volume: 1000.0 * (1.0 + (i as f64).sin()),
        });
    }

    // 创建STC指标
    let mut stc = SchaffTrendCycle::new(12, 26, 50);

    // 计算STC值
    let signals = stc.calculate(&candles);

    // 打印结果
    println!("STC分析:");
    println!("索引\t收盘价\tSTC值\t红色信号\t绿色信号");

    for (i, (candle, (stc_value, red_signal, green_signal))) in candles.iter().zip(signals.iter()).enumerate() {
        println!("{}\t{:.2}\t{:.2}\t{}\t{}",
                 i+1,
                 candle.close,
                 stc_value,
                 red_signal,
                 green_signal);
    }

    // 检查最新的信号
    if let Some(&(_, red_signal, green_signal)) = signals.last() {
        if red_signal {
            println!("最新K线生成卖出信号! (STC > 75 且转向下行)");
        }
        if green_signal {
            println!("最新K线生成买入信号! (STC < 25 且转向上行)");
        }
    }
}
