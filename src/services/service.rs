use std::sync::Arc;
use log::error;
use longport::{decimal, Decimal, Market, QuoteContext, TradeContext};
use longport::quote::{AdjustType, Candlestick, MarketTemperature, Period, TradeSessions};
use longport::trade::{AccountBalance, FundPositionChannel, FundPositionsResponse, GetHistoryOrdersOptions, GetTodayOrdersOptions, Order, OrderSide, OrderStatus, OrderType, StockPositionChannel, StockPositionsResponse, SubmitOrderOptions, SubmitOrderResponse, TimeInForceType};
use time::macros::datetime;
use time::{Duration, OffsetDateTime};

/// `Service` 结构体用于封装 `QuoteContext` 和 `TradeContext`，提供统一的服务接口。
pub struct Service {
    quote_ctx: Arc<QuoteContext>, // 引用计数的报价上下文
    trade_ctx: Arc<TradeContext>, // 引用计数的交易上下文
}

impl Service {
    /// 创建一个新的 `Service` 实例。
    ///
    /// # 参数
    /// - `ctx`: 上下文的引用计数实例。
    ///
    /// # 返回值
    /// 返回一个初始化完成的 `Service` 实例。
    pub fn new(quote_ctx: Arc<QuoteContext>, trade_ctx: Arc<TradeContext>) -> Self {
        Service { quote_ctx, trade_ctx }
    }

    /// 获取历史订单列表。
    ///
    /// # 参数
    /// - `symbol`: 股票代码。
    /// - `start_at`: 查询开始时间（可选）。
    /// - `end_at`: 查询结束时间（可选）。
    ///
    /// # 返回值
    /// 返回一个包含历史订单的向量。如果发生错误，则打印错误信息并返回空向量。
    pub async fn get_history_orders(
        &self,
        symbol: &str,
        start_at: Option<OffsetDateTime>,
        end_at: Option<OffsetDateTime>,
    ) -> Vec<Order> {
        let mut opts = GetHistoryOrdersOptions::new()
            .symbol(symbol)
            .status([OrderStatus::Filled, OrderStatus::New])
            .side(OrderSide::Buy)
            .market(Market::US);
        if let Some(start) = start_at {
            opts = opts.start_at(start); // 设置查询开始时间
        }
        if let Some(end) = end_at {
            opts = opts.end_at(end); // 设置查询结束时间
        }

        // 调用 `history_orders` 方法获取历史订单，若发生错误则打印错误信息并返回空向量。
        self.trade_ctx.history_orders(opts).await.unwrap_or_else(|e| {
            eprintln!("获取历史订单出错: {}", e); // 直接打印错误信息
            Vec::new() // 返回空的订单列表
        })
    }

    /// 获取今日订单列表。
    ///
    /// # 参数
    /// - `symbol`: 股票代码。
    ///
    /// # 返回值
    /// 返回一个包含今日订单的向量。如果发生错误，则打印错误信息并返回空向量。
    pub async fn get_today_orders(
        &self,
        symbol: &str,
    ) -> Vec<Order> {
        let opts = GetTodayOrdersOptions::new()
            .symbol(symbol)
            .status([OrderStatus::Filled, OrderStatus::New, OrderStatus::WaitToNew])
            .market(Market::US);
        let resp = self.trade_ctx.today_orders(opts).await.unwrap_or_else(|e| {
            eprintln!("获取今日订单出错: {}", e); // 直接打印错误信息
            Vec::new() // 返回空的订单列表
        });
        resp
    }

    /// 提交订单。
    ///
    /// # 参数
    /// - `symbol`: 股票代码。
    /// - `side`: 订单方向（买入或卖出）。
    /// - `quantity`: 订单数量。
    ///
    /// # 返回值
    /// 返回一个包含订单ID的响应。如果发生错误，则打印错误信息并返回一个空的订单ID。
    pub async fn submit_order(
        &self,
        symbol: String,
        side: OrderSide,
        price: Decimal,
        quantity: Decimal,
    ) -> SubmitOrderResponse {
        let mut submitted_price = price;
        if side.clone() == OrderSide::Buy{
            submitted_price = (price * decimal!(1.05)).round_dp(2);
        }
        if side.clone() == OrderSide::Sell{
            submitted_price = (price * decimal!(0.95)).round_dp(2);
        }
        println!("下单价格：{:?}", submitted_price.clone());
        // 修改点：将 `expire_time` 设置为 24 小时后（即直接下一天）
        let expire_time = OffsetDateTime::now_utc().saturating_add(Duration::days(1));
        let opts = SubmitOrderOptions::new(symbol, OrderType::LO, side, quantity, TimeInForceType::GoodTilDate)
            .submitted_price(submitted_price)
            .expire_date(expire_time.date());
        let resp = self.trade_ctx.submit_order(opts).await.unwrap_or_else(|e| {
            error!("下单出错: {}", e); // 直接打印错误信息
            SubmitOrderResponse { order_id: "".to_string() }
        });
        resp
    }

    /// 获取账户余额。
    ///
    /// # 返回值
    /// 返回一个包含账户余额的向量。如果发生错误，则打印错误信息并返回空向量。
    pub async fn account_balance(
        &self,
    ) -> Vec<AccountBalance> {
        let resp = self.trade_ctx.account_balance(None).await.unwrap_or_else(|e| {
            eprintln!("获取账户余额出错: {}", e); // 直接打印错误信息
            Vec::new() // 返回空的列表
        });
        resp
    }

    /// 取消订单。
    ///
    /// # 参数
    /// - `order_id`: 订单ID。
    ///
    /// # 返回值
    /// 如果发生错误，则打印错误信息。
    pub async fn cancel_order(
        &self,
        order_id: String,
    ) {
        let resp = self.trade_ctx.cancel_order(order_id).await.unwrap_or_else(|e| {
            eprintln!("取消订单出错: {}", e); // 直接打印错误信息
        });
        resp
    }

    /// 获取账户持仓。
    ///
    /// # 返回值
    /// 返回一个包含账户持仓的响应。如果发生错误，则打印错误信息并返回一个空的持仓列表。
    pub async fn fund_positions(
        &self,
    ) -> Vec<FundPositionChannel> {
        let resp = self.trade_ctx.fund_positions(None).await.unwrap_or_else(|e| {
            eprintln!("获取账户持仓出错: {}", e); // 直接打印错误信息
            FundPositionsResponse { channels: Vec::new() }
        });
        if resp.channels.is_empty() {
            return Vec::new();
        }
        resp.channels
    }

    /// 获取账户持仓。
    ///
    /// # 返回值
    /// 返回一个包含账户持仓的响应。如果发生错误，则打印错误信息并返回一个空的持仓列表。
    pub async fn stock_positions(
        &self,
    ) -> Vec<StockPositionChannel> {
        let resp = self.trade_ctx.stock_positions(None).await.unwrap_or_else(|e| {
            eprintln!("获取账户持仓出错: {}", e); // 直接打印错误信息
            StockPositionsResponse { channels: Vec::new() }
        });
        if resp.channels.is_empty() {
            return Vec::new();
        }
        resp.channels
    }

    /// 获取行情数据
    ///
    /// 返回值：Vec<Candlestick>
    /// 返回股票的K线数据集合
    pub async fn get_candlesticks(
        &self,
        symbol: String,
        period: String,
    ) -> Vec<Candlestick> {
        let adjust_type = AdjustType::NoAdjust;
        let trade_sessions = TradeSessions::All;
        let count = 365;
        let pd;
        match period.as_str() {
            "1d" => pd = Period::Day,
            "1w" => pd = Period::Week,
            "15m" => pd = Period::FifteenMinute,
            "2h" => pd = Period::TwoHour,
            "3h" => pd = Period::ThreeHour,
            "4h" => pd = Period::FourHour,
            "5m" => pd = Period::FiveMinute,
            _ => pd = Period::UnknownPeriod
        }
        let resp = self.quote_ctx.candlesticks(symbol, pd, count, adjust_type, trade_sessions).await.unwrap_or_else(|e| {
            eprintln!("获取行情数据出错: {}", e); // 直接打印错误信息
            Vec::new() // 返回空的订单列表
        });
        resp
    }

    pub async fn get_market_temperature(
        &self,
    ) -> MarketTemperature {
        let resp = self.quote_ctx.market_temperature(Market::US).await.unwrap_or_else(|e| {
            eprintln!("获取行情数据出错: {}", e); // 直接打印错误信息
            MarketTemperature {
                temperature: 0,
                description: "".to_string(),
                valuation: 0,
                sentiment: 0,
                timestamp: datetime!(2024-01-01 12:59:59.5 -5),
            }
        });
        resp
    }
}