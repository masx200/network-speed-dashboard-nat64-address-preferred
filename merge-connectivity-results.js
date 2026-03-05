import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

/**
 * 合并所有 connectivity_results-*.json 文件
 * 并按 target_ip 进行去重
 */
async function mergeConnectivityResults() {
  console.log("开始合并 connectivity_results 文件...\n");

  // 查找所有 connectivity_results-*.json 文件
  const allFiles = fs.readdirSync(__dirname);
  const files = allFiles.filter((file) =>
    file.startsWith("connectivity_results-") && file.endsWith(".json")
  );

  if (files.length === 0) {
    console.log("未找到任何 connectivity_results-*.json 文件");
    return;
  }

  console.log(`找到 ${files.length} 个文件:`);
  files.forEach((file) => console.log(`  - ${file}`));
  console.log("");

  // 读取并合并所有数据
  const ipMap = new Map(); // 使用 Map 来去重，key 为 target_ip
  let totalRecords = 0;
  let duplicateCount = 0;

  for (const file of files) {
    try {
      const filePath = path.join(__dirname, file);
      const content = fs.readFileSync(filePath, "utf-8");
      const data = JSON.parse(content);

      if (!Array.isArray(data)) {
        console.warn(`警告: ${file} 不是数组格式，跳过`);
        continue;
      }

      console.log(`处理 ${file}: ${data.length} 条记录`);

      for (const record of data) {
        totalRecords++;

        // 使用 target_ip 作为去重键
        const key = record.target_ip;

        if (ipMap.has(key)) {
          duplicateCount++;
          // 如果需要保留最新的记录，可以取消下面的注释
          // ipMap.set(key, record);
        } else {
          ipMap.set(key, record);
        }
      }
    } catch (error) {
      console.error(`处理 ${file} 时出错: ${error.message}`);
    }
  }

  // 将 Map 转换为数组
  const mergedData = Array.from(ipMap.values());

  // 按照某种字段排序（可选，例如按 latency_ms）
  mergedData.sort((a, b) => {
    if (a.latency_ms && b.latency_ms) {
      return a.latency_ms - b.latency_ms;
    }
    return a.target_ip.localeCompare(b.target_ip);
  });

  // 输出合并后的文件
  const outputFile = path.join(__dirname, "merged-connectivity_results.json");
  fs.writeFileSync(outputFile, JSON.stringify(mergedData, null, 2), "utf-8");

  console.log("\n========== 合并完成 ==========");
  console.log(`原始记录总数: ${totalRecords}`);
  console.log(`重复记录数: ${duplicateCount}`);
  console.log(`去重后记录数: ${mergedData.length}`);
  console.log(`输出文件: ${outputFile}`);
  console.log("=============================\n");
}

// 运行
mergeConnectivityResults().then(console.log).catch((error) => {
  console.error("发生错误:", error);
  process.exit(1);
});
