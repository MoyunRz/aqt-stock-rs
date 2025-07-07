use reqwest::Client;
use serde_json::Value;
use moka::sync::Cache;
use std::time::Duration;
use once_cell::sync::Lazy;
use std::sync::Arc;
use log::{info, warn};

/// 全局缓存 - 使用 once_cell 创建静态缓存实例
static TECHNICALS_CACHE: Lazy<Arc<Cache<String, Value>>> = Lazy::new(|| {
    Arc::new(
        Cache::builder()
            .time_to_live(Duration::from_secs(300)) // 5分钟缓存过期
            .max_capacity(1000) // 最大缓存1000个symbol
            .build()
    )
});

/// TradingviewTechnicals 指标结构体
#[derive(Clone)]
pub struct TradingTechnicals {
     values:  Value,
}

impl TradingTechnicals {

    pub async fn default() -> Self {
        let new_instance = Self::new("NASDAQ:TSLA").await;
        new_instance
    }

    pub async fn new(symbol: &str) -> Self {
        // 检查缓存中是否已有数据
        if let Some(cached_data) = TECHNICALS_CACHE.get(symbol) {
            info!("从缓存获取 {} 的技术指标数据", symbol);
            return TradingTechnicals {
                values: cached_data.clone(),
            };
        }

        info!("从 TradingView API 获取 {} 的技术指标数据", symbol);
        let url = format!("https://scanner.tradingview.com/symbol?symbol={}&fields=Recommend.Other,Recommend.All,Recommend.MA,RSI,RSI[1],Stoch.K,Stoch.D,Stoch.K[1],Stoch.D[1],CCI20,CCI20[1],ADX,ADX+DI,ADX-DI,ADX+DI[1],ADX-DI[1],AO,AO[1],AO[2],Mom,Mom[1],MACD.macd,MACD.signal,Rec.Stoch.RSI,Stoch.RSI.K,Rec.WR,W.R,Rec.BBPower,BBPower,Rec.UO,UO,EMA10,close,SMA10,EMA20,SMA20,EMA30,SMA30,EMA50,SMA50,EMA100,SMA100,EMA200,SMA200,Rec.Ichimoku,Ichimoku.BLine,Rec.VWMA,VWMA,Rec.HullMA9,HullMA9,Pivot.M.Classic.R3,Pivot.M.Classic.R2,Pivot.M.Classic.R1,Pivot.M.Classic.Middle,Pivot.M.Classic.S1,Pivot.M.Classic.S2,Pivot.M.Classic.S3,Pivot.M.Fibonacci.R3,Pivot.M.Fibonacci.R2,Pivot.M.Fibonacci.R1,Pivot.M.Fibonacci.Middle,Pivot.M.Fibonacci.S1,Pivot.M.Fibonacci.S2,Pivot.M.Fibonacci.S3,Pivot.M.Camarilla.R3,Pivot.M.Camarilla.R2,Pivot.M.Camarilla.R1,Pivot.M.Camarilla.Middle,Pivot.M.Camarilla.S1,Pivot.M.Camarilla.S2,Pivot.M.Camarilla.S3,Pivot.M.Woodie.R3,Pivot.M.Woodie.R2,Pivot.M.Woodie.R1,Pivot.M.Woodie.Middle,Pivot.M.Woodie.S1,Pivot.M.Woodie.S2,Pivot.M.Woodie.S3,Pivot.M.Demark.R1,Pivot.M.Demark.Middle,Pivot.M.Demark.S1&no_404=true&label-product=popup-technicals", symbol);
        
        let client = Client::new();
        let resp = match client.get(&url).send().await {
            Ok(response) => {
                match response.text().await {
                    Ok(text) => text,
                    Err(e) => {
                        warn!("读取响应正文失败: {}", e);
                        return TradingTechnicals {
                            values: Value::Null,
                        };
                    }
                }
            }
            Err(e) => {
                warn!("请求失败: {}", e);
                return TradingTechnicals {
                    values: Value::Null,
                };
            }
        };

        let v: Value = match serde_json::from_str(&resp) {
            Ok(data) => data,
            Err(e) => {
                warn!("解析JSON失败: {}", e);
                return TradingTechnicals {
                    values: Value::Null,
                };
            }
        };

        // 将数据存储到缓存中
        TECHNICALS_CACHE.insert(symbol.to_string(), v.clone());
        info!("已将 {} 的技术指标数据存储到缓存", symbol);

        TradingTechnicals {
            values: v,
        }
    }

    /// 清除指定symbol的缓存
    pub fn clear_cache(symbol: &str) {
        TECHNICALS_CACHE.invalidate(symbol);
        info!("已清除 {} 的缓存", symbol);
    }

    /// 清除所有缓存
    pub fn clear_all_cache() {
        TECHNICALS_CACHE.invalidate_all();
        info!("已清除所有技术指标缓存");
    }

    pub fn calculate(&mut self) -> (f64, f64, f64) {
        // 结构化归纳
        let summary_signal = match self.values.get("Recommend.All").and_then(|x| x.as_f64()) {
            Some(val) if val > 0.5 && val < 0.7=> 1.0,
            Some(val) if val > 0.7 => 2.0,
            Some(val) if val < -0.5 && val > -0.7 => -1.0,
            Some(val) if val < -0.7 =>-2.0,
            _ => 0.0,
        };

        let ma_signal = match self.values.get("Recommend.MA").and_then(|x| x.as_f64()) {
            Some(val) if val > 0.5 && val < 0.7=> 1.0,
            Some(val) if val > 0.7 => 2.0,
            Some(val) if val < -0.5 && val > -0.7 => -1.0,
            Some(val) if val < -0.7 =>-2.0,
            _ => 0.0,
        };

        let osc_signal = match self.values.get("Recommend.Other").and_then(|x| x.as_f64()) {
            Some(val) if val > 0.5 && val < 0.7=> 1.0,
            Some(val) if val > 0.7 => 2.0,
            Some(val) if val < -0.5 && val > -0.7 => -1.0,
            Some(val) if val < -0.7 =>-2.0,
            _ => 0.0,
        };
        (summary_signal, ma_signal, osc_signal)
    }

    /// 重置 计算器状态
    pub fn reset(&mut self) {
        self.values=Value::Null;
    }
}