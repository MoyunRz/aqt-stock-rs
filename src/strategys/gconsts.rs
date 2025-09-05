// 枚举
// #[derive(Debug, Clone, Copy, PartialEq)]
// pub enum Environment {
//     DEV,
//     LOCAL,
//     PROD,
// }

pub fn get_time_by_period(period: &str) -> i64 {
    match period {
        "1m" => 60 * 1000,
        "5m" => 300 * 1000,
        "15m" => 900 * 1000,
        "30m" => 1800 * 1000,
        "1h" => 3600 * 1000,
        "4h" => 14400 * 1000,
        "1d" => 86400 * 1000,
        _ => 1000 * 180,
    }
}
