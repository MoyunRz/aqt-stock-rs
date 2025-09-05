const FIB_LEVELS: [f64; 7] = [0.0, 0.236, 0.382, 0.5, 0.618, 0.786, 1.0];

fn calculate_fibonacci_retracement(high: f64, low: f64) -> Result<Vec<f64>, &'static str> {
    if high <= low {
        return Err("High price must be greater than low price");
    }
    let difference = high - low;
    let mut retracement_levels = Vec::new();

    for &level in FIB_LEVELS.iter() {
        let retracement = if level == 0.236 || level == 0.786 {
            // 对于 23.6% 和 78.6%，使用反向计算
            low + (difference * (1.0 - level))
        } else {
            // 其他水平使用标准公式
            high - (difference * level)
        };
        // 四舍五入到两位小数
        let rounded = (retracement * 100.0).round() / 100.0;
        retracement_levels.push(rounded);
    }
    Ok(retracement_levels)
}

#[test]
fn fibonacci_test() {
    let high_price = 488.99; // 股票价格高点
    let low_price = 183.9;  // 股票价格低点

    match calculate_fibonacci_retracement(high_price, low_price) {
        Ok(retracement_levels) => {
            for level in retracement_levels.iter() {
                println!("level: {:.2}", level);
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}