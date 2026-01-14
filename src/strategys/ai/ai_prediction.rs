use std::env;

use log::{error, warn};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::services::service::Service;
use crate::strategys::ai::doc::{AI_TEMP, SYSTEM_MESSAGE};
use crate::strategys::ai::indicator::build_inobj;
use crate::strategys::ai::model::{ChatContent, InObj};

#[derive(Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Deserialize)]
struct ChatChoice {
    message: ChatMessageContent,
}

#[derive(Deserialize)]
struct ChatMessageContent {
    content: String,
}

pub async fn request_ai(prompt: &str) -> Option<ChatContent> {

    let api_key = env::var("DASHSCOPE_API_KEY").unwrap_or_else(|_| "sk-841c07f088564912ac97416bc091150f".to_string());
    let model = env::var("BAILIAN_MODEL").unwrap_or_else(|_| "qwen3-max-preview".to_string());

    let api_base = "https://dashscope.aliyuncs.com/compatible-mode/v1/chat/completions".to_string();
    let req_body = ChatRequest {
        model,
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: SYSTEM_MESSAGE.to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            },
        ],
    };

    let client = Client::new();
    let resp = match client
        .post(&api_base)
        .bearer_auth(api_key)
        .json(&req_body)
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => {
            warn!("调用百炼接口失败: {}", e);
            return None;
        }
    };

    let status = resp.status();
    let text = match resp.text().await {
        Ok(t) => t,
        Err(e) => {
            warn!("读取百炼响应失败: {}", e);
            return None;
        }
    };

    if !status.is_success() {
        warn!("百炼接口返回错误状态: {} {}", status, text);
        return None;
    }

    let parsed: ChatResponse = match serde_json::from_str(&text) {
        Ok(v) => v,
        Err(e) => {
            warn!("解析百炼响应失败: {}", e);
            return None;
        }
    };

    let content = match parsed.choices.first() {
        Some(c) => c.message.content.clone(),
        None => {
            warn!("百炼响应中没有 choices");
            return None;
        }
    };

    let result: ChatContent = match serde_json::from_str(&content) {
        Ok(v) => v,
        Err(e) => {
            warn!("解析 AI 返回的 JSON 决策失败: {}", e);
            return None;
        }
    };

    Some(result)
}

fn build_prompt(symbol: &str, cul: &InObj) -> Option<String> {
    let ind4 = match &cul.ind4 {
        Some(v) => v,
        None => {
            warn!("ds_prediction 缺少 4H 指标数据");
            return None;
        }
    };
    let ind3 = match &cul.ind3 {
        Some(v) => v,
        None => {
            warn!("ds_prediction 缺少 15m 指标数据");
            return None;
        }
    };

    fn replace_first(mut s: String, from: &str, to: String) -> String {
        if let Some(pos) = s.find(from) {
            s.replace_range(pos..pos + from.len(), &to);
        }
        s
    }

    let mut qmsg = AI_TEMP.to_string();

    let replacements: Vec<(&str, String)> = vec![
        ("%s", symbol.to_string()),
        ("%s", cul.mark_px.to_string()),
        ("%+v", format!("{:?}", ind4.k_data)),
        ("%+v", format!("{:?}", ind4.macd)),
        ("%+v", format!("{:?}", ind4.rsi7)),
        ("%+v", format!("{:?}", ind4.rsi14)),
        ("%+v", format!("{:?}", ind4.ema)),
        ("%+v", format!("{:?}", ind4.sma)),
        ("%+v", format!("{:?}", ind4.atr3)),
        ("%+v", format!("{:?}", ind4.atr14)),
        ("%s", ind3.k_data.clone()),
        ("%+v", format!("{:?}", ind3.macd)),
        ("%+v", format!("{:?}", ind3.rsi7)),
        ("%+v", format!("{:?}", ind3.rsi14)),
        ("%+v", format!("{:?}", ind3.ema)),
        ("%+v", format!("{:?}", ind3.sma)),
        ("%+v", format!("{:?}", ind3.atr3)),
        ("%+v", format!("{:?}", ind3.atr14)),
    ];

    for (pat, val) in replacements {
        qmsg = replace_first(qmsg, pat, val);
    }

    Some(qmsg)
}

pub async fn ds_prediction(symbol: &str, cul: &InObj) -> Option<ChatContent> {
    let qmsg = match build_prompt(symbol, cul) {
        Some(v) => v,
        None => return None,
    };
    request_ai(&qmsg).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::strategys::ai::model::{AtrData, Indicators, KdjData, MacdData};

    #[test]
    fn test_ds_prediction_builds_prompt() {



        let ind = Indicators {
            k_data: "kline".to_string(),
            macd: vec![MacdData { dif: 1.0, dea: 0.5, macd: 0.5 }],
            kdj: vec![KdjData { rsv: 10.0, k: 20.0, d: 30.0, j: 40.0 }],
            rsi7: vec![70.0],
            rsi14: vec![50.0],
            ema: vec![1.1, 1.2],
            sma: vec![1.0, 1.1],
            vol: vec![1000.0],
            atr3: vec![AtrData { tr: 0.5, atr: 0.4 }],
            atr14: vec![AtrData { tr: 0.8, atr: 0.7 }],
        };

        let cul = InObj {
            mark_px: 100.0,
            ind3: Some(ind.clone()),
            ind4: Some(ind),
        };

        let qmsg = build_prompt("TEST", &cul).expect("prompt should be built");

        assert!(qmsg.contains("TEST"));
        assert!(qmsg.contains("现价"));
        assert!(qmsg.contains("4H趋势"));
        assert!(qmsg.contains("15m短线"));
    }

    #[tokio::test]
    async fn test_ds_prediction_missing_indicators_returns_none() {
        let cul_missing_4h = InObj {
            mark_px: 100.0,
            ind3: None,
            ind4: None,
        };

        let res = ds_prediction("TEST", &cul_missing_4h).await;
        assert!(res.is_none());
    }
}
