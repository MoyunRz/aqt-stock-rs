#[derive(Clone)]
pub struct Candle {
    pub symbol: Option<String>,
    pub timestamp: u64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

impl Default for Candle {
    fn default() -> Self {
        Self {
            symbol: None,
            timestamp: 0,
            open: 0.0,
            high: 0.0,
            low: 0.0,
            close: 0.0,
            volume: 0.0,
        }
    }
}