use aqt_stock::indicators::candle::Candle;
use aqt_stock::strategys::vecor_v1::VecorStrategy;

#[test]
fn test_timestamp_overflow_fix() {
    // 创建测试数据，模拟时间戳顺序错误的情况
    let candles = vec![
        Candle {
            symbol: Some("TEST".to_string()),
            timestamp: 1000, // 最新的时间戳
            open: 100.0,
            high: 110.0,
            low: 90.0,
            close: 105.0,
            volume: 1000.0,
        },
        Candle {
            symbol: Some("TEST".to_string()),
            timestamp: 2000, // 中间的时间戳
            open: 105.0,
            high: 115.0,
            low: 95.0,
            close: 110.0,
            volume: 1200.0,
        },
        Candle {
            symbol: Some("TEST".to_string()),
            timestamp: 3000, // 最早的时间戳
            open: 110.0,
            high: 120.0,
            low: 100.0,
            close: 115.0,
            volume: 1500.0,
        },
    ];

    // 测试正常情况（时间戳按时间顺序）
    let (symts, valid) = VecorStrategy::timestamp_to_time(candles.clone(), "TEST".to_string());
    assert!(valid, "正常时间戳顺序应该返回true");
    assert_eq!(symts.symbol, "TEST");
    assert_eq!(symts.interval_time, 1000); // 2000 - 3000 = 1000

    // 创建时间戳顺序错误的数据
    let invalid_candles = vec![
        Candle {
            symbol: Some("TEST".to_string()),
            timestamp: 3000, // 最新的时间戳
            open: 100.0,
            high: 110.0,
            low: 90.0,
            close: 105.0,
            volume: 1000.0,
        },
        Candle {
            symbol: Some("TEST".to_string()),
            timestamp: 2000, // 中间的时间戳
            open: 105.0,
            high: 115.0,
            low: 95.0,
            close: 110.0,
            volume: 1200.0,
        },
        Candle {
            symbol: Some("TEST".to_string()),
            timestamp: 1000, // 最早的时间戳
            open: 110.0,
            high: 120.0,
            low: 100.0,
            close: 115.0,
            volume: 1500.0,
        },
    ];

    // 测试时间戳顺序错误的情况
    let (symts, valid) = VecorStrategy::timestamp_to_time(invalid_candles, "TEST".to_string());
    // 即使时间戳顺序错误，也不应该panic，应该返回false表示数据无效
    assert!(!valid, "时间戳顺序错误应该返回false");
    assert_eq!(symts.interval_time, 0); // 应该返回0而不是panic
} 