# AQT Stock Trading System Makefile
.PHONY: help build run clean test check format lint install release package dev logs stop

# 项目信息
PROJECT_NAME := aqt_stock
VERSION := 0.1.0
TARGET_DIR := target
RELEASE_DIR := $(TARGET_DIR)/release
DEBUG_DIR := $(TARGET_DIR)/debug
LOGS_DIR := logs
CONFIG_FILE := config.yaml
LOG_CONFIG := log4rs.yaml

# 默认目标
help: ## 显示帮助信息
	@echo "AQT Stock Trading System - Makefile"
	@echo "====================================="
	@echo "可用命令:"
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  %-15s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

# 开发相关命令
build: ## 编译项目 (debug 模式)
	@echo "🔨 编译项目..."
	cargo build

build-release: ## 编译项目 (release 模式)
	@echo "🔨 编译项目 (release 模式)..."
	cargo build --release

run: ## 运行项目 (debug 模式)
	@echo "🚀 启动项目..."
	@mkdir -p $(LOGS_DIR)
	cargo run

run-release: ## 运行项目 (release 模式)
	@echo "🚀 启动项目 (release 模式)..."
	@mkdir -p $(LOGS_DIR)
	cargo run --release

dev: ## 开发模式 (带文件监控)
	@echo "🔧 开发模式启动..."
	@mkdir -p $(LOGS_DIR)
	cargo watch -x run

# 测试相关命令
test: ## 运行测试
	@echo "🧪 运行测试..."
	cargo test

test-verbose: ## 运行测试 (详细输出)
	@echo "🧪 运行测试 (详细输出)..."
	cargo test -- --nocapture

bench: ## 运行基准测试
	@echo "⚡ 运行基准测试..."
	cargo bench

# 代码质量检查
check: ## 检查代码
	@echo "🔍 检查代码..."
	cargo check

clippy: ## Clippy 代码检查
	@echo "📎 运行 Clippy..."
	cargo clippy -- -D warnings

format: ## 格式化代码
	@echo "💄 格式化代码..."
	cargo fmt

format-check: ## 检查代码格式
	@echo "💄 检查代码格式..."
	cargo fmt --check

lint: check clippy format-check ## 运行所有代码检查

# 清理相关命令
clean: ## 清理构建产物
	@echo "🧹 清理构建产物..."
	cargo clean
	rm -rf $(LOGS_DIR)/*.log

clean-logs: ## 清理日志文件
	@echo "🧹 清理日志文件..."
	rm -rf $(LOGS_DIR)/*.log

# 安装和依赖
install: ## 安装依赖
	@echo "📦 安装依赖..."
	cargo fetch

update: ## 更新依赖
	@echo "📦 更新依赖..."
	cargo update

# 发布相关命令
release: build-release ## 构建 release 版本
	@echo "📦 构建 release 版本完成"

package: release ## 打包项目
	@echo "📦 打包项目..."
	@mkdir -p dist
	@cp $(RELEASE_DIR)/$(PROJECT_NAME) dist/
	@cp $(CONFIG_FILE) dist/ 2>/dev/null || echo "警告: 配置文件不存在"
	@cp $(LOG_CONFIG) dist/ 2>/dev/null || echo "警告: 日志配置文件不存在"
	@mkdir -p dist/logs
	@echo "📦 打包完成，文件位于 dist/ 目录"

# 文档相关
doc: ## 生成文档
	@echo "📚 生成文档..."
	cargo doc --open

doc-build: ## 构建文档 (不打开)
	@echo "📚 构建文档..."
	cargo doc

# 日志相关
logs: ## 查看日志
	@echo "📋 查看日志..."
	@if [ -f "$(LOGS_DIR)/application.log" ]; then \
		tail -f $(LOGS_DIR)/application.log; \
	else \
		echo "日志文件不存在"; \
	fi

logs-error: ## 查看错误日志
	@echo "📋 查看错误日志..."
	@if [ -f "$(LOGS_DIR)/application.log" ]; then \
		grep -i "error\|warn" $(LOGS_DIR)/application.log | tail -20; \
	else \
		echo "日志文件不存在"; \
	fi

# 进程管理
stop: ## 停止正在运行的进程
	@echo "⏹️  停止进程..."
	@pkill -f "$(PROJECT_NAME)" || echo "没有找到运行中的进程"

ps: ## 查看进程状态
	@echo "🔍 查看进程状态..."
	@ps aux | grep "$(PROJECT_NAME)" | grep -v grep || echo "没有找到运行中的进程"

# 环境检查
env-check: ## 检查环境配置
	@echo "🔧 检查环境配置..."
	@echo "Rust 版本:"
	@rustc --version
	@echo "Cargo 版本:"
	@cargo --version
	@echo "项目配置:"
	@echo "  名称: $(PROJECT_NAME)"
	@echo "  版本: $(VERSION)"
	@echo "  配置文件: $(CONFIG_FILE)"
	@echo "  日志配置: $(LOG_CONFIG)"

# 安装开发工具
setup-dev: ## 安装开发工具
	@echo "🛠️  安装开发工具..."
	cargo install cargo-watch
	cargo install cargo-edit
	rustup component add clippy
	rustup component add rustfmt

# 备份和恢复
backup: ## 备份项目
	@echo "💾 备份项目..."
	@mkdir -p backups
	@tar -czf backups/$(PROJECT_NAME)-$(shell date +%Y%m%d-%H%M%S).tar.gz \
		--exclude=target \
		--exclude=.git \
		--exclude=logs \
		--exclude=backups \
		.
	@echo "备份完成"

# 全流程命令
all: clean build test ## 完整构建流程 (清理、编译、测试)

ci: lint test build-release ## CI/CD 流程

# 快速启动
quick: ## 快速启动 (跳过检查)
	@mkdir -p $(LOGS_DIR)
	@cargo run --release 