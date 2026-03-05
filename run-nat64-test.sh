#!/bin/bash
# NAT64 测试工具 - Linux/macOS 启动脚本

echo "===================================="
echo "NAT64/DNS64 服务测试工具"
echo "===================================="
echo

# 检查 Go 是否安装
if ! command -v go &> /dev/null; then
    echo "错误: 未找到 Go 编译器"
    echo "请先安装 Go: https://golang.org/dl/"
    exit 1
fi

echo "[1/2] 编译 nat64_test.go..."
go build -o nat64-tester nat64_test.go
if [ $? -ne 0 ]; then
    echo "编译失败！"
    exit 1
fi

echo "编译成功！"
echo
echo "[2/2] 运行测试..."
echo

# 运行测试（带详细输出）
./nat64-tester -verbose -concurrency 5 -timeout 10

echo
echo "===================================="
echo "测试完成！"
echo "===================================="
