use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct SymbolTimeData {
    /// 股票代码
    pub symbol: String,
    pub interval_time: u64,
    pub next_time: u64,
    pub last_time: u64,
}

impl SymbolTimeData {
    pub fn new() -> Self {
        SymbolTimeData {
            symbol: "".to_string(),
            interval_time:0,
            next_time:0,
            last_time: 0,
        }
    }
}

impl From<serde_json::Value> for SymbolTimeData {
    fn from(value: serde_json::Value) -> Self {
        SymbolTimeData {
            symbol: value["symbol"].as_str().unwrap_or("").to_string(),
            interval_time: value["interval_time"].as_u64().unwrap_or(0),
            next_time: value["next_time"].as_u64().unwrap_or(0),
            last_time: value["last_time"].as_u64().unwrap_or(0),
        }
    }
}