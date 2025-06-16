use longport::{Config, quote::{QuoteContext, SubFlags}};
use std::sync::Arc;
use longport::quote::PushEvent;
use tokio::sync::mpsc;

/// `DepthCollectors` 结构体用于管理深度订阅的上下文和接收器。
/// 它封装了与长桥API交互的核心逻辑，包括订阅、取消订阅以及接收推送数据。
pub struct DepthCollectors {
    ctx: QuoteContext, // 深度上下文，用于处理订阅和取消订阅操作
    receiver: mpsc::UnboundedReceiver<PushEvent>, // 接收推送事件的消息接收器
    symbols: Vec<String>, // 当前关注的股票代码列表
    sub_flags: SubFlags, // 订阅标志，指定订阅的数据类型
}

impl DepthCollectors {
    /// 初始化 `DepthCollectors` 实例。
    /// 
    /// 该方法从环境变量中加载配置，并初始化 `QuoteContext` 和消息接收器。
    /// 默认订阅标志为 `SubFlags::DEPTH`。
    /// 
    /// # 参数
    /// - `symbols`: 需要订阅的股票代码列表。
    /// 
    /// # 返回值
    /// 返回一个初始化完成的 `DepthCollectors` 实例。
    pub async fn new(symbols: Vec<String>) -> Self {
        let config = Arc::new(Config::from_env().unwrap()); // 从环境变量中加载配置
        let (ctx, receiver) = QuoteContext::try_new(config).await.expect("DepthContext init err"); // 初始化深度上下文和接收器
        DepthCollectors {
            ctx,
            receiver,
            symbols,
            sub_flags: SubFlags::DEPTH, // 默认订阅深度数据
        }
    }
    
    /// 订阅指定的股票代码列表。
    /// 
    /// 该方法会调用 `QuoteContext` 的 `subscribe` 方法进行订阅，
    /// 并通过 `receiver` 接收推送的消息，打印到控制台。
    /// 
    /// # 注意
    /// 该方法会阻塞当前任务，直到接收器关闭。
    pub async fn subscribe(&mut self) {
        self.ctx.subscribe(&self.symbols, self.sub_flags, true).await.unwrap(); // 订阅深度数据
        while let Some(msg) = self.receiver.recv().await {
            println!("{:?}", msg); // 处理接收到的推送消息
        }
    }

    /// 取消订阅指定的股票代码列表。
    /// 
    /// 该方法会调用 `QuoteContext` 的 `unsubscribe` 方法取消订阅，
    /// 并通过 `receiver` 接收推送的消息，打印到控制台。
    /// 
    /// # 注意
    /// 该方法会阻塞当前任务，直到接收器关闭。
    pub async fn unsubscribe(&mut self, symbols: Vec<String>) {
        self.ctx.unsubscribe(symbols, self.sub_flags).await.unwrap(); // 取消订阅深度数据
        while let Some(msg) = self.receiver.recv().await {
            println!("{:?}", msg); // 处理接收到的推送消息
        }
    }
}