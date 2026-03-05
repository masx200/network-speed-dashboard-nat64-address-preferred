import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// 查找所有 failed-test-report-*.json 文件
function findTestReports() {
  const files = fs.readdirSync(__dirname);
  return files.filter((file) =>
    file.startsWith("failed-test-report-") && file.endsWith(".json")
  );
}

// 查找所有 connectivity_results-*.json 文件
function findConnectivityResults() {
  const files = fs.readdirSync(__dirname);
  return files.filter((file) =>
    file.startsWith("connectivity_results-") && file.endsWith(".json")
  ).concat("connectivity_results.json");
}

// 清理单个报告文件，移除 success: false 的记录
function cleanReport(filePath) {
  try {
    const content = fs.readFileSync(filePath, "utf-8");
    const report = JSON.parse(content);

    let modified = false;
    let removedCount = 0;

    // 清理 top_latency_records 中的失败记录
    if (Array.isArray(report.top_latency_records)) {
      const originalLength = report.top_latency_records.length;
      report.top_latency_records = report.top_latency_records.filter((record) =>
        record.success !== false
      );
      removedCount += originalLength - report.top_latency_records.length;

      if (removedCount > 0) {
        modified = true;
      }
    }

    // 清理 failed_tests 数组中的所有记录（这些通常都是失败的）
    if (Array.isArray(report.failed_tests) && report.failed_tests.length > 0) {
      removedCount += report.failed_tests.length;
      report.failed_tests = [];
      modified = true;
    }

    // 清理 error_analysis.grouped_tests 中的失败记录
    if (report.error_analysis && report.error_analysis.grouped_tests) {
      const groupedTests = report.error_analysis.grouped_tests;
      let groupedRemovedCount = 0;

      // 遍历所有分组，移除失败记录
      for (const groupName in groupedTests) {
        if (Array.isArray(groupedTests[groupName])) {
          const originalLength = groupedTests[groupName].length;
          groupedTests[groupName] = groupedTests[groupName].filter((record) =>
            record.success !== false
          );
          groupedRemovedCount += originalLength -
            groupedTests[groupName].length;
        }
      }

      if (groupedRemovedCount > 0) {
        removedCount += groupedRemovedCount;
        modified = true;
      }
    }

    // 如果有修改，写回文件
    if (modified) {
      fs.writeFileSync(filePath, JSON.stringify(report, null, 2), "utf-8");
      console.log(
        `✓ 已清理 ${
          path.basename(filePath)
        } - 移除了 ${removedCount} 条失败记录`,
      );
      return { file: path.basename(filePath), removed: removedCount };
    } else {
      console.log(`- 跳过 ${path.basename(filePath)} - 无失败记录`);
      return { file: path.basename(filePath), removed: 0 };
    }
  } catch (error) {
    console.error(`✗ 处理 ${path.basename(filePath)} 时出错:`, error.message);
    return { file: path.basename(filePath), error: error.message };
  }
}

// 清理 connectivity_results 文件
function cleanConnectivityResults(filePath) {
  try {
    const content = fs.readFileSync(filePath, "utf-8");
    const data = JSON.parse(content);

    let modified = false;
    let removedCount = 0;

    // 如果是数组格式，过滤掉 success: false 的记录
    if (Array.isArray(data)) {
      const originalLength = data.length;
      const cleanedData = data.filter((record) => record.success !== false);
      removedCount = originalLength - cleanedData.length;

      if (removedCount > 0) {
        fs.writeFileSync(
          filePath,
          JSON.stringify(cleanedData, null, 2),
          "utf-8",
        );
        modified = true;
        console.log(
          `✓ 已清理 ${
            path.basename(filePath)
          } - 移除了 ${removedCount} 条失败记录`,
        );
        return { file: path.basename(filePath), removed: removedCount };
      }
    }

    if (!modified) {
      console.log(`- 跳过 ${path.basename(filePath)} - 无失败记录`);
      return { file: path.basename(filePath), removed: 0 };
    }
  } catch (error) {
    console.error(`✗ 处理 ${path.basename(filePath)} 时出错:`, error.message);
    return { file: path.basename(filePath), error: error.message };
  }
}

// 主函数
function main() {
  console.log("开始清理测试报告和结果文件...\n");

  const reportFiles = findTestReports();
  const connectivityFiles = findConnectivityResults();

  if (reportFiles.length === 0 && connectivityFiles.length === 0) {
    console.log("未找到任何测试报告或结果文件");
    return;
  }

  console.log(
    `找到 ${reportFiles.length} 个测试报告文件和 ${connectivityFiles.length} 个结果文件\n`,
  );

  const results = [];
  let totalRemoved = 0;

  // 处理测试报告文件
  for (const file of reportFiles) {
    const filePath = path.join(__dirname, file);
    const result = cleanReport(filePath);
    results.push(result);
    if (result.removed) {
      totalRemoved += result.removed;
    }
  }

  // 处理 connectivity_results 文件
  for (const file of connectivityFiles) {
    const filePath = path.join(__dirname, file);
    const result = cleanConnectivityResults(filePath);
    results.push(result);
    if (result.removed) {
      totalRemoved += result.removed;
    }
  }

  console.log("\n清理完成!");
  console.log(`总共移除了 ${totalRemoved} 条失败记录`);

  // 显示详细统计
  const successful = results.filter((r) => !r.error);
  const withRemovals = successful.filter((r) => r.removed > 0);
  console.log(`处理文件: ${successful.length}/${results.length}`);
  console.log(`有清理的文件: ${withRemovals.length}`);
}

main();
