#!/bin/bash

# ==============================================================================
# pastebinit-rs 安装与卸载脚本
# 使用方法:
#   安装: sudo bash install.sh
#   卸载: sudo bash install.sh uninstall
# ==============================================================================

# 定义颜色常量
readonly C_RESET='\033[0m'
readonly C_RED='\033[0;31m'
readonly C_GREEN='\033[0;32m'
readonly C_YELLOW='\033[0;33m'
readonly C_BLUE='\033[0;34m'

# 定义消息输出函数
info() {
    echo -e "${C_BLUE}[INFO]${C_RESET} $1"
}

success() {
    echo -e "${C_GREEN}[SUCCESS]${C_RESET} $1"
}

warn() {
    echo -e "${C_YELLOW}[WARNING]${C_RESET} $1"
}

error() {
    echo -e "${C_RED}[ERROR]${C_RESET} $1"
}

# --- 核心变量 ---
readonly BASE_URL="https://ghfast.top/https://github.com/GenshinMinecraft/pastebinit-rs/releases/download/latest"
readonly BIN_PATH="/usr/bin/pastebinit-rs"
readonly LINK_PATH="/usr/bin/pastebinit"

# --- 卸载函数 ---
uninstall() {
    info "开始卸载 pastebinit-rs..."
    if [ -f "$BIN_PATH" ]; then
        rm -f "$BIN_PATH"
        info "已删除二进制文件: $BIN_PATH"
    else
        warn "二进制文件不存在: $BIN_PATH"
    fi

    if [ -L "$LINK_PATH" ]; then
        rm -f "$LINK_PATH"
        info "已删除符号链接: $LINK_PATH"
    else
        warn "符号链接不存在: $LINK_PATH"
    fi
    success "卸载完成！"
    exit 0
}

# --- 主逻辑开始 ---

# 1. 检查 root 权限
if [ "$EUID" -ne 0 ]; then
    error "此脚本需要以 root 权限运行。请使用 'sudo' 执行。"
    exit 1
fi

# 2. 检查是否为卸载模式
if [ "$1" == "uninstall" ]; then
    uninstall
fi

info "开始安装 pastebinit-rs..."

# 3. 检查并安装 wget
PACKAGE_MANAGER=""
if command -v apt-get &> /dev/null; then
    PACKAGE_MANAGER="apt-get"
elif command -v dnf &> /dev/null; then
    PACKAGE_MANAGER="dnf"
elif command -v yum &> /dev/null; then
    PACKAGE_MANAGER="yum"
elif command -v pacman &> /dev/null; then
    PACKAGE_MANAGER="pacman"
elif command -v zypper &> /dev/null; then
    PACKAGE_MANAGER="zypper"
fi

if ! command -v wget &> /dev/null; then
    if [ -z "$PACKAGE_MANAGER" ]; then
        error "未找到 wget，且无法确定包管理器。"
        error "请手动安装 wget 后重试。"
        exit 1
    fi

    info "未找到 wget，正在尝试使用 $PACKAGE_MANAGER 进行安装..."
    case "$PACKAGE_MANAGER" in
        "apt-get")
            apt-get update -y && apt-get install -y wget
            ;;
        "dnf" | "yum")
            $PACKAGE_MANAGER install -y wget
            ;;
        "pacman")
            pacman -Sy --noconfirm wget
            ;;
        "zypper")
            zypper install -y wget
            ;;
    esac

    if ! command -v wget &> /dev/null; then
        error "wget 安装失败。请手动安装后重试。"
        exit 1
    fi
    success "wget 安装成功。"
fi

# 4. 检测系统架构
ARCH=$(uname -m)
ARCH_SUFFIX=""

case $ARCH in
    "x86_64")
        ARCH_SUFFIX="x86_64-musl"
        ;;
    "aarch64")
        ARCH_SUFFIX="aarch64-musl"
        ;;
    "armv7l")
        # 适用于大多数32位ARM设备，如树莓派2/3/4 (32位模式)
        ARCH_SUFFIX="armv7-musleabihf"
        ;;
    "armv5tel")
        ARCH_SUFFIX="armv5te-musleabi"
        ;;
    "i686" | "i386")
        ARCH_SUFFIX="i686-musl"
        ;;
    *)
        error "不支持的系统架构: $ARCH"
        info "您可以访问 ${BASE_URL/download\/latest/}releases 查看所有支持的架构。"
        exit 1
        ;;
esac

info "检测到系统架构: $ARCH, 将下载版本: $ARCH_SUFFIX"

# 5. 下载二进制文件
DOWNLOAD_URL="${BASE_URL}/pastebinit-rs-linux-${ARCH_SUFFIX}"
info "正在从 $DOWNLOAD_URL 下载..."

if ! wget -O "$BIN_PATH" "$DOWNLOAD_URL"; then
    error "下载失败！请检查网络连接或确认URL是否有效。"
    exit 1
fi

success "下载成功！文件已保存到 $BIN_PATH"

# 6. 设置权限并创建符号链接
info "正在设置执行权限..."
chmod +x "$BIN_PATH"

info "正在创建符号链接 $LINK_PATH -> $BIN_PATH..."
ln -sf "$BIN_PATH" "$LINK_PATH"

# 7. 验证安装
info "正在验证安装..."
if command -v pastebinit-rs &> /dev/null; then
    pastebinit-rs --help
    success "pastebinit-rs 安装成功！"
    info "现在您可以使用 'pastebinit' 或 'pastebinit-rs' 命令了。"
else
    error "安装验证失败！请检查 /usr/bin 是否在您的 PATH 环境变量中。"
    exit 1
fi

exit 0