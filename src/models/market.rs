use longport::{decimal, Decimal};
use serde::{Deserialize};
use time::OffsetDateTime;

#[derive(Debug, Deserialize)]
pub struct MarketData {
    /// 股票代码
    pub symbol: String,
    /// 当前价格
    pub price: Decimal,
    /// 涨跌幅
    pub change: Decimal,
    /// 成交量
    pub volume: i64,
    /// 最高价格
    pub high: Decimal,
    /// 最低价格
    pub low: Decimal,
    /// 开盘价格
    pub open: Decimal,
    /// 收盘价格
    pub close: Decimal,
    pub ts: OffsetDateTime, // 修改: 设置默认时间
}

impl From<serde_json::Value> for MarketData {
    fn from(value: serde_json::Value) -> Self {
        MarketData {
            symbol: value["symbol"].as_str().unwrap_or("").to_string(),
            price: decimal!(value["price"].as_f64().unwrap_or(0.0)),
            change: decimal!(value["change"].as_f64().unwrap_or(0.0)),
            volume: value["volume"].as_i64().unwrap_or(0),
            high: decimal!(value["high"].as_f64().unwrap_or(0.0)),
            low: decimal!(value["low"].as_f64().unwrap_or(0.0)),
            open: decimal!(value["open"].as_f64().unwrap_or(0.0)),
            close: decimal!(value["close"].as_f64().unwrap_or(0.0)),
            ts: OffsetDateTime::now_utc(), // 修改: 设置默认时间为当前时间
        }
    }
}