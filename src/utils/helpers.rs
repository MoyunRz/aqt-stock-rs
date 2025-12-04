use chrono::{Datelike, Timelike, Utc, Weekday};
use log::info;


pub  fn do_run_time() ->bool {
    let now = Utc::now();
    let weekday = now.weekday();
    let hour = now.hour();
    let minute = now.minute();
    // 检查是否是周末
    if matches!(weekday, Weekday::Sat | Weekday::Sun) {
        info!("周末不运行，当前时间: {:?}", weekday);
        return false
    }
    // 检查是否在运行时间范围内 (UTC 21:30 - 04:00)
    let is_trading_time = (hour+8 >= 21 ) && (hour+8 <=29);
    if !is_trading_time {
        info!("非交易时间，当前时间: {}:{}，等待至下一个交易时段", hour+8, minute);
        return false
    }
    true
}