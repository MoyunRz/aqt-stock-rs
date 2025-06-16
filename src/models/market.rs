use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct MarketData {
    /// 股票代码
    pub symbol: String,
    /// 当前价格
    pub price: f64,
    /// 涨跌幅
    pub change: f64,
    /// 成交量
    pub volume: u64,
    /// 最高价格
    pub high: f64,
    /// 最低价格
    pub low: f64,
    /// 开盘价格
    pub open: f64,
    /// 收盘价格
    pub close: f64,
}

impl From<serde_json::Value> for MarketData {
    fn from(value: serde_json::Value) -> Self {
        MarketData {
            symbol: value["symbol"].as_str().unwrap_or("").to_string(),
            price: value["price"].as_f64().unwrap_or(0.0),
            change: value["change"].as_f64().unwrap_or(0.0),
            volume: value["volume"].as_u64().unwrap_or(0),
            high: value["high"].as_f64().unwrap_or(0.0),
            low: value["low"].as_f64().unwrap_or(0.0),
            open: value["open"].as_f64().unwrap_or(0.0),
            close: value["close"].as_f64().unwrap_or(0.0),
        }
    }
}