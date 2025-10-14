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

/// Trading View Technicals 指标结构体
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
        let url = format!("https://scanner.tradingview.com/symbol?symbol={}&fields=Recommend.Other|120,Recommend.All|120,Recommend.MA|120,RSI|120,RSI[1]|120,Stoch.K|120,Stoch.D|120,Stoch.K[1]|120,Stoch.D[1]|120,CCI20|120,CCI20[1]|120,ADX|120,ADX+DI|120,ADX-DI|120,ADX+DI[1]|120,ADX-DI[1]|120,AO|120,AO[1]|120,AO[2]|120,Mom|120,Mom[1]|120,MACD.macd|120,MACD.signal|120,Rec.Stoch.RSI|120,Stoch.RSI.K|120,Rec.WR|120,W.R|120,Rec.BBPower|120,BBPower|120,Rec.UO|120,UO|120,EMA10|120,close|120,SMA10|120,EMA20|120,SMA20|120,EMA30|120,SMA30|120,EMA50|120,SMA50|120,EMA100|120,SMA100|120,EMA200|120,SMA200|120,Rec.Ichimoku|120,Ichimoku.BLine|120,Rec.VWMA|120,VWMA|120,Rec.HullMA9|120,HullMA9|120,Pivot.M.Classic.R3|120,Pivot.M.Classic.R2|120,Pivot.M.Classic.R1|120,Pivot.M.Classic.Middle|120,Pivot.M.Classic.S1|120,Pivot.M.Classic.S2|120,Pivot.M.Classic.S3|120,Pivot.M.Fibonacci.R3|120,Pivot.M.Fibonacci.R2|120,Pivot.M.Fibonacci.R1|120,Pivot.M.Fibonacci.Middle|120,Pivot.M.Fibonacci.S1|120,Pivot.M.Fibonacci.S2|120,Pivot.M.Fibonacci.S3|120,Pivot.M.Camarilla.R3|120,Pivot.M.Camarilla.R2|120,Pivot.M.Camarilla.R1|120,Pivot.M.Camarilla.Middle|120,Pivot.M.Camarilla.S1|120,Pivot.M.Camarilla.S2|120,Pivot.M.Camarilla.S3|120,Pivot.M.Woodie.R3|120,Pivot.M.Woodie.R2|120,Pivot.M.Woodie.R1|120,Pivot.M.Woodie.Middle|120,Pivot.M.Woodie.S1|120,Pivot.M.Woodie.S2|120,Pivot.M.Woodie.S3|120,Pivot.M.Demark.R1|120,Pivot.M.Demark.Middle|120,Pivot.M.Demark.S1|120&no_404=true&label-product=popup-technicals", symbol);
        
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
        // info!("已将 {} 的技术指标数据存储到缓存", symbol);
        // info!("{} 的技术指标数据: {:?}", symbol, v.clone());
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

    pub fn calculate(&self) -> (f64, f64, f64) {
        // 如果为空的话，返回0
        if self.values.is_null() {
            return (0.0, 0.0, 0.0);
        }
        // 结构化归纳
        let summary_signal = match self.values.get("Recommend.All").and_then(|x| x.as_f64()) {
            Some(val) if val > 0.25 && val < 0.6=> 1.0,
            Some(val) if val > 0.6 => 2.0,
            Some(val) if val < -0.25 && val > -0.6 => -1.0,
            Some(val) if val < -0.6 =>-2.0,
            _ => 0.0,
        };

        let ma_signal = match self.values.get("Recommend.MA").and_then(|x| x.as_f64()) {
            Some(val) if val > 0.25 && val < 0.6=> 1.0,
            Some(val) if val > 0.6 => 2.0,
            Some(val) if val < -0.25 && val > -0.6 => -1.0,
            Some(val) if val < -0.6 =>-2.0,
            _ => 0.0,
        };

        let osc_signal = match self.values.get("Recommend.Other").and_then(|x| x.as_f64()) {
            Some(val) if val > 0.25 && val < 0.6=> 1.0,
            Some(val) if val > 0.6 => 2.0,
            Some(val) if val < -0.25 && val > -0.6 => -1.0,
            Some(val) if val < -0.6 =>-2.0,
            _ => 0.0,
        };
        (summary_signal, ma_signal, osc_signal)
    }

    /// 重置 计算器状态
    pub fn reset(&mut self) {
        self.values=Value::Null;
    }
}