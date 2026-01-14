use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatContent {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "action")]
    pub action: String,
    #[serde(rename = "confidence")]
    pub confidence: f64,
    #[serde(rename = "rationale")]
    pub rationale: String,
    #[serde(rename = "target")]
    pub target: Value,
    #[serde(rename = "stop")]
    pub stop: Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InObj {
    #[serde(rename = "MarkPx")]
    pub mark_px: f64,
    #[serde(rename = "Ind3")]
    pub ind3: Option<Indicators>,
    #[serde(rename = "Ind4")]
    pub ind4: Option<Indicators>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Indicators {
    #[serde(rename = "k_data")]
    pub k_data: String,
    #[serde(rename = "MACD")]
    pub macd: Vec<MacdData>,
    #[serde(rename = "KDJ")]
    pub kdj: Vec<KdjData>,
    #[serde(rename = "RSI7")]
    pub rsi7: Vec<f64>,
    #[serde(rename = "RSI14")]
    pub rsi14: Vec<f64>,
    #[serde(rename = "Ema")]
    pub ema: Vec<f64>,
    #[serde(rename = "Sma")]
    pub sma: Vec<f64>,
    #[serde(rename = "Vol")]
    pub vol: Vec<f64>,
    #[serde(rename = "atr3")]
    pub atr3: Vec<AtrData>,
    #[serde(rename = "atr14")]
    pub atr14: Vec<AtrData>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Candlestick {
    #[serde(rename = "t")]
    pub t: Option<i64>,
    #[serde(rename = "v")]
    pub v: Option<i64>,
    #[serde(rename = "c")]
    pub c: Option<String>,
    #[serde(rename = "h")]
    pub h: Option<String>,
    #[serde(rename = "l")]
    pub l: Option<String>,
    #[serde(rename = "o")]
    pub o: Option<String>,
    #[serde(rename = "sum")]
    pub sum: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MacdData {
    #[serde(rename = "DIF")]
    pub dif: f64,
    #[serde(rename = "DEA")]
    pub dea: f64,
    #[serde(rename = "Macd")]
    pub macd: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KdjData {
    #[serde(rename = "RSV")]
    pub rsv: f64,
    #[serde(rename = "K")]
    pub k: f64,
    #[serde(rename = "D")]
    pub d: f64,
    #[serde(rename = "J")]
    pub j: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AtrData {
    #[serde(rename = "TR")]
    pub tr: f64,
    #[serde(rename = "Atr")]
    pub atr: f64,
}
