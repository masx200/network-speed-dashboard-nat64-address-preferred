# 自动文件检测功能说明

## 功能概述

`generate-test-report.js` 现在支持自动检测最新生成的 `connectivity_results`
文件，无需手动指定文件路径。

## 工作原理

### 自动检测逻辑

1. **文件扫描**: 扫描当前目录下所有以 `connectivity_results-` 开头、以 `.json`
   结尾的文件
2. **时间排序**: 按文件修改时间降序排序，最新的文件排在前面
3. **智能选择**: 自动选择最新的文件作为默认输入文件
4. **容错处理**: 如果没有找到符合条件的文件，回退到默认的
   `connectivity_results.json`

### 文件匹配规则

- **支持格式**: `connectivity_results-*.json`
- **示例文件名**:
  - `connectivity_results-20251212-145425.json`
  - `connectivity_results-20251211-183817.json`
  - `connectivity_results-20251210-175120.json`

## 使用方式

### 自动模式（推荐）

```bash
# 使用默认的自动检测功能
npm run start

# 或者直接调用脚本
node generate-test-report.js
```

### 手动指定文件

```bash
# 手动指定特定的文件
node generate-test-report.js -f connectivity_results-20251212-145425.json

# 或者使用相对路径
node generate-test-report.js -f ./results/connectivity_results-latest.json
```

### 禁用自动检测

```bash
# 如果有多个文件，想要使用默认的 connectivity_results.json
node generate-test-report.js -f connectivity_results.json
```

## 命令行选项

```bash
node generate-test-report.js [options]

选项:
  -f, --file <path>      测试结果文件路径 (默认: 自动检测最新文件)
  -c, --count <number>   延迟最低的记录数量 (默认: 100)
  --no-latency-section   不包含延迟最低记录部分
  --no-ip-info           不包含IP地址信息
  -o, --output <format>  输出格式 (markdown, json, both) (默认: both)
  -h, --help             显示帮助信息
```

## 输出示例

### 自动检测成功

```
找到最新的 connectivity_results 文件: connectivity_results-20251212-145425.json
HTTP/3 连接测试失败报告生成器 v2.0.0
========================================
数据文件: D:\GitHub\golang-http3-cloudflare-test-tool\connectivity_results-20251212-145425.json
延迟记录数量: 100
输出格式: both
IP信息: 启用
```

### 回退到默认文件

```
未找到 connectivity_results-*.json 文件，使用默认文件名
HTTP/3 连接测试失败报告生成器 v2.0.0
========================================
数据文件: D:\GitHub\golang-http3-cloudflare-test-tool\connectivity_results.json
```

## GitHub Actions 集成

在 `.github/workflows/test-and-pr-ipv4.yml` 中，我们使用了类似的逻辑：

```yaml
- name: Run start script
  run: |
    # 查找最新生成的 connectivity_results 文件
    LATEST_FILE=$(ls -t connectivity_results-*.json | head -n 1)
    if [ -n "$LATEST_FILE" ]; then
      echo "使用文件: $LATEST_FILE"
      node generate-test-report.js -f "$LATEST_FILE"
    else
      echo "未找到 connectivity_results-*.json 文件，使用默认文件"
      npm run start
    fi
```

现在由于 `generate-test-report.js` 内置了自动检测功能，这个工作流可以简化为：

```yaml
- name: Run start script
  run: |
    npm run start  # 自动使用最新的 connectivity_results 文件
```

## 技术实现细节

### 核心代码

```javascript
function findLatestConnectivityResultsFile() {
  try {
    // 读取当前目录下的所有文件
    const files = readdirSync(process.cwd());

    // 筛选出 connectivity_results-*.json 格式的文件
    const connectivityFiles = files
      .filter(
        (file) =>
          file.startsWith("connectivity_results-") && file.endsWith(".json"),
      )
      .map((file) => {
        const filePath = join(process.cwd(), file);
        const stats = statSync(filePath);
        return {
          name: file,
          path: filePath,
          mtime: stats.mtime,
        };
      })
      .sort((a, b) => b.mtime - a.mtime); // 按修改时间降序排序

    if (connectivityFiles.length > 0) {
      console.log(
        `找到最新的 connectivity_results 文件: ${connectivityFiles[0].name}`,
      );
      return connectivityFiles[0].path;
    } else {
      console.log("未找到 connectivity_results-*.json 文件，使用默认文件名");
      return join(__dirname, "connectivity_results.json");
    }
  } catch (error) {
    console.error("查找文件时出错:", error.message);
    return join(__dirname, "connectivity_results.json");
  }
}
```

### 路径处理

```javascript
// 处理文件路径：如果已经是绝对路径，直接使用；否则使用相对路径
let resultsFilePath;
if (options.file.startsWith("/") || /^[A-Za-z]:/.test(options.file)) {
  // 绝对路径（Linux/macOS 或 Windows）
  resultsFilePath = options.file;
} else {
  // 相对路径
  resultsFilePath = join(process.cwd(), options.file);
}
```

## 优势

1. **自动化**: 无需手动查找和指定最新文件
2. **容错性**: 提供完善的错误处理和回退机制
3. **兼容性**: 保持向后兼容，仍支持手动指定文件
4. **跨平台**: 支持 Windows、Linux 和 macOS
5. **集成友好**: 与 CI/CD 流程无缝集成

## 注意事项

1. **文件命名**: 确保测试结果文件遵循 `connectivity_results-*.json` 命名规范
2. **权限**: 确保脚本有读取当前目录文件的权限
3. **文件完整性**: 确保文件完整写入后再运行报告生成脚本
4. **性能**: 在包含大量文件的目录中，首次扫描可能需要一些时间

## 故障排除

### 问题：找不到文件

**解决方案**：

- 检查文件命名是否符合 `connectivity_results-*.json` 格式
- 确认文件确实存在于当前工作目录
- 使用 `-f` 参数手动指定文件路径

### 问题：路径错误

**解决方案**：

- 检查当前工作目录是否正确
- 使用绝对路径：`node generate-test-report.js -f /path/to/file.json`
- 检查文件权限

### 问题：使用错误的文件

**解决方案**：

- 手动指定文件：`node generate-test-report.js -f desired-file.json`
- 检查文件的修改时间：`ls -la connectivity_results-*.json`
- 删除不需要的旧文件
