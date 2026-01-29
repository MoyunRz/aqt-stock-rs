#[cfg(test)]
mod debug_tests {
    use aqt_stock::indicators::utils::{round_precision, format_precision};

    #[test]
    fn debug_atr_precision() {
        // 模拟ATR计算中的情况
        let test_values = vec![4.0, 4.123456, 3.987654, 4.000001];
        
        println!("ATR精度调试测试:");
        for val in test_values {
            let processed = round_precision(val);
            let formatted = format_precision(val);
            println!("原始值: {:.10} -> 处理后: {:.10} -> 格式化: {}", val, processed, formatted);
            
            // 检查小数位数
            let str_repr = format!("{:.10}", processed);
            let decimal_places = str_repr.split('.').nth(1).unwrap_or("").trim_end_matches('0').len();
            println!("  小数位数: {}", decimal_places);
        }
    }
}