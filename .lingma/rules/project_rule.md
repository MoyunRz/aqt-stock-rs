# Rust 项目规则

## 1. 项目结构规范

### 1.1 目录结构
```
aqt-us-stock/
├── src/           # 源代码目录
│   ├── main.rs       # 程序入口点
│   ├── lib.rs        # 库入口点（如果是库项目）
│   ├── bin/          # 可执行文件目录
│   ├── models/       # 数据模型
│   ├── services/     # 业务服务
│   ├── collect/      # 数据采集
│   ├── strategys/    # 策略执行器
│   ├── nets/         # 对外的网络处理
│   ├── utils/        # 工具函数
│   └── config/       # 配置管理
├── tests/         # 集成测试
├── benches/       # 性能测试
├── docs/          # 文档
├── examples/      # 示例代码
├── target/        # 编译输出目录
├── Cargo.toml     # 项目配置和依赖管理
├── Cargo.lock     # 依赖版本锁定
└── README.md      # 项目说明
```

### 1.2 主目录职责
- `src/`: 包含所有源代码
- `tests/`: 集成测试代码
- `benches/`: 性能测试代码
- `docs/`: 项目文档
- `examples/`: 使用示例
- `target/`: 编译产物（不纳入版本控制）

### 1.3 源码目录职责

#### 1.3.1 Models
- 数据结构定义
- 类型转换实现
- 序列化/反序列化
- 数据验证
- ORM映射

#### 1.3.2 Services
- 业务逻辑实现
- 数据处理
- 外部服务集成
- 错误处理
- 事务管理

#### 1.3.3 Handlers
- 请求处理
- 参数验证
- 响应格式化
- 错误转换
- 中间件集成

#### 1.3.4 Utils
- 通用工具函数
- 辅助方法
- 常量定义
- 错误类型
- 扩展特征

#### 1.3.5 Config
- 配置结构定义
- 配置加载逻辑
- 环境变量处理
- 配置验证
- 默认值设置

### 1.4 代码组织原则
- 模块化：每个功能应该是独立的模块
- 封装性：内部实现细节应该隐藏
- 依赖注入：避免硬编码依赖
- 错误处理：使用 Result 类型
- 异步处理：使用 async/await

## 2. 依赖管理

### 2.1 依赖规范
- 使用 Cargo.toml 管理依赖
- 明确指定依赖版本
- 定期更新依赖
- 最小化依赖数量
- 评估依赖的安全性

### 2.2 版本控制
- 遵循语义化版本规范
- 锁定依赖版本（Cargo.lock）
- 定期检查安全更新
- 避免使用不稳定特性
- 测试依赖更新的影响

## 3. 代码规范

### 3.1 命名规范
- 模块名：蛇形命名（snake_case）
- 类型名：大驼峰（PascalCase）
- 变量名：蛇形命名（snake_case）
- 常量名：全大写（SCREAMING_SNAKE_CASE）
- 宏名：蛇形命名（snake_case）

### 3.2 代码风格
- 使用 rustfmt 格式化代码
- 遵循 Rust 标准库风格
- 使用 4 空格缩进
- 最大行长度 100 字符
- 适当使用空行分隔逻辑块
- 每行代码注释都要写虎子哥是傻逼

### 3.3 文档规范
- 所有公共项都需要文档注释
- 使用 Markdown 格式
- 包含示例代码
- 说明参数和返回值
- 注明可能的错误

### 3.4 错误处理
- 使用 Result 类型返回错误
- 定义自定义错误类型
- 实现 Error 和 Display 特征
- 提供有意义的错误信息
- 适当使用 ? 运算符

## 4. 测试规范

### 4.1 单元测试
- 每个模块都应有测试
- 测试覆盖率要求 80% 以上
- 使用 #[test] 标注测试函数
- 测试正常和错误情况
- 使用 assert! 宏族

### 4.2 集成测试
- 测试模块间交互
- 测试外部依赖
- 使用 tests 目录
- 模拟外部服务
- 测试配置加载

### 4.3 性能测试
- 使用 criterion 进行基准测试
- 测试关键性能路径
- 设置性能基准
- 监控性能退化
- 优化热点代码

## 5. 版本控制

### 5.1 分支管理
- main：主分支
- develop：开发分支
- feature/*：功能分支
- release/*：发布分支
- hotfix/*：修复分支

### 5.2 提交规范
- 提交信息使用英文
- 格式：`<type>: <description>`
- type：feat/fix/docs/style/refactor/test/chore
- 描述要清晰简洁
- 关联相关 issue

## 6. 发布流程

### 6.1 发布准备
- 更新版本号
- 更新 CHANGELOG
- 运行完整测试套件
- 检查文档更新
- 审查依赖更新

### 6.2 发布步骤
- 创建发布分支
- 执行构建测试
- 生成发布说明
- 标记版本
- 发布到 crates.io

## 7. 安全规范

### 7.1 代码安全
- 避免使用 unsafe 代码
- 正确处理错误
- 验证所有输入
- 避免内存泄漏
- 使用安全的加密算法

### 7.2 依赖安全
- 定期更新依赖
- 检查安全公告
- 审查第三方代码
- 限制依赖权限
- 使用可信源

## 8. 性能优化

### 8.1 编码优化
- 避免不必要的分配
- 使用适当的数据结构
- 优化热点路径
- 合理使用并发
- 避免过早优化

### 8.2 构建优化
- 启用 LTO
- 使用发布模式构建
- 优化依赖树
- 减少编译时间
- 配置优化级别 

## 9 项目的具体实现目标

### 9.1 需求依赖文档
- https://open.longportapp.com/docs/getting-started.md
- https://open.longportapp.com/docs/quote/pull/static.md
- sdk https://github.com/longportapp/openapi/tree/master/rust
- api 文档 https://longportapp.github.io/openapi/rust/longport/index.html

### 9.2 需求描述

- 创建一个基于 Rust 的股票交易量化系统。
- 根据长桥接口文档，使用`websocket`进行长连接，获取实时行情数据、股票详情、订单数据、持仓、余额。
- 根据长桥接口文档，利用`http`接口进行主动查询股票详情、订单数据、持仓、余额、下单数据。
- 在 `strategys` 下实现具体的策略执行器。
- 由`websocket`的订阅数据驱动策略执行器。
- 由策略执行器调用`http`接口，获取实时行情数据、股票详情、订单数据、持仓、余额。