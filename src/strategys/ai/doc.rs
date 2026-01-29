pub const AI_TEMP: &str = r#"交易对:%s 现价:%s
[1D趋势]
K线:%s
MACD:%+v
RSI(7/14):%+v/%+v
EMA/SMA(5):%+v/%+v
ATR(3/14):%+v/%+v
[2H短线]
K线:%s
MACD:%+v
RSI(7/14):%+v/%+v
EMA/SMA(5):%+v/%+v
ATR(3/14):%+v/%+v

请按此JSON格式输出决策(无Markdown):
{{
 "symbol": "TSLA|NVDA|null",
 "action": "BUY|SELL|HOLD",
 "confidence": 0.0-1.0,
 "rationale": "理由<50字",
 "target": "止盈价/0",
 "stop": "止损价/0"
}}
要求:
1.仅输出JSON
2.以1D趋势为主
3.开仓必填止盈损
"#;

pub const SYSTEM_MESSAGE: &str = "你是资深量化分析师，可以根据1D和2H的K线及MACD/RSI/EMA/SMA/ATR指标分析行情，输出JSON交易决策。";
