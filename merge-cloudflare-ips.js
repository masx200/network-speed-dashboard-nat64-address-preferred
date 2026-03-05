import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// 读取 cloudflare_ips.json
const cloudflareIpsPath = path.join(__dirname, "cloudflare_ips.json");
const cloudflareIps = JSON.parse(fs.readFileSync(cloudflareIpsPath, "utf8"));

// 读取 hosts.json
const hostsPath = path.join(__dirname, "hosts.json");
const hosts = JSON.parse(fs.readFileSync(hostsPath, "utf8"));

// 从 cloudflare_ips.json 提取 IP 地址，转换为 host 格式
const newHosts = cloudflareIps.data.map((entry) => ({
  ...entry,
  host: entry.ip,
}));

// 合并两个数组
const mergedHosts = [...hosts, ...newHosts];

// 去重（基于 host 字段）
const seen = new Set();
const deduplicatedHosts = mergedHosts.filter((item) => {
  if (seen.has(item.host)) {
    return false;
  }
  seen.add(item.host);
  return true;
});

// 写入覆盖 hosts.json
fs.writeFileSync(hostsPath, JSON.stringify(deduplicatedHosts, null, 2), "utf8");

console.log(`合并完成！`);
console.log(`原 hosts.json 数量: ${hosts.length}`);
console.log(`cloudflare_ips.json 数量: ${newHosts.length}`);
console.log(`合并后去重数量: ${deduplicatedHosts.length}`);
