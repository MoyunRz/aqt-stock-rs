use crate::calculates::base_calculate::BaseCalculate;
use crate::indicators::candle::Candle;
use crate::indicators::utbot::UTBot;

pub struct UTBotCalculate {
    pub(crate) candles: Vec<Candle>,
}

impl BaseCalculate for UTBotCalculate {
    fn calculate(&self) -> i64 {
        // 创建STC指标
        let mut ubot = UTBot::new(1.0, 10, false);
        // 不再保存返回值，直接调用计算方法
        ubot.calculate(&self.candles);
        // 获取最新状态
        if let Some(stop) = ubot.latest_stop() {
            println!("最新跟踪止损线: {:.2}", stop);
        }

        if ubot.is_long() {
            return 1
        } else if ubot.is_short() {
           return -1
        }
        // 检查最新信号
        let buy_signals = ubot.buy_signals();  // 使用方法获取信号数组
        if let Some(&buy) = buy_signals.last() {
            if buy {
                return 1
            }
        }
        let sell_signals = ubot.sell_signals();  // 使用方法获取信号数组
        if let Some(&sell) = sell_signals.last() {
            if sell {
                return -1
            }
        }
        0
    }
    fn get_name(&self) -> String {
        "UTBOT".to_string()
    }
    fn get_description(&self) -> String {
        "UTBOT指标".to_string()
    }
}
