mod kdj_test;

use aqt_stock::indicators::candle::Candle;
use aqt_stock::indicators::utbot::UTBot;
#[test]
fn utbot_test() {
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

    // 创建 UT Bot 指标并计算
    let mut ubot = UTBot::new(1.0, 10, false);
    // 不再保存返回值，直接调用计算方法
    ubot.calculate(&candles);  

    // 获取最新状态
    if let Some(stop) = ubot.latest_stop() {
        println!("最新跟踪止损线: {:.2}", stop);
    }

    if ubot.is_long() {
        println!("当前处于多头状态");
    } else if ubot.is_short() {
        println!("当前处于空头状态");
    }

    // 检查最新信号
    let buy_signals = ubot.buy_signals();  // 使用方法获取信号数组
    if let Some(&buy) = buy_signals.last() {
        if buy {
            println!("生成买入信号!");
        }
    }

    let sell_signals = ubot.sell_signals();  // 使用方法获取信号数组
    if let Some(&sell) = sell_signals.last() {
        if sell {
            println!("生成卖出信号!");
        }
    }

    // 获取所有的跟踪止损值，可用于绘制图表
    let stops = ubot.trailing_stops();
    println!("跟踪止损线数据点数量: {}", stops.len());

    // 打印每个K线对应的止损线和信号情况，便于分析
    println!("\n价格和信号分析:");
    println!("索引\t收盘价\t止损线\t多空\t买入\t卖出");
    for i in 0..candles.len() {
        println!("{}\t{:.2}\t{:.2}\t{}\t{}\t{}",
                 i+1,
                 candles[i].close,
                 ubot.trailing_stops()[i],
                 ubot.positions()[i],
                 ubot.buy_signals()[i],
                 ubot.sell_signals()[i]);
    }
}
