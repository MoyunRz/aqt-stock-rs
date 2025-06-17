# AQT Stock Trading System - 使用指南

## 概述

本项目提供了两种方式来管理和运行 AQT Stock Trading System：

1. **Makefile** - 用于开发和构建管理
2. **Shell 脚本** - 用于服务运行管理

## 1. Makefile 使用

### 查看所有可用命令
```bash
make help
```

### 常用开发命令

#### 编译和运行
```bash
# 编译项目 (debug 模式)
make build

# 编译项目 (release 模式)  
make build-release

# 运行项目 (debug 模式)
make run

# 运行项目 (release 模式)
make run-release

# 开发模式 (文件监控自动重启)
make dev
```

#### 测试相关
```bash
# 运行测试
make test

# 运行测试 (详细输出)
make test-verbose

# 运行基准测试
make bench
```

#### 代码质量检查
```bash
# 检查代码
make check

# Clippy 检查
make clippy

# 格式化代码
make format

# 运行所有检查
make lint
```

#### 清理和维护
```bash
# 清理构建产物
make clean

# 清理日志文件
make clean-logs

# 更新依赖
make update
```

#### 打包和发布
```bash
# 构建 release 版本
make release

# 打包项目 (包含配置文件)
make package

# 生成文档
make doc
```

#### 日志和监控
```bash
# 查看日志
make logs

# 查看错误日志
make logs-error

# 停止运行中的进程
make stop

# 查看进程状态
make ps
```

#### 环境管理
```bash
# 检查环境
make env-check

# 安装开发工具
make setup-dev

# 备份项目
make backup
```

#### 一键命令
```bash
# 完整构建流程 (清理、编译、测试)
make all

# CI/CD 流程
make ci

# 快速启动
make quick
```

## 2. Shell 脚本使用 (aqt-manager.sh)

### 基本用法
```bash
# 查看帮助
./aqt-manager.sh help
```

### 服务管理

#### 启动服务
```bash
# 启动服务 (debug 模式)
./aqt-manager.sh start

# 启动服务 (release 模式)
./aqt-manager.sh start release
```

#### 停止服务
```bash
# 停止服务
./aqt-manager.sh stop
```

#### 重启服务
```bash
# 重启服务 (debug 模式)
./aqt-manager.sh restart

# 重启服务 (release 模式)
./aqt-manager.sh restart release
```

#### 查看状态
```bash
# 查看运行状态
./aqt-manager.sh status
```

### 日志管理

#### 查看日志
```bash
# 查看最近50行日志
./aqt-manager.sh logs

# 查看最近100行日志
./aqt-manager.sh logs 100

# 实时查看日志
./aqt-manager.sh logs follow
```

#### 其他日志操作
```bash
# 查看错误日志
./aqt-manager.sh error-logs

# 清理日志文件
./aqt-manager.sh clean-logs
```

### 监控和健康检查

#### 监控模式
```bash
# 进入监控模式 (自动刷新状态)
./aqt-manager.sh monitor
```

#### 健康检查
```bash
# 执行健康检查
./aqt-manager.sh health
```

## 3. 推荐工作流程

### 开发流程
```bash
# 1. 开发时使用文件监控模式
make dev

# 2. 代码提交前检查
make lint
make test

# 3. 构建和测试
make all
```

### 生产部署流程
```bash
# 1. 构建 release 版本
make release

# 2. 打包项目
make package

# 3. 启动服务
./aqt-manager.sh start release

# 4. 查看状态
./aqt-manager.sh status

# 5. 监控运行
./aqt-manager.sh monitor
```

### 日常维护
```bash
# 查看服务状态
./aqt-manager.sh status

# 查看日志
./aqt-manager.sh logs follow

# 健康检查
./aqt-manager.sh health

# 重启服务
./aqt-manager.sh restart release
```

## 4. 文件说明

### 生成的文件和目录
```
.
├── aqt_stock.pid          # 进程ID文件
├── logs/                  # 日志目录
│   ├── application.log    # 应用日志
│   ├── rolling.log        # 滚动日志
│   └── stdout.log         # 标准输出日志
├── dist/                  # 打包输出目录
├── backups/               # 备份目录
└── target/                # Cargo 构建目录
```

### 配置文件
- `config.yaml` - 应用配置文件
- `log4rs.yaml` - 日志配置文件
- `Cargo.toml` - Rust 项目配置

## 5. 故障排除

### 常见问题

#### 服务启动失败
```bash
# 检查配置文件
make env-check

# 查看详细错误
./aqt-manager.sh logs

# 查看标准输出
cat logs/stdout.log
```

#### 编译错误
```bash
# 检查代码
make check

# 运行 Clippy
make clippy

# 清理后重新构建
make clean
make build
```

#### 日志文件过大
```bash
# 清理日志
make clean-logs
# 或
./aqt-manager.sh clean-logs
```

### 调试技巧

1. **使用开发模式进行调试**
   ```bash
   make dev
   ```

2. **查看实时日志**
   ```bash
   ./aqt-manager.sh logs follow
   ```

3. **监控系统状态**
   ```bash
   ./aqt-manager.sh monitor
   ```

4. **健康检查**
   ```bash
   ./aqt-manager.sh health
   ```

## 6. 高级功能

### 自动化脚本示例

#### 自动重启脚本
```bash
#!/bin/bash
# auto-restart.sh
while true; do
    if ! ./aqt-manager.sh status | grep -q "服务正在运行"; then
        echo "检测到服务停止，正在重启..."
        ./aqt-manager.sh start release
    fi
    sleep 60
done
```

#### 日志轮转脚本
```bash
#!/bin/bash
# log-rotate.sh
LOG_SIZE=$(du -m logs/application.log 2>/dev/null | cut -f1)
if [ "$LOG_SIZE" -gt 100 ]; then
    echo "日志文件过大，正在清理..."
    ./aqt-manager.sh clean-logs
fi
```

## 7. 快速参考

### Makefile 常用命令
```bash
make help          # 帮助
make dev           # 开发模式
make lint          # 代码检查
make test          # 运行测试
make package       # 打包
make clean         # 清理
```

### Shell 脚本常用命令
```bash
./aqt-manager.sh start release    # 启动
./aqt-manager.sh stop             # 停止
./aqt-manager.sh status           # 状态
./aqt-manager.sh logs follow      # 日志
./aqt-manager.sh monitor          # 监控
```

这些工具将帮助你更高效地管理和维护 AQT Stock Trading System。 