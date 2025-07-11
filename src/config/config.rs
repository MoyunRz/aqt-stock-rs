use serde::Deserialize;
use std::fs;
use std::path::Path;

use longport::Config;
use std::error::Error;

// 新增: SymbolConfig 结构体，用于描述每个股票的配置
#[derive(Debug, Deserialize,Clone)]
pub struct SymbolConfig {
    pub symbol: String,       // 股票代码
    pub symbol_type: String, // 股票类型
    pub volume: f64,          // 开仓比例
    pub period: String,   // K线级别
    pub tp_ratio: i32,        // 止盈比例
    pub sl_ratio: i32,        // 止损比例
}

impl SymbolConfig {
    pub fn new() -> Self {
        SymbolConfig {
            symbol: "".to_string(),
            symbol_type: "".to_string(),
            volume: 0.0,
            period: "".to_string(),
            tp_ratio: 0,
            sl_ratio: 0,
        }
    }
}

/// `Configs` 结构体用于加载和解析配置文件。
#[derive(Debug, Deserialize)]
pub struct Configs {
    pub symbols: Vec<SymbolConfig>, // 股票配置列表
}

/// 加载配置文件的静态方法。
///
/// # 返回值
/// 返回一个包含配置的 `Result`，若发生错误则返回 `Box<dyn Error>`。
impl Configs {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let path = Path::new("config.yaml");
        let content = fs::read_to_string(path)?; // 读取配置文件内容
        let config: Configs = serde_yaml::from_str(&content)?; // 解析 YAML 格式的配置
        Ok(config)
    }
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String, // 数据库连接URL
}

#[derive(Debug, Deserialize)]
pub struct LoggingConfig {
    pub level: String, // 日志级别
}

pub async fn load_config() -> Result<Config, Box<dyn Error>> {
    Ok(Config::from_env()?) // 从环境变量加载长桥配置
}