
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Stock {
    pub symbol: String,
    pub name: String,
    pub price: f64,
    pub change: f64,
    pub volume: u64,
}