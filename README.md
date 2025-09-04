# AQT US Stock Trading System

<div align="center">

![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Build](https://img.shields.io/badge/build-passing-brightgreen.svg)
![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)

一个基于 Rust 的高性能美股量化交易系统，集成 LongPort OpenAPI，支持实时行情数据获取、技术指标分析和策略执行。

[功能特性](#功能特性) • 
[快速开始](#快速开始) • 
[架构设计](#架构设计) • 
[API 文档](#api-文档) • 
[贡献指南](#贡献指南)

</div>

## 🚀 功能特性

### 📊 数据获取
- **实时行情数据** - 通过 WebSocket 获取实时股票价格、成交量等数据
- **历史数据查询** - 支持多种时间周期的 K 线数据获取
- **市场深度数据** - 获取买卖盘口数据和交易深度信息
- **经纪商数据** - 获取经纪商买卖单信息

### 🔧 技术指标
- **MACD** - 移动平均线收敛散度指标
- **KDJ** - 随机指标
- **STC** - Schaff 趋势周期指标
- **UTBot** - 通用交易机器人指标
- **自定义指标** - 支持扩展更多技术指标

### 🎯 策略执行
- **多策略支持** - 模块化策略架构，支持多种交易策略
- **实时信号生成** - 基于技术指标生成买卖信号
- **风险管理** - 内置止盈止损机制
- **资金管理** - 灵活的仓位管理策略

### 📈 交易功能
- **下单管理** - 支持市价单、限价单等多种订单类型
- **持仓查询** - 实时查询持仓信息和盈亏状态
- **订单管理** - 订单状态跟踪和管理
- **风险控制** - 多层次风险控制机制

### 🔍 监控与日志
- **实时监控** - 系统运行状态实时监控
- **详细日志** - 分级日志记录，支持日志轮转
- **性能指标** - 交易性能和系统性能监控
- **错误追踪** - 详细的错误日志和异常处理

## 🏗️ 技术栈

### 核心技术
- **Rust 2021** - 系统编程语言，保证性能和安全性
- **Tokio** - 异步运行时，支持高并发处理
- **Serde** - 序列化和反序列化框架
- **LongPort OpenAPI** - 长桥证券 API，提供股票数据和交易接口

### 网络通信
- **WebSocket** - 实时数据订阅
- **HTTP/HTTPS** - RESTful API 调用
- **TLS** - 安全通信加密

### 数据处理
- **YAML** - 配置文件格式
- **JSON** - 数据交换格式
- **Time** - 时间处理库

### 日志与监控
- **log4rs** - 高性能日志框架
- **log** - 标准日志接口

## 🏛️ 架构设计

```
┌─────────────────────────────────────────────────────────────┐
│                     AQT Trading System                      │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐          │
│  │   Config    │  │    Tasks    │  │  Strategies │          │
│  │   配置管理   │  │   任务调度   │  │   策略引擎   │           │
│  └─────────────┘  └─────────────┘  └─────────────┘          │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐          │
│  │  Collect    │  │ Indicators  │  │  Computes   │          │
│  │  数据采集    │  │  技术指标    │  │   计算引擎   │           │
│  └─────────────┘  └─────────────┘  └─────────────┘          │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐          │
│  │  Services   │  │   Models    │  │    Utils    │          │
│  │   服务层     │  │  数据模型    │  │   工具库     │          │
│  └─────────────┘  └─────────────┘  └─────────────┘          │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
         ┌─────────────────────────────────────────┐
         │          LongPort OpenAPI               │
         │  ┌─────────────┐  ┌─────────────┐      │
         │  │ Quote API   │  │ Trade API   │      │
         │  │   行情接口   │  │   交易接口   │      │
         │  └─────────────┘  └─────────────┘      │
         └─────────────────────────────────────────┘
```

### 模块说明

- **Config** - 配置管理模块，处理系统配置和参数
- **Tasks** - 任务调度模块，管理定时任务和异步任务
- **Strategies** - 策略引擎，执行交易策略和信号生成
- **Collect** - 数据收集模块，获取实时和历史市场数据
- **Indicators** - 技术指标模块，计算各种技术分析指标
- **Computes** - 计算引擎，处理复杂的数学计算
- **Services** - 服务层，提供交易服务和账户管理
- **Models** - 数据模型，定义系统中的数据结构
- **Utils** - 工具库，提供通用工具函数

## 🚀 快速开始

### 环境要求

- **Rust** 1.70 或更高版本
- **macOS** / **Linux** / **Windows**
- **LongPort** 账户和 API 密钥

### 安装步骤

1. **克隆项目**
   ```bash
   git clone https://github.com/your-username/aqt-us-stock.git
   cd aqt-us-stock
   ```

2. **配置环境变量**
   ```bash
   # 创建 .env 文件
   cp .env.example .env
   
   # 编辑环境变量
   export LONGPORT_APP_KEY="your-app-key"
   export LONGPORT_APP_SECRET="your-app-secret"
   export LONGPORT_ACCESS_TOKEN="your-access-token"
   ```

3. **安装依赖**
   ```bash
   # 使用 Makefile（推荐）
   make install
   
   # 或使用 Cargo
   cargo build
   ```

4. **配置系统**
   ```bash
   # 编辑配置文件
   vim config.yaml
   
   # 配置日志
   vim log4rs.yaml
   ```

5. **运行系统**
   ```bash
   # 使用 Makefile
   make run
   
   # 或使用管理脚本
   ./aqt-manager.sh start
   
   # 或直接使用 Cargo
   cargo run
   ```

### 开发模式

```bash
# 文件监控模式（自动重启）
make dev

# 或
cargo watch -x run
```

## ⚙️ 配置说明

### 主配置文件 (config.yaml)

```yaml
symbols:
  - symbol: AAPL.US        # 股票代码
    volume: 0.01           # 开仓比例
    period: 15m            # K线周期
    tp_ratio: 500          # 止盈比例
    sl_ratio: 500          # 止损比例
  - symbol: NVDL.US
    volume: 0.01
    period: 15m
    tp_ratio: 10
    sl_ratio: 500
```

### 日志配置 (log4rs.yaml)

```yaml
refresh_rate: 60 seconds

appenders:
  console_appender:
    kind: console
    target: stdout
    encoder:
      pattern: "{d(%Y-%m-%d %H:%M:%S)} [{l}] {t} - {m}{n}"

  file_appender:
    kind: file
    path: "logs/application.log"
    append: true
    encoder:
      pattern: "{d} - {l} - {M} - {m}{n}"

root:
  level: info
  appenders:
    - console_appender
    - file_appender
```

### 环境变量

| 变量名 | 说明 | 示例 |
|--------|------|------|
| `LONGPORT_APP_KEY` | LongPort API Key | `your-app-key` |
| `LONGPORT_APP_SECRET` | LongPort API Secret | `your-app-secret` |
| `LONGPORT_ACCESS_TOKEN` | LongPort Access Token | `your-access-token` |
| `RUST_LOG` | 日志级别 | `debug`, `info`, `warn`, `error` |

## 📋 使用指南

### 管理脚本使用

系统提供了便捷的管理脚本 `aqt-manager.sh`：

```bash
# 启动服务
./aqt-manager.sh start [debug|release]

# 停止服务
./aqt-manager.sh stop

# 重启服务
./aqt-manager.sh restart [debug|release]

# 查看状态
./aqt-manager.sh status

# 查看日志
./aqt-manager.sh logs [lines|follow]

# 监控模式
./aqt-manager.sh monitor

# 健康检查
./aqt-manager.sh health
```

### Makefile 使用

```bash
# 查看所有命令
make help

# 编译项目
make build

# 运行测试
make test

# 代码检查
make lint

# 打包发布
make package

# 清理项目
make clean
```

### 策略开发

1. **创建新策略**
   ```rust
   use crate::strategys::strategy::Strategy;
   
   pub struct MyStrategy {
       // 策略参数
   }
   
   impl Strategy for MyStrategy {
       fn new(quote_ctx: Arc<QuoteContext>, trade_ctx: Arc<TradeContext>) -> Self {
           // 初始化策略
       }
       
       async fn execute(&mut self, event: &MarketData) -> Result<(), Box<dyn std::error::Error>> {
           // 执行策略逻辑
       }
   }
   ```

2. **注册策略**
   ```rust
   let executor = Executor::<MyStrategy>::new(quote_ctx, trade_ctx, receiver);
   ```

## 🔍 API 文档

### 数据获取 API

```rust
// 获取实时行情
let quote_ctx = QuoteContext::try_new(config).await?;
let quotes = quote_ctx.quote(&["AAPL.US"]).await?;

// 获取 K 线数据
let candles = quote_ctx.candlesticks(
    "AAPL.US",
    Period::FifteenMinute,
    100,
    AdjustType::NoAdjust,
    TradeSessions::All
).await?;
```

### 交易 API

```rust
// 下单
let trade_ctx = TradeContext::try_new(config).await?;
let order = trade_ctx.submit_order(SubmitOrderOptions::new(
    "AAPL.US",
    OrderType::LO,
    OrderSide::Buy,
    Decimal::from(100),
    TimeInForceType::Day
)).await?;

// 查询持仓
let positions = trade_ctx.stock_positions(None).await?;
```

### 技术指标 API

```rust
// MACD 指标
let mut macd = MACD::new(12, 26, 9);
let signals = macd.calculate(&candles);

// KDJ 指标
let mut kdj = KDJ::new(9, 3, 3);
let kdj_values = kdj.calculate(&candles);
```

## 🧪 测试

### 运行测试

```bash
# 运行所有测试
make test

# 运行单个测试
cargo test kdj_test

# 运行测试并显示输出
make test-verbose
```

### 测试覆盖率

```bash
# 安装覆盖率工具
cargo install cargo-tarpaulin

# 生成覆盖率报告
cargo tarpaulin --out Html
```

## 📈 性能监控

### 系统监控

```bash
# 查看系统状态
./aqt-manager.sh status

# 监控模式
./aqt-manager.sh monitor

# 查看日志
./aqt-manager.sh logs follow
```

### 性能指标

- **延迟** - 数据接收到处理完成的时间
- **吞吐量** - 每秒处理的消息数量
- **内存使用** - 系统内存占用情况
- **CPU 使用** - CPU 使用率监控

## 🔒 安全考虑

- **API 密钥管理** - 使用环境变量存储敏感信息
- **网络安全** - 所有 API 调用使用 HTTPS/WSS
- **错误处理** - 完善的错误处理和日志记录
- **资金安全** - 严格的风险控制机制

## 🛠️ 开发指南

### 代码结构

```
src/
├── main.rs              # 程序入口
├── lib.rs              # 库入口
├── config/             # 配置模块
├── tasks/              # 任务管理
├── strategys/          # 策略引擎
├── collect/            # 数据收集
├── indicators/         # 技术指标
├── calculates/         # 计算模块
├── computes/           # 计算引擎
├── services/           # 服务层
├── models/             # 数据模型
└── utils/              # 工具库
```

### 开发流程

1. **功能开发**
   ```bash
   # 创建功能分支
   git checkout -b feature/new-indicator
   
   # 开发调试
   make dev
   
   # 代码检查
   make lint
   
   # 运行测试
   make test
   ```

2. **代码提交**
   ```bash
   # 格式化代码
   make format
   
   # 最终检查
   make ci
   
   # 提交代码
   git commit -m "feat: add new indicator"
   ```

### 贡献规范

- 遵循 [Rust 编码规范](https://doc.rust-lang.org/style-guide/)
- 使用 [Conventional Commits](https://www.conventionalcommits.org/) 提交格式
- 添加必要的测试和文档
- 通过所有 CI 检查

## 🗂️ 目录结构

```
aqt-us-stock/
├── src/                    # 源代码
├── tests/                  # 测试文件
├── logs/                   # 日志文件
├── dist/                   # 打包输出
├── backups/                # 备份文件
├── config.yaml             # 主配置文件
├── log4rs.yaml             # 日志配置
├── Cargo.toml              # 项目配置
├── Cargo.lock              # 依赖锁文件
├── Makefile                # 构建脚本
├── aqt-manager.sh          # 管理脚本
├── USAGE.md                # 使用指南
├── README.md               # 项目说明
└── .env.example            # 环境变量示例
```

## 📄 许可证

本项目采用 [MIT License](LICENSE) 许可证。

## 🤝 贡献

欢迎提交 Issues 和 Pull Requests！

1. Fork 本项目
2. 创建功能分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 创建 Pull Request

## 📞 联系方式

- **项目主页**: [GitHub Repository](https://github.com/your-username/aqt-us-stock)
- **问题反馈**: [GitHub Issues](https://github.com/your-username/aqt-us-stock/issues)
- **讨论交流**: [GitHub Discussions](https://github.com/your-username/aqt-us-stock/discussions)

## 🙏 致谢

- [LongPort OpenAPI](https://open.longportapp.com/) - 提供股票数据和交易接口
- [Rust Community](https://www.rust-lang.org/community) - 优秀的开源社区
- 所有贡献者和用户的支持

---

<div align="center">
  <sub>Built with ❤️ by the AQT Team</sub>
</div>
