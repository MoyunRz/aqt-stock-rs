#!/bin/bash

# AQT Stock Trading System Manager
# 用于管理项目的启动、停止、状态查看等操作

set -e

# 配置变量
PROJECT_NAME="aqt_stock"
PID_FILE="$PROJECT_NAME.pid"
LOG_FILE="logs/application.log"
CONFIG_FILE="config.yaml"
LOG_CONFIG="log4rs.yaml"

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# 打印带颜色的消息
print_message() {
    local color=$1
    local message=$2
    echo -e "${color}${message}${NC}"
}

print_info() {
    print_message $BLUE "ℹ️  $1"
}

print_success() {
    print_message $GREEN "✅ $1"
}

print_warning() {
    print_message $YELLOW "⚠️  $1"
}

print_error() {
    print_message $RED "❌ $1"
}

print_header() {
    echo
    print_message $CYAN "===================================="
    print_message $CYAN "  AQT Stock Trading System Manager  "
    print_message $CYAN "===================================="
    echo
}

# 检查项目是否正在运行
is_running() {
    if [ -f "$PID_FILE" ]; then
        local pid=$(cat "$PID_FILE")
        if ps -p "$pid" > /dev/null 2>&1; then
            return 0  # running
        else
            # PID文件存在但进程不存在，清理PID文件
            rm -f "$PID_FILE"
            return 1  # not running
        fi
    else
        return 1  # not running
    fi
}

# 获取进程ID
get_pid() {
    if [ -f "$PID_FILE" ]; then
        cat "$PID_FILE"
    else
        echo ""
    fi
}

# 检查必要文件
check_requirements() {
    print_info "检查必要文件..."
    
    # 检查 Cargo.toml
    if [ ! -f "Cargo.toml" ]; then
        print_error "未找到 Cargo.toml 文件，请确保在项目根目录运行"
        exit 1
    fi
    
    # 检查配置文件
    if [ ! -f "$CONFIG_FILE" ]; then
        print_warning "配置文件 $CONFIG_FILE 不存在"
    fi
    
    # 检查日志配置文件
    if [ ! -f "$LOG_CONFIG" ]; then
        print_warning "日志配置文件 $LOG_CONFIG 不存在"
    fi
    
    # 创建日志目录
    mkdir -p logs
    
    print_success "环境检查完成"
}

# 编译项目
build_project() {
    local mode=${1:-debug}
    print_info "编译项目 ($mode 模式)..."
    
    if [ "$mode" = "release" ]; then
        cargo build --release
    else
        cargo build
    fi
    
    if [ $? -eq 0 ]; then
        print_success "编译完成"
    else
        print_error "编译失败"
        exit 1
    fi
}

# 启动项目
start() {
    local mode=${1:-debug}
    
    print_header
    print_info "启动 AQT Stock Trading System..."
    
    if is_running; then
        local pid=$(get_pid)
        print_warning "项目已在运行中 (PID: $pid)"
        return 0
    fi
    
    check_requirements
    build_project $mode
    
    print_info "启动服务..."
    
    # 根据模式选择可执行文件
    if [ "$mode" = "release" ]; then
        nohup ./target/release/$PROJECT_NAME > logs/stdout.log 2>&1 &
    else
        nohup ./target/debug/$PROJECT_NAME > logs/stdout.log 2>&1 &
    fi
    
    local pid=$!
    echo $pid > "$PID_FILE"
    
    # 等待一会儿检查进程是否成功启动
    sleep 2
    
    if is_running; then
        print_success "服务启动成功 (PID: $pid)"
        print_info "日志文件: $LOG_FILE"
        print_info "使用 '$0 logs' 查看实时日志"
        print_info "使用 '$0 status' 查看运行状态"
    else
        print_error "服务启动失败"
        if [ -f "logs/stdout.log" ]; then
            print_info "错误信息:"
            tail -10 logs/stdout.log
        fi
        exit 1
    fi
}

# 停止项目
stop() {
    print_header
    print_info "停止 AQT Stock Trading System..."
    
    if ! is_running; then
        print_warning "项目未在运行"
        return 0
    fi
    
    local pid=$(get_pid)
    print_info "正在停止进程 (PID: $pid)..."
    
    # 优雅停止
    kill $pid
    
    # 等待进程结束
    local count=0
    while is_running && [ $count -lt 10 ]; do
        sleep 1
        ((count++))
        print_info "等待进程结束... ($count/10)"
    done
    
    if is_running; then
        print_warning "优雅停止失败，强制结束进程..."
        kill -9 $pid
        sleep 1
    fi
    
    # 清理 PID 文件
    rm -f "$PID_FILE"
    
    if ! is_running; then
        print_success "服务已停止"
    else
        print_error "停止服务失败"
        exit 1
    fi
}

# 重启项目
restart() {
    local mode=${1:-debug}
    print_header
    print_info "重启 AQT Stock Trading System..."
    
    if is_running; then
        stop
        sleep 2
    fi
    
    start $mode
}

# 查看状态
status() {
    print_header
    print_info "AQT Stock Trading System 状态"
    echo
    
    if is_running; then
        local pid=$(get_pid)
        print_success "服务正在运行"
        echo "  PID: $pid"
        echo "  运行时间: $(ps -o etime= -p $pid | tr -d ' ')"
        echo "  内存使用: $(ps -o rss= -p $pid | tr -d ' ') KB"
        echo "  CPU使用: $(ps -o %cpu= -p $pid | tr -d ' ')%"
    else
        print_warning "服务未运行"
    fi
    
    echo
    print_info "日志信息"
    if [ -f "$LOG_FILE" ]; then
        echo "  日志文件: $LOG_FILE"
        echo "  文件大小: $(du -h $LOG_FILE | cut -f1)"
        echo "  最后修改: $(stat -f '%Sm' $LOG_FILE 2>/dev/null || stat -c '%y' $LOG_FILE 2>/dev/null)"
    else
        echo "  日志文件不存在"
    fi
}

# 查看日志
logs() {
    local lines=${1:-50}
    
    if [ "$1" = "follow" ] || [ "$1" = "-f" ]; then
        print_info "实时查看日志 (按 Ctrl+C 退出)..."
        if [ -f "$LOG_FILE" ]; then
            tail -f "$LOG_FILE"
        else
            print_warning "日志文件不存在"
        fi
    else
        print_info "查看最近 $lines 行日志..."
        if [ -f "$LOG_FILE" ]; then
            tail -n "$lines" "$LOG_FILE"
        else
            print_warning "日志文件不存在"
        fi
    fi
}

# 查看错误日志
error_logs() {
    print_info "查看错误日志..."
    if [ -f "$LOG_FILE" ]; then
        grep -i "error\|warn\|panic" "$LOG_FILE" | tail -20
    else
        print_warning "日志文件不存在"
    fi
}

# 清理日志
clean_logs() {
    print_info "清理日志文件..."
    rm -f logs/*.log
    print_success "日志文件已清理"
}

# 监控模式
monitor() {
    print_header
    print_info "进入监控模式 (按 Ctrl+C 退出)..."
    
    while true; do
        clear
        status
        echo
        print_info "$(date '+%Y-%m-%d %H:%M:%S') - 自动刷新中..."
        sleep 5
    done
}

# 健康检查
health_check() {
    print_info "执行健康检查..."
    
    if ! is_running; then
        print_error "服务未运行"
        return 1
    fi
    
    # 检查日志中是否有错误
    if [ -f "$LOG_FILE" ]; then
        local recent_errors=$(tail -100 "$LOG_FILE" | grep -i "error\|panic" | wc -l)
        if [ $recent_errors -gt 0 ]; then
            print_warning "发现 $recent_errors 个最近的错误"
        else
            print_success "未发现最近的错误"
        fi
    fi
    
    print_success "健康检查完成"
}

# 显示帮助信息
show_help() {
    print_header
    echo "用法: $0 [命令] [选项]"
    echo
    echo "命令:"
    echo "  start [debug|release]  启动服务 (默认 debug 模式)"
    echo "  stop                   停止服务"
    echo "  restart [debug|release] 重启服务"
    echo "  status                 查看运行状态"
    echo "  logs [行数|follow]     查看日志"
    echo "  error-logs             查看错误日志"
    echo "  clean-logs             清理日志文件"
    echo "  monitor                监控模式"
    echo "  health                 健康检查"
    echo "  help                   显示帮助信息"
    echo
    echo "示例:"
    echo "  $0 start               # 启动服务 (debug 模式)"
    echo "  $0 start release       # 启动服务 (release 模式)"
    echo "  $0 logs follow         # 实时查看日志"
    echo "  $0 logs 100            # 查看最近100行日志"
    echo "  $0 restart release     # 重启服务 (release 模式)"
    echo
}

# 主函数
main() {
    case "${1:-help}" in
        start)
            start ${2:-debug}
            ;;
        stop)
            stop
            ;;
        restart)
            restart ${2:-debug}
            ;;
        status)
            status
            ;;
        logs)
            logs ${2:-50}
            ;;
        error-logs)
            error_logs
            ;;
        clean-logs)
            clean_logs
            ;;
        monitor)
            monitor
            ;;
        health)
            health_check
            ;;
        help|--help|-h)
            show_help
            ;;
        *)
            print_error "未知命令: $1"
            echo
            show_help
            exit 1
            ;;
    esac
}

# 运行主函数
main "$@" 