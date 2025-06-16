# AQT US Stock

美股交易系统后端服务。

## 功能特性

- 股票行情查询
- 股票列表过滤
- 股票搜索
- RESTful API
- 实时数据更新
- 集成 LongPort OpenAPI 获取实时股票数据

## 技术栈

- Rust
- Axum (Web 框架)
- SQLx (数据库)
- Tokio (异步运行时)
- Serde (序列化)
- Tracing (日志)
- LongPort OpenAPI SDK (股票数据)

## 开发环境要求

- Rust 1.75+
- PostgreSQL 14+
- Docker (可选)
- LongPort API 访问凭证

## 快速开始

1. 克隆项目：

```bash
git clone https://github.com/your-username/aqt-us-stock.git
cd aqt-us-stock
```

2. 配置环境变量：

复制 `.env.example` 到 `.env` 并修改配置：

```bash
cp .env.example .env
```

确保填写正确的 LongPort API 凭证：
```
LONGPORT_APP_KEY=your-longport-app-key
LONGPORT_APP_SECRET=your-longport-app-secret
LONGPORT_ACCESS_TOKEN=your-longport-access-token
```

你可以从 LongPort 用户中心获取这些凭证。

3. 安装依赖：

```bash
cargo build
```

4. 运行服务：

```bash
cargo run
```

服务将在 http://127.0.0.1:8080 启动。

## API 文档

### 获取股票信息

```
GET /api/stocks/:symbol
```

参数：
- `symbol`: 股票代码，例如 `AAPL.US`, `700.HK`

### 获取股票列表

```
GET /api/stocks?min_price=100&max_price=200&min_volume=1000000
```

参数：
- `min_price`: 最低价格
- `max_price`: 最高价格
- `min_volume`: 最低成交量
- `order_by`: 排序方式 (price_asc, price_desc, volume_asc, volume_desc)
- `limit`: 返回数量限制

### 搜索股票

```
GET /api/stocks/search?query=AAPL
```

参数：
- `query`: 搜索关键词

## LongPort SDK 集成

本项目使用 LongPort OpenAPI SDK 获取实时股票数据。SDK 提供了以下功能：

- 行情查询 (QuoteContext)
- 交易操作 (TradeContext)
- 实时数据订阅

要使用 LongPort SDK，您需要：

1. 在 LongPort 用户中心获取 API 凭证
2. 在 `.env` 文件中配置凭证
3. 确保网络环境能够访问 LongPort API

更多信息，请参考 [LongPort OpenAPI 文档](https://longportapp.github.io/openapi/)。

## 项目结构

```
src/
├── config/      # 配置管理
├── handlers/    # 请求处理器
├── models/      # 数据模型
├── services/    # 业务服务
│   ├── longport_service.rs # LongPort SDK 集成
│   └── stock_service.rs    # 股票服务
└── utils/       # 工具函数
```

## 测试

运行测试：

```bash
cargo test
```

## 部署

1. 构建发布版本：

```bash
cargo build --release
```

2. 运行服务：

```bash
./target/release/aqt-us-stock
```

## 贡献指南

1. Fork 项目
2. 创建特性分支
3. 提交变更
4. 推送到分支
5. 创建 Pull Request

## 许可证

MIT License 