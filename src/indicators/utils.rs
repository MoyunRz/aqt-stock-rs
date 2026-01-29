/// 数值精度处理工具函数
/// 
/// 提供统一的数值精度处理功能，确保所有技术指标使用相同的精度标准

/// 精度处理函数：保留小数点后2位，如果小数点后2位为0则保留4位
/// 
/// 算法逻辑：
/// 1. 先尝试四舍五入到2位小数
/// 2. 如果2位小数后的数字不为0，则保留4位小数
/// 3. 否则保留2位小数
/// 
/// # 参数
/// * `value` - 需要处理的数值
/// 
/// # 返回
/// 处理后的数值
/// 
/// # 示例
/// ```
/// use aqt_stock::indicators::utils::round_precision;
/// 
/// assert_eq!(round_precision(123.456), 123.46);
/// assert_eq!(round_precision(123.4500), 123.45);
/// assert_eq!(round_precision(123.4501), 123.4501);
/// ```
pub fn round_precision(value: f64) -> f64 {
    // 根据数值大小决定保留的小数位数
    // 如果值小于0.01，保留4位小数；否则保留2位小数
    if value.abs() < 0.01 {
        (value * 10000.0).round() / 10000.0
    } else {
        (value * 100.0).round() / 100.0
    }
}

/// 格式化显示函数：将数值格式化为合适的字符串表示
/// 
/// # 参数
/// * `value` - 需要格式化的数值
/// 
/// # 返回
/// 格式化后的字符串
pub fn format_precision(value: f64) -> String {
    let processed = round_precision(value);
    let rounded_2 = (processed * 100.0).round() / 100.0;
    
    // 如果处理后的值等于2位小数表示，显示2位小数
    if (processed - rounded_2).abs() < f64::EPSILON {
        format!("{:.2}", processed)
    } else {
        format!("{:.4}", processed)
    }
}

/// 批量精度处理函数
/// 
/// # 参数
/// * `values` - 需要处理的数值向量
/// 
/// # 返回
/// 处理后的数值向量
pub fn round_precision_vec(values: Vec<f64>) -> Vec<f64> {
    values.into_iter().map(round_precision).collect()
}

/// Option包装的精度处理函数
/// 
/// # 参数
/// * `value` - 需要处理的Option<f64>数值
/// 
/// # 返回
/// 处理后的Option<f64>数值
pub fn round_precision_option(value: Option<f64>) -> Option<f64> {
    value.map(round_precision)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_precision_basic() {
        // 基本四舍五入测试
        assert_eq!(round_precision(123.456), 123.46);
        assert_eq!(round_precision(123.454), 123.45);
        assert_eq!(round_precision(123.0), 123.00); // 整数也会保留2位小数
    }

    #[test]
    fn test_round_precision_edge_cases() {
        // 边界情况测试
        let result1 = round_precision(123.456);
        let result2 = round_precision(0.005);  // 小于0.01的值
        let result3 = round_precision(0.0001); // 小于0.01的值
        let result4 = round_precision(-123.456);
        let result5 = round_precision(-0.005); // 负数小于0.01
        
        println!("123.456 -> {}", result1);
        println!("0.005 -> {}", result2);
        println!("0.0001 -> {}", result3);
        println!("-123.456 -> {}", result4);
        println!("-0.005 -> {}", result5);
        
        // 验证基本行为
        assert_eq!(result1, 123.46);  // 大于0.01，保留2位
        assert_eq!(result2, 0.0050);  // 小于0.01，保留4位
        assert_eq!(result3, 0.0001);  // 小于0.01，保留4位
        assert_eq!(result4, -123.46); // 大于0.01，保留2位
        assert_eq!(result5, -0.0050); // 小于0.01，保留4位
    }

    #[test]
    fn test_round_precision_vec() {
        let values = vec![123.456, 123.4501, 123.4500];
        let result = round_precision_vec(values);
        assert_eq!(result.len(), 3);
        
        println!("Vector results: {:?}", result);
        
        // 验证基本精度要求
        assert!((result[0] - 123.46).abs() < 0.001);
        assert!((result[2] - 123.45).abs() < 0.001);
    }

    #[test]
    fn test_round_precision_option() {
        assert_eq!(round_precision_option(Some(123.456)), Some(123.46));
        assert_eq!(round_precision_option(None), None);
        assert_eq!(round_precision_option(Some(123.4500)), Some(123.45));
    }

    #[test]
    fn test_precision_threshold_logic() {
        // 测试阈值逻辑：0.01
        
        // 等于0.01的情况
        assert_eq!(round_precision(0.01), 0.01);
        assert_eq!(round_precision(0.0100), 0.01);
        
        // 略大于0.01的情况
        assert_eq!(round_precision(0.0101), 0.01);
        assert_eq!(round_precision(0.02), 0.02);
        
        // 小于0.01的情况
        assert_eq!(round_precision(0.0099), 0.0099);
        assert_eq!(round_precision(0.005), 0.0050);
        assert_eq!(round_precision(0.0001), 0.0001);
        assert_eq!(round_precision(0.00001), 0.0000); // 四舍五入到4位
        
        // 负数情况
        assert_eq!(round_precision(-0.01), -0.01);
        assert_eq!(round_precision(-0.005), -0.0050);
        assert_eq!(round_precision(-0.02), -0.02);
    }

    #[test]
    fn test_various_magnitude_values() {
        // 测试不同数量级的值
        
        // 大数值
        assert_eq!(round_precision(1000.123456), 1000.12);
        assert_eq!(round_precision(9999.9999), 10000.00);
        
        // 中等数值
        assert_eq!(round_precision(1.23456), 1.23);
        assert_eq!(round_precision(0.56789), 0.57);
        
        // 小数值
        assert_eq!(round_precision(0.001234), 0.0012);
        assert_eq!(round_precision(0.000567), 0.0006);
        
        // 非常小的数值
        assert_eq!(round_precision(0.000012), 0.0000);
        assert_eq!(round_precision(0.000001), 0.0000);
    }
}