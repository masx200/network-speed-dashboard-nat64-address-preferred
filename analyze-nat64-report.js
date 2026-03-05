#!/usr/bin/env node

import {
  existsSync,
  readdirSync,
  readFileSync,
  statSync,
  writeFileSync,
} from "fs";
import { basename, join } from "path";
import { Command } from "commander";

/**
 * 查找最新生成的 nat64-test-report 文件
 * @returns {string} 最新文件的路径，如果没有找到则返回默认文件名
 */
function findLatestNAT64ReportFile() {
  try {
    const files = readdirSync(process.cwd());

    // 筛选出 nat64-test-report-*.json 格式的文件
    const nat64Files = files
      .filter(
        (file) =>
          file.startsWith("nat64-test-report-") && file.endsWith(".json"),
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
      .sort((a, b) => b.mtime - a.mtime);

    if (nat64Files.length > 0) {
      console.log(
        `找到最新的 NAT64 测试报告文件: ${nat64Files[0].name}`,
      );
      return nat64Files[0].path;
    } else {
      console.log("未找到 nat64-test-report-*.json 文件");
      return null;
    }
  } catch (error) {
    console.error("查找文件时出错:", error.message);
    return null;
  }
}

/**
 * NAT64 测试报告分析器
 */
class NAT64ReportAnalyzer {
  constructor(reportFilePath) {
    this.reportFilePath = reportFilePath;
    this.reportData = null;
    this.analysisResult = null;
  }

  /**
   * 读取并解析测试报告文件
   */
  loadReport() {
    try {
      console.log("正在读取 NAT64 测试报告...");
      const fileContent = readFileSync(this.reportFilePath, "utf8");
      this.reportData = JSON.parse(fileContent);

      if (!this.reportData.test_results) {
        throw new Error("报告文件格式错误：缺少 test_results 字段");
      }

      console.log(
        `成功读取 ${this.reportData.test_results.length} 条测试记录`,
      );
      return true;
    } catch (error) {
      console.error("读取测试报告文件失败:", error.message);
      return false;
    }
  }

  /**
   * 分析测试报告
   */
  analyze() {
    console.log("正在分析测试数据...");

    const testResults = this.reportData.test_results;

    // 按服务提供商和前缀分组
    const serviceGroups = {};

    // 协议统计
    const protocolStats = {
      h3: { total: 0, success: 0, failed: 0, latencies: [] },
      h2: { total: 0, success: 0, failed: 0, latencies: [] },
      unknown: { total: 0, success: 0, failed: 0, latencies: [] },
    };

    // 总体统计
    const overallStats = {
      total_tests: testResults.length,
      successful_tests: 0,
      failed_tests: 0,
      dns64_success: 0,
      dns64_failed: 0,
    };

    // 按目标主机统计
    const targetHostStats = {};

    testResults.forEach((result) => {
      const key = `${result.service_provider}|${result.nat64_prefix}`;

      if (!serviceGroups[key]) {
        serviceGroups[key] = {
          service_provider: result.service_provider,
          service_location: result.service_location,
          nat64_prefix: result.nat64_prefix,
          total_tests: 0,
          successful_tests: 0,
          failed_tests: 0,
          dns64_successful: 0,
          dns64_failed: 0,
          protocols: {
            h3: { total: 0, success: 0, failed: 0, latencies: [] },
            h2: { total: 0, success: 0, failed: 0, latencies: [] },
            unknown: { total: 0, success: 0, failed: 0, latencies: [] },
          },
          targets: {},
          avg_dns_latency: 0,
          avg_connect_latency: 0,
          dns_latencies: [],
          connect_latencies: [],
        };
      }

      const group = serviceGroups[key];

      // 总体统计
      group.total_tests++;
      if (result.connect_success) {
        group.successful_tests++;
        overallStats.successful_tests++;
      } else {
        group.failed_tests++;
        overallStats.failed_tests++;
      }

      // DNS64 统计
      if (result.dns64_success) {
        group.dns64_successful++;
        overallStats.dns64_success++;
      } else {
        group.dns64_failed++;
        overallStats.dns64_failed++;
      }

      // 延迟统计
      if (result.dns64_latency_ms) {
        group.dns_latencies.push(result.dns64_latency_ms);
      }
      if (result.connect_success && result.connect_latency_ms) {
        group.connect_latencies.push(result.connect_latency_ms);
      }

      // 协议统计
      const protocol = result.protocol || "unknown";
      if (group.protocols[protocol]) {
        group.protocols[protocol].total++;
        if (result.connect_success) {
          group.protocols[protocol].success++;
          if (result.connect_latency_ms) {
            group.protocols[protocol].latencies.push(result.connect_latency_ms);
          }
        } else {
          group.protocols[protocol].failed++;
        }
      }

      // 全局协议统计
      if (protocolStats[protocol]) {
        protocolStats[protocol].total++;
        if (result.connect_success) {
          protocolStats[protocol].success++;
          if (result.connect_latency_ms) {
            protocolStats[protocol].latencies.push(result.connect_latency_ms);
          }
        } else {
          protocolStats[protocol].failed++;
        }
      }

      // 目标主机统计
      const targetKey = result.target_host + (result.target_path || "");
      if (!group.targets[targetKey]) {
        group.targets[targetKey] = {
          host: result.target_host,
          path: result.target_path || "",
          total: 0,
          success: 0,
          failed: 0,
          protocols: {},
          latencies: [],
        };
      }
      group.targets[targetKey].total++;
      if (result.connect_success) {
        group.targets[targetKey].success++;
        if (result.connect_latency_ms) {
          group.targets[targetKey].latencies.push(result.connect_latency_ms);
        }
      } else {
        group.targets[targetKey].failed++;
      }
      if (protocol) {
        group.targets[targetKey].protocols[protocol] =
          (group.targets[targetKey].protocols[protocol] || 0) + 1;
      }

      // 全局目标主机统计
      if (!targetHostStats[targetKey]) {
        targetHostStats[targetKey] = {
          host: result.target_host,
          path: result.target_path || "",
          total: 0,
          success: 0,
          failed: 0,
          avg_latency: 0,
          latencies: [],
        };
      }
      targetHostStats[targetKey].total++;
      if (result.connect_success) {
        targetHostStats[targetKey].success++;
        if (result.connect_latency_ms) {
          targetHostStats[targetKey].latencies.push(result.connect_latency_ms);
        }
      } else {
        targetHostStats[targetKey].failed++;
      }
    });

    // 计算平均延迟和成功率
    Object.values(serviceGroups).forEach((group) => {
      // DNS 平均延迟
      if (group.dns_latencies.length > 0) {
        group.avg_dns_latency = group.dns_latencies.reduce((a, b) => a + b, 0) /
          group.dns_latencies.length;
      }

      // 连接平均延迟
      if (group.connect_latencies.length > 0) {
        group.avg_connect_latency = group.connect_latencies.reduce((a, b) =>
          a + b, 0) / group.connect_latencies.length;
      }

      // 协议平均延迟
      Object.keys(group.protocols).forEach((protocol) => {
        const proto = group.protocols[protocol];
        if (proto.latencies.length > 0) {
          proto.avg_latency = proto.latencies.reduce((a, b) => a + b, 0) /
            proto.latencies.length;
          proto.min_latency = Math.min(...proto.latencies);
          proto.max_latency = Math.max(...proto.latencies);
        }
      });

      // 协议成功率
      Object.keys(group.protocols).forEach((protocol) => {
        const proto = group.protocols[protocol];
        proto.success_rate = proto.total > 0
          ? (proto.success / proto.total * 100).toFixed(2)
          : "0.00";
      });

      // 目标主机平均延迟
      Object.values(group.targets).forEach((target) => {
        if (target.latencies.length > 0) {
          target.avg_latency = target.latencies.reduce((a, b) => a + b, 0) /
            target.latencies.length;
        }
      });
    });

    // 全局协议统计
    Object.keys(protocolStats).forEach((protocol) => {
      const proto = protocolStats[protocol];
      if (proto.latencies.length > 0) {
        proto.avg_latency = proto.latencies.reduce((a, b) => a + b, 0) /
          proto.latencies.length;
        proto.min_latency = Math.min(...proto.latencies);
        proto.max_latency = Math.max(...proto.latencies);
      }
      proto.success_rate = proto.total > 0
        ? (proto.success / proto.total * 100).toFixed(2)
        : "0.00";
    });

    // 全局目标主机统计
    Object.values(targetHostStats).forEach((target) => {
      if (target.latencies.length > 0) {
        target.avg_latency = target.latencies.reduce((a, b) => a + b, 0) /
          target.latencies.length;
      }
      target.success_rate = target.total > 0
        ? (target.success / target.total * 100).toFixed(2)
        : "0.00";
    });

    // 排序服务组（按成功率降序）
    const sortedServiceGroups = Object.values(serviceGroups).sort((a, b) => {
      const rateA = (a.successful_tests / a.total_tests) * 100;
      const rateB = (b.successful_tests / b.total_tests) * 100;
      return rateB - rateA;
    });

    this.analysisResult = {
      report_info: {
        generated_at: new Date().toISOString(),
        source_file: basename(this.reportFilePath),
        original_report_time: this.reportData.report_info?.generated_at,
      },
      overall_statistics: {
        total_tests: overallStats.total_tests,
        successful_tests: overallStats.successful_tests,
        failed_tests: overallStats.failed_tests,
        success_rate: (
          (overallStats.successful_tests / overallStats.total_tests) *
          100
        ).toFixed(2) + "%",
        dns64_success: overallStats.dns64_success,
        dns64_failed: overallStats.dns64_failed,
        dns64_success_rate: this.reportData.summary?.dns64_success_rate,
      },
      protocol_statistics: protocolStats,
      service_analysis: sortedServiceGroups.map((group) => ({
        ...group,
        success_rate: (
          (group.successful_tests / group.total_tests) *
          100
        ).toFixed(2) + "%",
        http3_available: group.protocols.h3.total > 0,
        http3_success_rate: group.protocols.h3.success_rate,
        http2_available: group.protocols.h2.total > 0,
        http2_success_rate: group.protocols.h2.success_rate,
        recommended_protocol: this.getRecommendedProtocol(group),
      })),
      target_host_statistics: targetHostStats,
      recommendations: this.generateRecommendations(
        sortedServiceGroups,
        protocolStats,
      ),
    };

    console.log("分析完成！");
    return this.analysisResult;
  }

  /**
   * 获取推荐协议
   */
  getRecommendedProtocol(group) {
    const h3Rate = parseFloat(group.protocols.h3.success_rate) || 0;
    const h2Rate = parseFloat(group.protocols.h2.success_rate) || 0;

    if (h3Rate > h2Rate && h3Rate > 50) {
      return { protocol: "h3", reason: "HTTP/3 成功率更高" };
    } else if (h2Rate > h3Rate && h2Rate > 50) {
      return { protocol: "h2", reason: "HTTP/2 成功率更高" };
    } else if (h3Rate === h2Rate && h3Rate > 0) {
      return {
        protocol: "h3",
        reason: "HTTP/3 和 HTTP/2 成功率相同，优先使用 HTTP/3",
      };
    } else if (
      group.protocols.h3.total === 0 && group.protocols.h2.total === 0
    ) {
      return { protocol: "none", reason: "无可用协议数据" };
    } else {
      return { protocol: "h2", reason: "HTTP/2 作为稳定备选" };
    }
  }

  /**
   * 生成推荐建议
   */
  generateRecommendations(serviceGroups, protocolStats) {
    const recommendations = [];

    // 分析最佳服务
    const bestServices = serviceGroups
      .filter((s) => s.successful_tests > 0)
      .slice(0, 3);

    if (bestServices.length > 0) {
      recommendations.push({
        type: "best_services",
        title: "推荐的 NAT64 服务",
        items: bestServices.map((s) => ({
          provider: s.service_provider,
          location: s.service_location,
          prefix: s.nat64_prefix,
          success_rate:
            ((s.successful_tests / s.total_tests) * 100).toFixed(2) + "%",
          recommended_protocol: s.protocols.h3.total > 0 ? "h3" : "h2",
          avg_latency: Math.round(s.avg_connect_latency) + "ms",
        })),
      });
    }

    // 分析协议支持
    const h3Services = serviceGroups.filter(
      (s) =>
        s.protocols.h3.total > 0 &&
        (s.protocols.h3.success / s.protocols.h3.total) > 0.5,
    );

    if (h3Services.length > 0) {
      recommendations.push({
        type: "http3_support",
        title: "支持 HTTP/3 的服务",
        count: h3Services.length,
        items: h3Services.map((s) => ({
          provider: s.service_provider,
          location: s.service_location,
          prefix: s.nat64_prefix,
          h3_success_rate: s.protocols.h3.success_rate + "%",
          avg_h3_latency: Math.round(s.protocols.h3.avg_latency || 0) + "ms",
        })),
      });
    }

    // 分析低延迟服务
    const lowLatencyServices = serviceGroups
      .filter((s) => s.connect_latencies.length > 0)
      .sort((a, b) => a.avg_connect_latency - b.avg_connect_latency)
      .slice(0, 3);

    if (lowLatencyServices.length > 0) {
      recommendations.push({
        type: "low_latency",
        title: "低延迟服务推荐",
        items: lowLatencyServices.map((s) => ({
          provider: s.service_provider,
          location: s.service_location,
          prefix: s.nat64_prefix,
          avg_latency: Math.round(s.avg_connect_latency) + "ms",
          success_rate:
            ((s.successful_tests / s.total_tests) * 100).toFixed(2) + "%",
        })),
      });
    }

    // 协议统计建议
    if (protocolStats.h3.total > 0) {
      recommendations.push({
        type: "protocol_summary",
        title: "协议使用统计",
        h3: {
          total: protocolStats.h3.total,
          success_rate: protocolStats.h3.success_rate + "%",
          avg_latency: Math.round(protocolStats.h3.avg_latency || 0) + "ms",
        },
        h2: {
          total: protocolStats.h2.total,
          success_rate: protocolStats.h2.success_rate + "%",
          avg_latency: Math.round(protocolStats.h2.avg_latency || 0) + "ms",
        },
      });
    }

    return recommendations;
  }

  /**
   * 保存分析结果到 JSON 文件
   */
  saveAnalysis(outputFile) {
    if (!this.analysisResult) {
      console.error("没有可保存的分析结果");
      return false;
    }

    try {
      const jsonContent = JSON.stringify(this.analysisResult, null, 2);
      writeFileSync(outputFile, jsonContent, "utf8");
      console.log(`分析结果已保存到: ${outputFile}`);
      return true;
    } catch (error) {
      console.error("保存分析结果失败:", error.message);
      return false;
    }
  }

  /**
   * 显示分析摘要
   */
  displaySummary() {
    if (!this.analysisResult) {
      console.log("没有可显示的分析结果");
      return;
    }

    console.log("\n" + "=".repeat(60));
    console.log("NAT64 测试报告分析摘要");
    console.log("=".repeat(60));

    const { overall_statistics, protocol_statistics, service_analysis } =
      this.analysisResult;

    console.log(`\n📊 总体统计:`);
    console.log(`  总测试数: ${overall_statistics.total_tests}`);
    console.log(
      `  成功: ${overall_statistics.successful_tests} | 失败: ${overall_statistics.failed_tests}`,
    );
    console.log(`  成功率: ${overall_statistics.success_rate}`);
    console.log(
      `  DNS64 成功率: ${overall_statistics.dns64_success_rate?.toFixed(2)}%`,
    );

    console.log(`\n🚀 协议统计:`);
    console.log(
      `  HTTP/3: ${protocol_statistics.h3.total} 次测试, ${protocol_statistics.h3.success_rate}% 成功率`,
    );
    if (protocol_statistics.h3.avg_latency) {
      console.log(
        `    平均延迟: ${Math.round(protocol_statistics.h3.avg_latency)}ms`,
      );
    }
    console.log(
      `  HTTP/2: ${protocol_statistics.h2.total} 次测试, ${protocol_statistics.h2.success_rate}% 成功率`,
    );
    if (protocol_statistics.h2.avg_latency) {
      console.log(
        `    平均延迟: ${Math.round(protocol_statistics.h2.avg_latency)}ms`,
      );
    }

    console.log(`\n🏆 前 5 名 NAT64 服务:`);
    service_analysis.slice(0, 5).forEach((service, index) => {
      console.log(
        `  ${
          index + 1
        }. ${service.service_provider} (${service.service_location})`,
      );
      console.log(`     前缀: ${service.nat64_prefix}`);
      console.log(`     成功率: ${service.success_rate}`);
      console.log(
        `     HTTP/3: ${
          service.http3_available ? "✅" : "❌"
        } (${service.http3_success_rate}% 成功率)`,
      );
      console.log(
        `     HTTP/2: ${
          service.http2_available ? "✅" : "❌"
        } (${service.http2_success_rate}% 成功率)`,
      );
      console.log(
        `     推荐协议: ${service.recommended_protocol.protocol} - ${service.recommended_protocol.reason}`,
      );
      if (service.avg_connect_latency > 0) {
        console.log(
          `     平均延迟: ${Math.round(service.avg_connect_latency)}ms`,
        );
      }
    });

    console.log("\n" + "=".repeat(60));
  }
}

// 主执行函数
async function main() {
  const program = new Command();

  // 查找最新的报告文件
  const defaultFile = findLatestNAT64ReportFile();

  program
    .name("analyze-nat64-report")
    .description("NAT64 测试报告分析工具")
    .version("1.0.0")
    .option("-f, --file <path>", "NAT64 测试报告文件路径")
    .option(
      "-o, --output <path>",
      "输出分析结果文件路径",
      "nat64-analysis-result.json",
    )
    .option("--no-summary", "不显示分析摘要");

  program.parse(process.argv);
  const options = program.opts();

  // 确定输入文件
  let inputFile = options.file;
  if (!inputFile) {
    if (!defaultFile) {
      console.error("错误: 未找到 nat64-test-report-*.json 文件");
      console.log("请使用 -f 参数指定文件路径");
      process.exit(1);
    }
    inputFile = defaultFile;
  } else if (!inputFile.startsWith("/") && !/^[A-Za-z]:/.test(inputFile)) {
    inputFile = join(process.cwd(), inputFile);
  }

  // 检查文件是否存在
  if (!existsSync(inputFile)) {
    console.error(`错误: 找不到文件 ${inputFile}`);
    process.exit(1);
  }

  console.log("NAT64 测试报告分析工具 v1.0.0");
  console.log("=".repeat(50));
  console.log(`输入文件: ${inputFile}`);
  console.log(`输出文件: ${options.output}`);

  // 创建分析器
  const analyzer = new NAT64ReportAnalyzer(inputFile);

  // 加载报告
  if (!analyzer.loadReport()) {
    process.exit(1);
  }

  // 分析报告
  analyzer.analyze();

  // 显示摘要
  if (options.summary !== false) {
    analyzer.displaySummary();
  }

  // 保存结果
  analyzer.saveAnalysis(options.output);

  console.log("\n✅ 分析完成！");
}

// 如果直接运行此脚本
if (import.meta.main) {
  main();
}

export default NAT64ReportAnalyzer;
