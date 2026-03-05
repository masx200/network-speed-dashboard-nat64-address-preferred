#!/usr/bin/env node

import { exec } from "child_process";
import { promisify } from "util";

const execAsync = promisify(exec);

/**
 * 获取当前IP地址信息
 * 使用多个API服务获取IP地理位置信息
 */
class IPInfoFetcher {
  constructor() {
    this.ipinfo = {
      ip: null,
      asn: null,
      as_name: null,
      as_domain: null,
      country_code: null,
      country: null,
      continent_code: null,
      continent: null,
      latitude: null,
      longitude: null,
      time_zone: null,
      org: null,
      user_agent: null,
      source: "unknown",
    };
  }

  /**
   * 使用 curl 调用 ipinfo.io API
   */
  async fetchFromIPInfo() {
    try {
      console.log("正在从 ipinfo.io 获取IP信息...");
      const { stdout } = await execAsync(
        'curl -s https://api.ipinfo.io/lite/me -H "Authorization: Bearer e1d992dda9d73e"',
      );

      const data = JSON.parse(stdout);

      this.ipinfo = Object.assign({}, data, {
        ...this.ipinfo,
        ip: data.ip,
        asn: data.asn,
        as_name: data.as_name,
        as_domain: data.as_domain,
        country_code: data.country_code,
        country: data.country,
        continent_code: data.continent_code,
        continent: data.continent,
        source: "ipinfo.io",
      });

      console.log(`✅ ipinfo.io 获取成功: ${data.ip} (${data.country})`);
      return true;
    } catch (error) {
      console.error("❌ ipinfo.io 获取失败:", error.message);
      return false;
    }
  }

  /**
   * 使用 curl 调用 ifconfig.co API
   */
  async fetchFromIfConfig() {
    try {
      console.log("正在从 ifconfig.co 获取IP信息...");
      const { stdout } = await execAsync("curl -s https://ifconfig.co/json");

      const data = JSON.parse(stdout);

      this.ipinfo = Object.assign({}, data, {
        ...this.ipinfo,
        ip: data.ip,
        country: data.country,
        country_code: data.country_iso,
        latitude: data.latitude,
        longitude: data.longitude,
        time_zone: data.time_zone,
        asn: data.asn,
        as_name: data.asn_org,
        user_agent: data.user_agent,
        source: this.ipinfo.source === "unknown" ? "ifconfig.co" : "combined",
      });

      console.log(`✅ ifconfig.co 获取成功: ${data.ip} (${data.country})`);
      return true;
    } catch (error) {
      console.error("❌ ifconfig.co 获取失败:", error.message);
      return false;
    }
  }

  /**
   * 使用 curl 调用 ping0.cc API
   */
  async fetchFromPing0() {
    try {
      console.log("正在从 ping0.cc 获取IP信息...");
      const { stdout } = await execAsync(
        'curl -s -H "User-Agent:Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.113 Safari/537.36" https://ping0.cc/geo -L',
      );

      // ping0.cc 返回的是文本格式，需要解析
      const lines = stdout.trim().split("\n");
      if (lines.length >= 4) {
        const ip = lines[0].trim();
        const location = lines[1].trim();
        const asn = lines[2].trim();
        const org = lines[3].trim();

        // 解析位置信息
        const locationParts = location.split(/\s+/);
        const country = locationParts[0] || "unknown";
        const region = locationParts.slice(1).join(" ") || "unknown";

        // 解析ASN
        const asnMatch = asn.match(/AS(\d+)/);
        const asnNumber = asnMatch ? parseInt(asnMatch[1]) : null;

        this.ipinfo = Object.assign({}, this.ipinfo, {
          ip: ip,
          country: country,
          region: region,
          asn: asnNumber,
          as_name: org,
          source: this.ipinfo.source === "unknown" ? "ping0.cc" : "combined",
        });

        console.log(`✅ ping0.cc 获取成功: ${ip} (${country})`);
        return true;
      }
      return false;
    } catch (error) {
      console.error("❌ ping0.cc 获取失败:", error.message);
      return false;
    }
  }

  /**
   * 使用 curl 调用 api.ip.sb API
   */
  async fetchFromIPSb() {
    try {
      console.log("正在从 api.ip.sb 获取IP信息...");
      const { stdout } = await execAsync(
        'curl -s https://api.ip.sb/geoip -H "User-Agent:Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.113 Safari/537.36"',
      );

      const data = JSON.parse(stdout);

      this.ipinfo = Object.assign({}, data, {
        ...this.ipinfo,
        ip: data.ip,
        country: data.country,
        country_code: data.country_code,
        region: data.region,
        region_code: data.region_code,
        city: data.city,
        latitude: data.latitude,
        longitude: data.longitude,
        time_zone: data.timezone,
        asn: data.asn,
        as_name: data.asn_organization || data.isp,
        org: data.organization,
        continent_code: data.continent_code,
        source: this.ipinfo.source === "unknown" ? "api.ip.sb" : "combined",
      });

      console.log(`✅ api.ip.sb 获取成功: ${data.ip} (${data.country})`);
      return true;
    } catch (error) {
      console.error("❌ api.ip.sb 获取失败:", error.message);
      return false;
    }
  }

  /**
   * 获取IP信息的主要方法
   * 依次尝试不同的API服务
   */
  async fetchIPInfo() {
    console.log("🔍 开始获取当前IP地址信息...");

    let success = false;

    // 按顺序尝试各个API
    const apis = [
      { name: "ipinfo.io", method: "fetchFromIPInfo" },
      { name: "api.ip.sb", method: "fetchFromIPSb" },
      { name: "ping0.cc", method: "fetchFromPing0" },
      { name: "ifconfig.co", method: "fetchFromIfConfig" },
    ];

    for (let i = 0; i < apis.length; i++) {
      const api = apis[i];
      console.log(`尝试 ${api.name} API...`);
      const apiSuccess = await this[api.method]();

      if (apiSuccess && !success) {
        success = true;
      }

      // 如果成功了但不是第一个API，标记为combined
      if (apiSuccess && i > 0 && this.ipinfo.source !== "combined") {
        this.ipinfo.source = "combined";
      }
    }

    if (success) {
      console.log("✅ IP信息获取完成");
      console.log(`   IP地址: ${this.ipinfo.ip}`);
      console.log(
        `   国家: ${this.ipinfo.country} (${this.ipinfo.country_code})`,
      );
      if (this.ipinfo.region) {
        console.log(`   地区: ${this.ipinfo.region}`);
      }
      if (this.ipinfo.city) {
        console.log(`   城市: ${this.ipinfo.city}`);
      }
      console.log(`   ASN: ${this.ipinfo.asn}`);
      console.log(`   组织: ${this.ipinfo.as_name || this.ipinfo.org}`);
      if (this.ipinfo.latitude && this.ipinfo.longitude) {
        console.log(
          `   坐标: ${this.ipinfo.latitude}, ${this.ipinfo.longitude}`,
        );
      }
      if (this.ipinfo.time_zone) {
        console.log(`   时区: ${this.ipinfo.time_zone}`);
      }
      console.log(`   数据源: ${this.ipinfo.source}`);
    } else {
      console.error("❌ 所有IP信息获取都失败了");
      // 设置默认值，确保程序能继续运行
      this.ipinfo = {
        ip: "unknown",
        country: "unknown",
        country_code: "unknown",
        asn: "unknown",
        as_name: "unknown",
        source: "failed",
        error: "所有IP信息API都失败了",
      };
    }

    return this.ipinfo;
  }

  /**
   * 将IP信息格式化为Markdown字符串
   */
  formatAsMarkdown() {
    const timestamp = new Date().toLocaleString("zh-CN");

    let markdown = `## 🌐 当前测试环境信息

- **获取时间**: ${timestamp}
- **IP地址**: ${this.ipinfo.ip}
- **国家/地区**: ${this.ipinfo.country} (${this.ipinfo.country_code})
- **ASN**: ${this.ipinfo.asn}
- **网络组织**: ${this.ipinfo.as_name || this.ipinfo.org || "N/A"}
- **网络域名**: ${this.ipinfo.as_domain || "N/A"}`;

    if (this.ipinfo.continent) {
      markdown +=
        `\n- **大洲**: ${this.ipinfo.continent} (${this.ipinfo.continent_code})`;
    }

    if (this.ipinfo.latitude && this.ipinfo.longitude) {
      markdown +=
        `\n- **地理坐标**: ${this.ipinfo.latitude}, ${this.ipinfo.longitude}`;
    }

    if (this.ipinfo.time_zone) {
      markdown += `\n- **时区**: ${this.ipinfo.time_zone}`;
    }

    markdown += `\n- **数据源**: ${this.ipinfo.source}`;

    if (this.ipinfo.error) {
      markdown += `\n- ⚠️ **错误**: ${this.ipinfo.error}`;
    }

    markdown += `\n\n---\n\n`;

    return markdown;
  }

  /**
   * 将IP信息格式化为JSON对象
   */
  formatAsJSON() {
    return {
      timestamp: new Date().toISOString(),
      ip_info: this.ipinfo,
    };
  }
}

// 如果直接运行此脚本，则获取并显示IP信息
if (import.meta.main) {
  const fetcher = new IPInfoFetcher();
  fetcher.fetchIPInfo().then((ipInfo) => {
    console.log("\n" + "=".repeat(50));
    console.log("IP地址信息摘要");
    console.log("=".repeat(50));
    console.log(JSON.stringify(ipInfo, null, 2));
  });
}

export default IPInfoFetcher;
