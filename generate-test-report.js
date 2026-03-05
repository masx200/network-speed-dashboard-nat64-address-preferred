#!/usr/bin/env node

import {
  appendFileSync,
  existsSync,
  readdirSync,
  readFileSync,
  statSync,
  writeFileSync,
} from "fs";
import { basename, join } from "path";
import { Command } from "commander";
import IPInfoFetcher from "./ip-info.js";

/**
 * 查找最新生成的 connectivity_results 文件
 * @returns {string} 最新文件的路径，如果没有找到则返回默认文件名
 */
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

/**
 * 生成HTTP/3连接测试失败报告
 * 从connectivity_results.json中提取所有失败的测试结果并生成格式化报告
 * 新增功能：输出延迟最低的前2000条记录
 */

class TestReportGenerator {
  constructor(resultsFilePath, options = {}) {
    this.resultsFilePath = resultsFilePath;
    this.allTests = []; // 存储所有测试结果（包括成功的）
    this.failedTests = [];
    this.successfulTests = [];
    this.ipInfo = null; // 存储IP信息
    this.ipFetcher = new IPInfoFetcher(); // IP信息获取器
    this.options = {
      topLatencyCount: options.topLatencyCount || 2000,
      includeLatencySection: options.includeLatencySection !== false,
      includeIPInfo: options.includeIPInfo !== false,
      ...options,
    };
    this.statistics = {
      total: 0,
      failed: 0,
      success: 0,
      failureRate: 0,
      avgLatency: 0,
      minLatency: Infinity,
      maxLatency: 0,
    };
  }

  /**
   * 获取IP地址信息
   */
  async fetchIPInfo() {
    if (this.options.includeIPInfo) {
      try {
        console.log("正在获取当前IP地址信息...");
        this.ipInfo = await this.ipFetcher.fetchIPInfo();
        return true;
      } catch (error) {
        console.error("获取IP信息失败:", error.message);
        // 设置默认IP信息，避免报告生成失败
        this.ipInfo = {
          ip: "unknown",
          country: "unknown",
          country_code: "unknown",
          asn: "unknown",
          as_name: "unknown",
          source: "failed",
          error: error.message,
        };
        return false;
      }
    }
    return false;
  }

  /**
   * 读取并解析测试结果文件
   */
  loadResults() {
    try {
      console.log("正在读取测试结果文件...");
      const fileContent = readFileSync(this.resultsFilePath, "utf8");
      const results = JSON.parse(fileContent);

      console.log(`成功读取 ${results.length} 条测试记录`);
      return results;
    } catch (error) {
      console.error("读取测试结果文件失败:", error.message);
      process.exit(1);
    }
  }

  /**
   * 分析测试结果，提取失败的测试和成功的测试
   */
  analyzeResults(results) {
    console.log("正在分析测试结果...");

    this.statistics.total = results.length;
    let totalLatency = 0;
    let validLatencyCount = 0;

    results.forEach((result, index) => {
      const testRecord = {
        index: index + 1,
        host: result.host || "Unknown",
        target_ip: result.target_ip || "Unknown",
        ip_version: result.ip_version || "Unknown",
        protocol: result.protocol || "none",
        status_code: result.status_code,
        latency_ms: result.latency_ms || 0,
        server_header: result.server_header || "N/A",
        error_msg: result.error_msg || "No error message",
        timestamp: result.timestamp || new Date().toISOString(),
        success: result.success,
      };

      // 存储所有测试结果
      this.allTests.push(testRecord);

      if (result.success === false) {
        this.failedTests.push(testRecord);
      } else if (result.success === true) {
        this.successfulTests.push(testRecord);
        this.statistics.success++;
      }

      // 计算延迟统计
      if (
        result.latency_ms &&
        typeof result.latency_ms === "number" &&
        result.latency_ms > 0
      ) {
        totalLatency += result.latency_ms;
        validLatencyCount++;
        this.statistics.minLatency = Math.min(
          this.statistics.minLatency,
          result.latency_ms,
        );
        this.statistics.maxLatency = Math.max(
          this.statistics.maxLatency,
          result.latency_ms,
        );
      }
    });

    this.statistics.failed = this.failedTests.length;
    this.statistics.failureRate = (
      (this.statistics.failed / this.statistics.total) *
      100
    ).toFixed(2);

    this.statistics.avgLatency = validLatencyCount > 0
      ? (totalLatency / validLatencyCount).toFixed(2)
      : 0;

    console.log(
      `分析完成: 失败 ${this.statistics.failed} 条，成功 ${this.statistics.success} 条`,
    );
    console.log(
      `延迟统计: 平均 ${this.statistics.avgLatency}ms，最小 ${this.statistics.minLatency}ms，最大 ${this.statistics.maxLatency}ms`,
    );
  }

  /**
   * 获取延迟最低的前N条记录
   */
  getTopLatencyRecords(count = this.options.topLatencyCount) {
    // 过滤出有有效延迟数据的记录
    const validLatencyTests = this.allTests.filter(
      (test) =>
        test.latency_ms &&
        typeof test.latency_ms === "number" &&
        test.latency_ms > 0,
    );

    // 按延迟升序排序
    return validLatencyTests
      .sort((a, b) => a.latency_ms - b.latency_ms)
      .slice(0, count);
  }

  /**
   * 生成Markdown格式的报告
   */
  generateMarkdownReport() {
    const reportDate = new Date().toLocaleString("zh-CN");

    let report = `# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: ${reportDate}
- **数据来源**: ${basename(this.resultsFilePath)}
- **总测试数**: ${this.statistics.total}
- **失败测试数**: ${this.statistics.failed}
- **成功测试数**: ${this.statistics.success}
- **失败率**: ${this.statistics.failureRate}%
- **平均延迟**: ${this.statistics.avgLatency}ms
- **最小延迟**: ${
      this.statistics.minLatency === Infinity
        ? "N/A"
        : this.statistics.minLatency + "ms"
    }
- **最大延迟**: ${this.statistics.maxLatency}ms

`;

    // 添加IP信息部分
    if (this.options.includeIPInfo && this.ipInfo) {
      const ipInfoMarkdown = this.ipFetcher.formatAsMarkdown();
      report += ipInfoMarkdown;
    }

    report += `---

## 失败测试详情

`;

    if (this.failedTests.length === 0) {
      report += `🎉 **恭喜！所有测试都成功了！**\n\n`;
    } else {
      // 按错误类型分组统计
      const errorGroups = this.groupErrorsByType();
      const groupedTests = this.groupFailedTestsByErrorType();

      report += `### 📊 错误类型统计\n\n`;
      Object.entries(errorGroups)
        .sort(([, a], [, b]) => b - a) // 按出现次数降序排列
        .forEach(([errorType, count]) => {
          const percentage = ((count / this.failedTests.length) * 100).toFixed(
            1,
          );
          report += `- **${errorType}**: ${count} 次 (${percentage}%)\n`;
        });

      // 按错误类型详细展示失败的测试
      report += `\n### 🔍 按错误类型分类的失败测试详情\n\n`;

      Object.entries(groupedTests)
        .sort(([, a], [, b]) => b.length - a.length) // 按测试数量降序排列
        .forEach(([errorType, tests]) => {
          report += `#### ${errorType} (${tests.length} 次测试)\n\n`;
          report +=
            `| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |\n`;
          report +=
            `|------|-----------|--------|--------|------|--------|----------|--------|----------|\n`;

          tests.forEach((test) => {
            const host = test.host.length > 2000
              ? test.host.substring(0, 170) + "..."
              : test.host;
            const errorMsg = test.error_msg.length > 500
              ? test.error_msg.substring(0, 470) + "..."
              : test.error_msg;
            const serverHeader = test.server_header.length > 150
              ? test.server_header.substring(0, 120) + "..."
              : test.server_header;

            report +=
              `| ${test.index} | ${host} | ${test.target_ip} | ${test.ip_version} | ${test.protocol} | ${
                test.status_code || "N/A"
              } | ${test.latency_ms} | ${serverHeader} | ${errorMsg} |\n`;
          });

          report += `\n`;
        });

      // 错误分析总结
      report += `### 📈 错误分析总结\n\n`;

      // 按主错误类型分组进行统计
      const mainErrorTypes = {};
      Object.entries(errorGroups).forEach(([fullErrorType, count]) => {
        const mainType = fullErrorType.split(":")[0]; // 获取主错误类型
        mainErrorTypes[mainType] = (mainErrorTypes[mainType] || 0) + count;
      });

      report += `#### 主错误类型分布\n\n`;
      Object.entries(mainErrorTypes)
        .sort(([, a], [, b]) => b - a)
        .forEach(([mainType, count]) => {
          const percentage = ((count / this.failedTests.length) * 100).toFixed(
            1,
          );
          report += `- **${mainType}**: ${count} 次 (${percentage}%)\n`;
        });

      report += `\n#### 错误模式分析\n\n`;

      // 分析常见的错误模式
      const patterns = this.analyzeErrorPatterns(groupedTests);
      Object.entries(patterns).forEach(([pattern, analysis]) => {
        report += `**${pattern}**: ${analysis}\n\n`;
      });
    }

    // 添加延迟最低的记录部分
    if (this.options.includeLatencySection) {
      const topLatencyRecords = this.getTopLatencyRecords();

      if (topLatencyRecords.length > 0) {
        report += `

---

## 🚀 延迟最低的 ${topLatencyRecords.length} 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|
`;

        topLatencyRecords.forEach((test) => {
          const host = test.host.length > 2000
            ? test.host.substring(0, 170) + "..."
            : test.host;
          const serverHeader = test.server_header.length > 150
            ? test.server_header.substring(0, 120) + "..."
            : test.server_header;
          const status = test.success ? "✅ 成功" : "❌ 失败";

          report +=
            `| ${test.index} | ${host} | ${test.target_ip} | ${test.ip_version} | ${test.protocol} | ${status} | ${test.latency_ms} | ${serverHeader} |\n`;
        });

        // 延迟分布统计
        report += `\n### 延迟分布分析\n\n`;

        const latencyRanges = {
          "超快 (<50ms)": 0,
          "快 (50-100ms)": 0,
          "正常 (100-200ms)": 0,
          "慢 (200-500ms)": 0,
          "很慢 (>500ms)": 0,
        };

        topLatencyRecords.forEach((test) => {
          const latency = test.latency_ms;
          if (latency < 50) latencyRanges["超快 (<50ms)"]++;
          else if (latency < 100) latencyRanges["快 (50-100ms)"]++;
          else if (latency < 200) latencyRanges["正常 (100-200ms)"]++;
          else if (latency < 500) latencyRanges["慢 (200-500ms)"]++;
          else latencyRanges["很慢 (>500ms)"]++;
        });

        Object.entries(latencyRanges).forEach(([range, count]) => {
          report += `- **${range}**: ${count} 条记录\n`;
        });
      }
    }

    report += `

---

## 详细分析

### 按IP版本统计
`;

    // 按IP版本统计
    const ipv4Failed = this.failedTests.filter(
      (t) => t.ip_version === "IPv4",
    ).length;
    const ipv6Failed = this.failedTests.filter(
      (t) => t.ip_version === "IPv6",
    ).length;

    report += `- **IPv4 失败**: ${ipv4Failed} 次\n`;
    report += `- **IPv6 失败**: ${ipv6Failed} 次\n\n`;

    // 按协议统计
    const protocolStats = {};
    this.failedTests.forEach((test) => {
      protocolStats[test.protocol] = (protocolStats[test.protocol] || 0) + 1;
    });

    report += `### 按协议统计\n\n`;
    Object.entries(protocolStats).forEach(([protocol, count]) => {
      report += `- **${protocol}**: ${count} 次失败\n`;
    });

    report += `

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

*此报告由 HTTP/3 连接测试报告生成器自动生成*
`;

    return report;
  }

  /**
   * 按错误类型分组
   */
  groupErrorsByType() {
    const errorGroups = {};

    this.failedTests.forEach((test) => {
      let errorType = "未知错误";
      let errorSubType = null;

      if (test.error_msg) {
        const errorMsg = test.error_msg.toLowerCase();

        // 连接超时类错误
        if (
          errorMsg.includes("timeout") ||
          errorMsg.includes("超时") ||
          errorMsg.includes("deadline exceeded") ||
          errorMsg.includes("context deadline exceeded")
        ) {
          errorType = "连接超时";

          if (errorMsg.includes("i/o timeout")) {
            errorSubType = "I/O超时";
          } else if (errorMsg.includes("context")) {
            errorSubType = "上下文超时";
          } else if (errorMsg.includes("dial")) {
            errorSubType = "连接建立超时";
          } else if (errorMsg.includes("tls")) {
            errorSubType = "TLS握手超时";
          } else {
            errorSubType = "其他超时";
          }
        } // 连接被拒绝类错误
        else if (
          errorMsg.includes("connection refused") ||
          errorMsg.includes("actively refused") ||
          errorMsg.includes("connectex: no connection could be made") ||
          errorMsg.includes("connect: connection refused") ||
          errorMsg.includes("拒绝") ||
          errorMsg.includes("refused")
        ) {
          errorType = "连接被拒绝";

          if (errorMsg.includes("connectex")) {
            errorSubType = "Windows连接错误";
          } else if (errorMsg.includes("actively refused")) {
            errorSubType = "目标主动拒绝";
          } else {
            errorSubType = "通用连接拒绝";
          }
        } // DNS解析错误
        else if (
          errorMsg.includes("dns") ||
          errorMsg.includes("no such host") ||
          errorMsg.includes("name resolution") ||
          errorMsg.includes("解析") ||
          errorMsg.includes("nxdomain") ||
          errorMsg.includes("servfail")
        ) {
          errorType = "DNS解析错误";

          if (errorMsg.includes("timeout")) {
            errorSubType = "DNS超时";
          } else if (errorMsg.includes("nxdomain")) {
            errorSubType = "域名不存在";
          } else if (errorMsg.includes("servfail")) {
            errorSubType = "DNS服务器失败";
          } else {
            errorSubType = "其他DNS错误";
          }
        } // TLS/SSL错误
        else if (
          errorMsg.includes("tls") ||
          errorMsg.includes("ssl") ||
          errorMsg.includes("certificate") ||
          errorMsg.includes("certificate verify failed") ||
          errorMsg.includes("handshake") ||
          errorMsg.includes("证书") ||
          errorMsg.includes("tls handshake")
        ) {
          errorType = "TLS/SSL错误";

          if (errorMsg.includes("certificate")) {
            errorSubType = "证书错误";
          } else if (errorMsg.includes("handshake")) {
            errorSubType = "握手失败";
          } else if (errorMsg.includes("verify")) {
            errorSubType = "验证失败";
          } else {
            errorSubType = "其他TLS错误";
          }
        } // 网络不可达错误
        else if (
          errorMsg.includes("network is unreachable") ||
          errorMsg.includes("no route to host") ||
          errorMsg.includes("host unreachable") ||
          errorMsg.includes("network unreachable") ||
          errorMsg.includes("不可达") ||
          errorMsg.includes("路由")
        ) {
          errorType = "网络不可达";

          if (errorMsg.includes("host")) {
            errorSubType = "主机不可达";
          } else if (errorMsg.includes("network")) {
            errorSubType = "网络不可达";
          } else {
            errorSubType = "其他路由错误";
          }
        } // 连接重置错误
        else if (
          errorMsg.includes("connection reset") ||
          errorMsg.includes("reset by peer") ||
          errorMsg.includes("broken pipe") ||
          errorMsg.includes("连接重置") ||
          errorMsg.includes("连接中断")
        ) {
          errorType = "连接中断";

          if (errorMsg.includes("peer")) {
            errorSubType = "对端重置";
          } else if (errorMsg.includes("broken")) {
            errorSubType = "管道破裂";
          } else {
            errorSubType = "其他连接中断";
          }
        } // 协议相关错误
        else if (
          errorMsg.includes("protocol") ||
          errorMsg.includes("alpn") ||
          errorMsg.includes("http") ||
          errorMsg.includes("h2") ||
          errorMsg.includes("h3") ||
          errorMsg.includes("protocol error")
        ) {
          errorType = "协议错误";

          if (errorMsg.includes("alpn")) {
            errorSubType = "ALPN协商失败";
          } else if (errorMsg.includes("http")) {
            errorSubType = "HTTP协议错误";
          } else if (errorMsg.includes("h3") || errorMsg.includes("quic")) {
            errorSubType = "HTTP/3错误";
          } else if (errorMsg.includes("h2")) {
            errorSubType = "HTTP/2错误";
          } else {
            errorSubType = "其他协议错误";
          }
        } // 代理相关错误
        else if (
          errorMsg.includes("proxy") ||
          errorMsg.includes("socks") ||
          errorMsg.includes("代理")
        ) {
          errorType = "代理错误";
        } // 防火墙和安全软件阻止
        else if (
          errorMsg.includes("blocked") ||
          errorMsg.includes("firewall") ||
          errorMsg.includes("security") ||
          errorMsg.includes("blocked") ||
          errorMsg.includes("阻止") ||
          errorMsg.includes("防火墙")
        ) {
          errorType = "安全阻止";
        }
      } else {
        errorType = "无错误信息";
      }

      // 使用复合错误类型（主类型: 子类型）
      const finalErrorType = errorSubType
        ? `${errorType}: ${errorSubType}`
        : errorType;
      errorGroups[finalErrorType] = (errorGroups[finalErrorType] || 0) + 1;
    });

    return errorGroups;
  }

  /**
   * 按错误类型对失败测试进行分组
   */
  groupFailedTestsByErrorType() {
    const groupedTests = {};

    this.failedTests.forEach((test) => {
      let errorType = "未知错误";
      let errorSubType = null;

      if (test.error_msg) {
        const errorMsg = test.error_msg.toLowerCase();

        // 使用与上面相同的分类逻辑
        if (
          errorMsg.includes("timeout") ||
          errorMsg.includes("超时") ||
          errorMsg.includes("deadline exceeded") ||
          errorMsg.includes("context deadline exceeded")
        ) {
          errorType = "连接超时";

          if (errorMsg.includes("i/o timeout")) {
            errorSubType = "I/O超时";
          } else if (errorMsg.includes("context")) {
            errorSubType = "上下文超时";
          } else if (errorMsg.includes("dial")) {
            errorSubType = "连接建立超时";
          } else if (errorMsg.includes("tls")) {
            errorSubType = "TLS握手超时";
          } else {
            errorSubType = "其他超时";
          }
        } else if (
          errorMsg.includes("connection refused") ||
          errorMsg.includes("actively refused") ||
          errorMsg.includes("connectex: no connection could be made") ||
          errorMsg.includes("connect: connection refused") ||
          errorMsg.includes("拒绝") ||
          errorMsg.includes("refused")
        ) {
          errorType = "连接被拒绝";

          if (errorMsg.includes("connectex")) {
            errorSubType = "Windows连接错误";
          } else if (errorMsg.includes("actively refused")) {
            errorSubType = "目标主动拒绝";
          } else {
            errorSubType = "通用连接拒绝";
          }
        } else if (
          errorMsg.includes("dns") ||
          errorMsg.includes("no such host") ||
          errorMsg.includes("name resolution") ||
          errorMsg.includes("解析") ||
          errorMsg.includes("nxdomain") ||
          errorMsg.includes("servfail")
        ) {
          errorType = "DNS解析错误";

          if (errorMsg.includes("timeout")) {
            errorSubType = "DNS超时";
          } else if (errorMsg.includes("nxdomain")) {
            errorSubType = "域名不存在";
          } else if (errorMsg.includes("servfail")) {
            errorSubType = "DNS服务器失败";
          } else {
            errorSubType = "其他DNS错误";
          }
        } else if (
          errorMsg.includes("tls") ||
          errorMsg.includes("ssl") ||
          errorMsg.includes("certificate") ||
          errorMsg.includes("certificate verify failed") ||
          errorMsg.includes("handshake") ||
          errorMsg.includes("证书") ||
          errorMsg.includes("tls handshake")
        ) {
          errorType = "TLS/SSL错误";

          if (errorMsg.includes("certificate")) {
            errorSubType = "证书错误";
          } else if (errorMsg.includes("handshake")) {
            errorSubType = "握手失败";
          } else if (errorMsg.includes("verify")) {
            errorSubType = "验证失败";
          } else {
            errorSubType = "其他TLS错误";
          }
        } else if (
          errorMsg.includes("network is unreachable") ||
          errorMsg.includes("no route to host") ||
          errorMsg.includes("host unreachable") ||
          errorMsg.includes("network unreachable") ||
          errorMsg.includes("不可达") ||
          errorMsg.includes("路由")
        ) {
          errorType = "网络不可达";

          if (errorMsg.includes("host")) {
            errorSubType = "主机不可达";
          } else if (errorMsg.includes("network")) {
            errorSubType = "网络不可达";
          } else {
            errorSubType = "其他路由错误";
          }
        } else if (
          errorMsg.includes("connection reset") ||
          errorMsg.includes("reset by peer") ||
          errorMsg.includes("broken pipe") ||
          errorMsg.includes("连接重置") ||
          errorMsg.includes("连接中断")
        ) {
          errorType = "连接中断";

          if (errorMsg.includes("peer")) {
            errorSubType = "对端重置";
          } else if (errorMsg.includes("broken")) {
            errorSubType = "管道破裂";
          } else {
            errorSubType = "其他连接中断";
          }
        } else if (
          errorMsg.includes("protocol") ||
          errorMsg.includes("alpn") ||
          errorMsg.includes("http") ||
          errorMsg.includes("h2") ||
          errorMsg.includes("h3") ||
          errorMsg.includes("protocol error")
        ) {
          errorType = "协议错误";

          if (errorMsg.includes("alpn")) {
            errorSubType = "ALPN协商失败";
          } else if (errorMsg.includes("http")) {
            errorSubType = "HTTP协议错误";
          } else if (errorMsg.includes("h3") || errorMsg.includes("quic")) {
            errorSubType = "HTTP/3错误";
          } else if (errorMsg.includes("h2")) {
            errorSubType = "HTTP/2错误";
          } else {
            errorSubType = "其他协议错误";
          }
        } else if (
          errorMsg.includes("proxy") ||
          errorMsg.includes("socks") ||
          errorMsg.includes("代理")
        ) {
          errorType = "代理错误";
        } else if (
          errorMsg.includes("blocked") ||
          errorMsg.includes("firewall") ||
          errorMsg.includes("security") ||
          errorMsg.includes("blocked") ||
          errorMsg.includes("阻止") ||
          errorMsg.includes("防火墙")
        ) {
          errorType = "安全阻止";
        }
      } else {
        errorType = "无错误信息";
      }

      const finalErrorType = errorSubType
        ? `${errorType}: ${errorSubType}`
        : errorType;

      if (!groupedTests[finalErrorType]) {
        groupedTests[finalErrorType] = [];
      }
      groupedTests[finalErrorType].push(test);
    });

    return groupedTests;
  }

  /**
   * 分析错误模式
   */
  analyzeErrorPatterns(groupedTests) {
    const patterns = {};

    // 分析超时相关的模式
    const timeoutTests = groupedTests["连接超时: I/O超时"] || [];
    const connectionRefusedTests =
      groupedTests["连接被拒绝: Windows连接错误"] || [];

    if (timeoutTests.length > 0) {
      // 分析超时测试的IP模式
      const ipPatterns = {};
      timeoutTests.forEach((test) => {
        const ipPrefix = test.target_ip.split(".").slice(0, 2).join(".");
        ipPatterns[ipPrefix] = (ipPatterns[ipPrefix] || 0) + 1;
      });

      const topIpPattern = Object.entries(ipPatterns).sort(
        ([, a], [, b]) => b - a,
      )[0];

      if (topIpPattern) {
        patterns[`超时集中度分析`] =
          `共有 ${timeoutTests.length} 次超时，主要集中在IP段 ${
            topIpPattern[0]
          }（${topIpPattern[1]} 次），可能存在网络路由问题或目标服务器负载过高`;
      }
    }

    if (connectionRefusedTests.length > 0) {
      // 分析连接被拒绝的IP模式
      patterns[`连接拒绝分析`] =
        `共有 ${connectionRefusedTests.length} 次连接被拒绝，这些目标可能存在防火墙阻止、服务未运行或网络配置问题`;
    }

    // 分析协议分布
    const protocolDistribution = {};
    Object.values(groupedTests)
      .flat()
      .forEach((test) => {
        protocolDistribution[test.protocol] =
          (protocolDistribution[test.protocol] || 0) + 1;
      });

    const noProtocolCount = protocolDistribution["none"] || 0;
    if (noProtocolCount > 0) {
      patterns[`协议协商分析`] =
        `有 ${noProtocolCount} 次失败是因为协议协商失败（protocol: none），说明无法与目标建立HTTP/3或其他现代协议连接`;
    }

    // 分析IP版本分布
    const ipv4Count = Object.values(groupedTests)
      .flat()
      .filter((t) => t.ip_version === "IPv4").length;
    const ipv6Count = Object.values(groupedTests)
      .flat()
      .filter((t) => t.ip_version === "IPv6").length;

    if (ipv4Count > 0 && ipv6Count === 0) {
      patterns[`IP版本分析`] =
        `所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好`;
    } else if (ipv6Count > 0 && ipv4Count === 0) {
      patterns[`IP版本分析`] =
        `所有失败的测试都使用IPv6，IPv4连接可能更稳定或目标服务器的IPv4配置更好`;
    } else if (ipv4Count > 0 && ipv6Count > 0) {
      patterns[`IP版本分析`] =
        `IPv4失败 ${ipv4Count} 次，IPv6失败 ${ipv6Count} 次，两种协议都存在问题`;
    }

    // 分析特定的主机模式
    const hostCounts = {};
    Object.values(groupedTests)
      .flat()
      .forEach((test) => {
        hostCounts[test.host] = (hostCounts[test.host] || 0) + 1;
      });

    const problematicHosts = Object.entries(hostCounts)
      .filter(([, count]) => count > 2)
      .sort(([, a], [, b]) => b - a)
      .slice(0, 3);

    if (problematicHosts.length > 0) {
      const hostList = problematicHosts
        .map(([host, count]) => `${host} (${count}次)`)
        .join(", ");
      patterns[`问题主机分析`] =
        `以下主机出现多次失败：${hostList}，建议重点检查这些主机的网络状态和服务可用性`;
    }

    return patterns;
  }

  /**
   * 生成JSON格式的报告
   */
  generateJsonReport() {
    const topLatencyRecords = this.getTopLatencyRecords();
    const errorGroups = this.groupErrorsByType();
    const groupedTests = this.groupFailedTestsByErrorType();

    const reportData = {
      report_info: {
        generated_at: new Date().toISOString(),
        source_file: basename(this.resultsFilePath),
        total_tests: this.statistics.total,
        failed_tests: this.statistics.failed,
        success_tests: this.statistics.success,
        failure_rate: parseFloat(this.statistics.failureRate),
      },
      latency_statistics: {
        average_latency_ms: parseFloat(this.statistics.avgLatency),
        min_latency_ms: this.statistics.minLatency === Infinity
          ? null
          : this.statistics.minLatency,
        max_latency_ms: this.statistics.maxLatency,
        top_latency_count: topLatencyRecords.length,
        latency_ranges: this.getLatencyRanges(topLatencyRecords),
      },
      error_analysis: {
        error_types: errorGroups,
        grouped_tests: groupedTests,
        error_patterns: this.analyzeErrorPatterns(groupedTests),
        main_error_distribution: this.getMainErrorDistribution(errorGroups),
      },
      statistics: {
        by_ip_version: {
          ipv4: this.failedTests.filter((t) => t.ip_version === "IPv4").length,
          ipv6: this.failedTests.filter((t) => t.ip_version === "IPv6").length,
        },
        by_protocol: this.getProtocolStatistics(),
        by_error_type: errorGroups,
      },
      failed_tests: this.failedTests,
      // successful_tests: this.successfulTests,
      top_latency_records: topLatencyRecords,
    };

    // 添加IP信息到JSON报告
    if (this.options.includeIPInfo && this.ipInfo) {
      reportData.test_environment = this.ipFetcher.formatAsJSON();
    }

    return reportData;
  }

  /**
   * 获取主错误类型分布
   */
  getMainErrorDistribution(errorGroups) {
    const mainTypes = {};

    Object.entries(errorGroups).forEach(([fullErrorType, count]) => {
      const mainType = fullErrorType.split(":")[0].trim();
      mainTypes[mainType] = (mainTypes[mainType] || 0) + count;
    });

    // 计算百分比
    const total = Object.values(mainTypes).reduce(
      (sum, count) => sum + count,
      0,
    );
    const distribution = {};

    Object.entries(mainTypes)
      .sort(([, a], [, b]) => b - a)
      .forEach(([type, count]) => {
        distribution[type] = {
          count: count,
          percentage: ((count / total) * 100).toFixed(1),
        };
      });

    return distribution;
  }

  /**
   * 获取延迟范围统计
   */
  getLatencyRanges(records = null) {
    const targetRecords = records || this.allTests;
    const latencyRanges = {
      "超快 (<50ms)": 0,
      "快 (50-100ms)": 0,
      "正常 (100-200ms)": 0,
      "慢 (200-500ms)": 0,
      "很慢 (>500ms)": 0,
    };

    targetRecords.forEach((test) => {
      if (!test.latency_ms || typeof test.latency_ms !== "number") return;

      const latency = test.latency_ms;
      if (latency < 50) latencyRanges["超快 (<50ms)"]++;
      else if (latency < 100) latencyRanges["快 (50-100ms)"]++;
      else if (latency < 200) latencyRanges["正常 (100-200ms)"]++;
      else if (latency < 500) latencyRanges["慢 (200-500ms)"]++;
      else latencyRanges["很慢 (>500ms)"]++;
    });

    return latencyRanges;
  }

  /**
   * 获取协议统计信息
   */
  getProtocolStatistics() {
    const protocolStats = {};
    this.failedTests.forEach((test) => {
      protocolStats[test.protocol] = (protocolStats[test.protocol] || 0) + 1;
    });
    return protocolStats;
  }

  /**
   * 保存报告到文件
   */
  saveReport(format = "markdown") {
    const timestamp = new Date().toISOString().replace(/[:.]/g, "-");

    if (format === "markdown" || format === "both") {
      const markdownReport = this.generateMarkdownReport();
      const markdownFile = `failed-test-report-${timestamp}.md`;
      writeFileSync(markdownFile, markdownReport, "utf8");
      console.log(`Markdown报告已保存到: ${markdownFile}`);
    }

    if (format === "json" || format === "both") {
      const jsonReport = this.generateJsonReport();
      const jsonFile = `failed-test-report-${timestamp}.json`;
      writeFileSync(jsonFile, JSON.stringify(jsonReport, null, 2), "utf8");
      console.log(`JSON报告已保存到: ${jsonFile}`);
    }
  }

  /**
   * 在控制台显示简要报告
   */
  displaySummary() {
    console.log("\n" + "=".repeat(50));
    console.log("HTTP/3 连接测试报告摘要");
    console.log("=".repeat(50));
    console.log(`总测试数: ${this.statistics.total}`);
    console.log(
      `失败测试数: ${this.statistics.failed} (${this.statistics.failureRate}%)`,
    );
    console.log(`成功测试数: ${this.statistics.success}`);

    // 显示延迟统计
    if (this.statistics.avgLatency > 0) {
      console.log("\n📊 延迟统计:");
      console.log(`  平均延迟: ${this.statistics.avgLatency}ms`);
      console.log(
        `  最小延迟: ${
          this.statistics.minLatency === Infinity
            ? "N/A"
            : this.statistics.minLatency + "ms"
        }`,
      );
      console.log(`  最大延迟: ${this.statistics.maxLatency}ms`);
    }

    if (this.failedTests.length > 0) {
      console.log("\n❌ 主要失败原因:");
      const errorGroups = this.groupErrorsByType();
      Object.entries(errorGroups)
        .sort(([, a], [, b]) => b - a)
        .slice(0, 5)
        .forEach(([errorType, count]) => {
          console.log(`  - ${errorType}: ${count} 次`);
        });
    }

    // 显示最佳延迟记录
    if (this.options.includeLatencySection) {
      const topLatencyRecords = this.getTopLatencyRecords(5);
      if (topLatencyRecords.length > 0) {
        console.log("\n🚀 最佳延迟记录 (前5条):");
        topLatencyRecords.forEach((test, index) => {
          const status = test.success ? "✅" : "❌";
          const host = test.host.length > 250
            ? test.host.substring(0, 220) + "..."
            : test.host;
          console.log(
            `  ${
              index + 1
            }. ${status} ${host} - ${test.latency_ms}ms (${test.protocol})`,
          );
        });
      }
    }

    console.log("=".repeat(50));
  }
}

import { fileURLToPath } from "node:url";
import { dirname } from "node:path";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// 主执行函数
async function main() {
  // 解析命令行参数
  const program = new Command();
  // 获取默认的测试结果文件路径（查找最新的文件）
  const defaultFile = findLatestConnectivityResultsFile();

  program
    .name("generate-test-report")
    .description("HTTP/3 连接测试报告生成器")
    .version("2.0.0")
    .option("-f, --file <path>", "测试结果文件路径", defaultFile)
    .option("-c, --count <number>", "延迟最低的记录数量", "2000")
    .option("--no-latency-section", "不包含延迟最低记录部分")
    .option("--no-ip-info", "不包含IP地址信息")
    .option("-o, --output <format>", "输出格式 (markdown, json, both)", "both");

  program.parse(process.argv);
  const options = program.opts();

  // 处理文件路径：如果已经是绝对路径，直接使用；否则使用相对路径
  let resultsFilePath;
  if (options.file.startsWith("/") || /^[A-Za-z]:/.test(options.file)) {
    // 绝对路径（Linux/macOS 或 Windows）
    resultsFilePath = options.file;
  } else {
    // 相对路径
    resultsFilePath = join(process.cwd(), options.file);
  }

  console.log("HTTP/3 连接测试失败报告生成器 v2.0.0");
  console.log("=".repeat(40));
  console.log(`数据文件: ${resultsFilePath}`);
  console.log(`延迟记录数量: ${options.count}`);
  console.log(`输出格式: ${options.output}`);
  console.log(`IP信息: ${options.ipInfo !== false ? "启用" : "禁用"}`);

  // 检查文件是否存在
  if (!existsSync(resultsFilePath)) {
    console.error(`错误: 找不到测试结果文件 ${resultsFilePath}`);
    console.log("请确保 connectivity_results.json 文件存在于指定路径中");
    process.exit(1);
  }

  // 创建报告生成器
  const generatorOptions = {
    topLatencyCount: parseInt(options.count),
    includeLatencySection: options.latencySection !== false,
    includeIPInfo: options.ipInfo !== false,
  };
  const generator = new TestReportGenerator(resultsFilePath, generatorOptions);

  // 获取IP信息
  if (generatorOptions.includeIPInfo) {
    console.log("\n正在获取测试环境信息...");
    await generator.fetchIPInfo();
  }

  // 加载和分析测试结果
  const results = generator.loadResults();
  generator.analyzeResults(results);

  // 显示简要报告
  generator.displaySummary();

  // 保存报告
  generator.saveReport(options.output);

  console.log("\n✅ 报告生成完成！");

  // 显示延迟统计信息
  const topLatencyRecords = generator.getTopLatencyRecords();
  if (topLatencyRecords.length > 0) {
    console.log(
      `📊 延迟最低的 ${topLatencyRecords.length} 条记录已添加到报告中`,
    );
    console.log(`   最快延迟: ${topLatencyRecords[0].latency_ms}ms`);
    console.log(
      `   最慢延迟: ${
        topLatencyRecords[topLatencyRecords.length - 1].latency_ms
      }ms`,
    );
  }

  // 显示IP信息摘要
  if (generatorOptions.includeIPInfo && generator.ipInfo) {
    console.log("\n🌐 测试环境信息:");
    console.log(`   IP地址: ${generator.ipInfo.ip}`);
    console.log(
      `   位置: ${generator.ipInfo.country} (${generator.ipInfo.country_code})`,
    );
    console.log(
      `   网络: ${generator.ipInfo.as_name || generator.ipInfo.org || "N/A"}`,
    );
  }
}

// 如果直接运行此脚本
if (import.meta.main) {
  main();
}

export default TestReportGenerator;
