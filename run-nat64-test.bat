@echo off
REM NAT64 测试工具 - Windows 启动脚本

echo ====================================
echo NAT64/DNS64 服务测试工具
echo ====================================
echo.

REM 检查 Go 是否安装
where go >nul 2>nul
if %ERRORLEVEL% neq 0 (
    echo 错误: 未找到 Go 编译器
    echo 请先安装 Go: https://golang.org/dl/
    pause
    exit /b 1
)

echo [1/2] 编译 nat64_test.go...
go build -o nat64-tester.exe nat64_test.go
if %ERRORLEVEL% neq 0 (
    echo 编译失败！
    pause
    exit /b 1
)

echo 编译成功！
echo.
echo [2/2] 运行测试...
echo.

REM 运行测试（带详细输出）
nat64-tester.exe -verbose -concurrency 5 -timeout 10

echo.
echo ====================================
echo 测试完成！
echo ====================================
pause
