<template>
  <v-app>
    <v-app-bar
      title="网络测速结果仪表板-cloudflare优选ip地址"
      color="primary"
      dark
    >
      <v-spacer></v-spacer>
      <v-btn icon @click="refreshData">
        <v-icon>mdi-refresh</v-icon>
      </v-btn>
    </v-app-bar>

    <v-main>
      <v-container fluid>
        <!-- 筛选区域 -->
        <v-card class="mb-4">
          <v-card-title>筛选条件</v-card-title>
          <v-card-text>
            <v-row>
              <v-col cols="12" md="3">
                <v-select
                  v-model="filters.country"
                  :items="countryOptions"
                  label="国家"
                  clearable
                  @update:model-value="applyFilters"
                ></v-select>
              </v-col>
              <v-col cols="12" md="3">
                <v-select
                  v-model="filters.asn"
                  :items="asnOptions"
                  label="ASN"
                  clearable
                  @update:model-value="applyFilters"
                ></v-select>
              </v-col>
              <v-col cols="12" md="3">
                <v-select
                  v-model="filters.ipVersion"
                  :items="['IPv4', 'IPv6']"
                  label="IP版本"
                  clearable
                  @update:model-value="applyFilters"
                ></v-select>
              </v-col>
              <v-col cols="12" md="3">
                <v-select
                  v-model="filters.protocol"
                  :items="protocolOptions"
                  label="协议"
                  clearable
                  @update:model-value="applyFilters"
                ></v-select>
              </v-col>
            </v-row>
            <v-row>
              <v-col cols="12" md="2">
                <v-text-field
                  v-model="filters.hostname"
                  label="主机域名搜索"
                  clearable
                  prepend-inner-icon="mdi-magnify"
                  @update:model-value="applyFilters"
                ></v-text-field>
              </v-col>
              <v-col cols="12" md="2">
                <v-select
                  v-model="filters.asName"
                  :items="asNameOptions"
                  label="ISP名称"
                  clearable
                  @update:model-value="applyFilters"
                ></v-select>
              </v-col>
              <v-col cols="12" md="2">
                <v-select
                  v-model="filters.city"
                  :items="cityOptions"
                  label="城市"
                  clearable
                  @update:model-value="applyFilters"
                ></v-select>
              </v-col>
              <v-col cols="12" md="2">
                <v-select
                  v-model="filters.statusFilter"
                  :items="statusOptions"
                  label="测试状态"
                  @update:model-value="applyFilters"
                ></v-select>
              </v-col>
              <v-col cols="12" md="2">
                <v-date-input
                  v-model="filters.testDate"
                  label="测试日期"
                  clearable
                  @update:model-value="applyFilters"
                ></v-date-input>
              </v-col>
              <v-col cols="12" md="1">
                <v-btn @click="clearFilters" color="primary" block
                  >清除筛选</v-btn
                >
              </v-col>
            </v-row>
          </v-card-text>
        </v-card>

        <!-- 统计信息 -->
        <v-row class="mb-4">
          <v-col
            cols="12"
            md="3"
            v-for="(stat, index) in statistics"
            :key="index"
          >
            <v-card>
              <v-card-title>{{ stat.title }}</v-card-title>
              <v-card-text>
                <div class="text-h4">{{ stat.value }}</div>
                <div class="text-subtitle-1">{{ stat.subtitle }}</div>
              </v-card-text>
            </v-card>
          </v-col>
        </v-row>

        <!-- 测试结果表格 -->
        <v-card>
          <v-card-title>
            测试结果
            <v-spacer></v-spacer>
            <v-text-field
              v-model="search"
              append-inner-icon="mdi-magnify"
              label="搜索"
              single-line
              hide-details
            ></v-text-field>
          </v-card-title>
          <v-card-text>
            <v-data-table
              :headers="headers"
              :items="filteredResults"
              :search="search"
              :sort-by="[{ key: 'latency_ms', order: 'asc' }]"
              :loading="loading"
              :items-per-page="itemsPerPage"
              v-model:page="currentPage"
              :items-per-page-options="[]"
              hide-default-footer
              class="elevation-1"
            >
              <template v-slot:item.success="{ item }">
                <v-chip :color="item.success ? 'success' : 'error'" dark>
                  {{ item.success ? "成功" : "失败" }}
                </v-chip>
              </template>
              <template v-slot:item.latency_ms="{ item }">
                <v-chip :color="getLatencyColor(item.latency_ms)" dark>
                  {{ item.latency_ms }} ms
                </v-chip>
              </template>
              <template v-slot:item.error_msg="{ item }">
                <span
                  v-if="item.error_msg && item.error_msg !== 'No error message'"
                >
                  {{ item.error_msg }}
                </span>
                <span v-else>-</span>
              </template>
              <template v-slot:item.testDate="{ item }">
                <span v-if="item.generatedAt">
                  {{ formatDate(item.generatedAt) }}
                </span>
                <span v-else>-</span>
              </template>
              <template v-slot:item.asn="{ item }">
                <span v-if="item.asn">{{ item.asn }}</span>
                <span v-else>-</span>
              </template>
              <template v-slot:item.country="{ item }">
                <span v-if="item.country">{{ item.country }}</span>
                <span v-else>-</span>
              </template>
              <template v-slot:item.asName="{ item }">
                <span v-if="item.asName">{{ item.asName }}</span>
                <span v-else>-</span>
              </template>
              <template v-slot:item.city="{ item }">
                <span v-if="item.city">{{ item.city }}</span>
                <span v-else>-</span>
              </template>
            </v-data-table>

            <!-- 自定义分页控制 -->
            <v-row class="mt-4" justify="center">
              <v-col cols="12" md="10">
                <v-card>
                  <v-card-text>
                    <v-row align="center" justify="center">
                      <v-col cols="6" md="1" class="text-center">
                        <v-btn
                          @click="goToFirstPage"
                          :disabled="currentPage === 1"
                          color="primary"
                          size="small"
                          block
                        >
                          第一页
                        </v-btn>
                      </v-col>
                      <v-col cols="6" md="1" class="text-center">
                        <v-btn
                          @click="currentPage = Math.max(1, currentPage - 1)"
                          :disabled="currentPage === 1"
                          color="primary"
                          size="small"
                          block
                        >
                          上一页
                        </v-btn>
                      </v-col>
                      <v-col cols="12" md="2" class="text-center d-flex align-center justify-center">
                        <span class="text-subtitle-1">
                          第 {{ currentPage }} 页，
                          共 {{ totalPages }} 页
                        </span>
                      </v-col>
                      <v-col cols="6" md="1" class="text-center">
                        <v-text-field
                          v-model.number="jumpToPage"
                          label="跳转到页"
                          type="number"
                          :min="1"
                          :max="totalPages"
                          outlined
                          dense
                          hide-details
                          @keyup.enter="goToPage"
                          style="max-width: 100px;"
                        ></v-text-field>
                      </v-col>
                      <v-col cols="6" md="1" class="text-center">
                        <v-select
                          v-model="itemsPerPage"
                          :items="itemsPerPageOptions"
                          label="每页显示"
                          @update:model-value="updateItemsPerPage"
                          style="max-width: 100px;"
                          dense
                          hide-details
                        ></v-select>
                      </v-col>
                      <v-col cols="6" md="1" class="text-center">
                        <v-btn
                          @click="currentPage = Math.min(totalPages, currentPage + 1)"
                          :disabled="currentPage >= totalPages"
                          color="primary"
                          size="small"
                          block
                        >
                          下一页
                        </v-btn>
                      </v-col>
                      <v-col cols="6" md="1" class="text-center">
                        <v-btn
                          @click="goToLastPage"
                          :disabled="currentPage >= totalPages"
                          color="primary"
                          size="small"
                          block
                        >
                          最后一页
                        </v-btn>
                      </v-col>
                    </v-row>
                  </v-card-text>
                </v-card>
              </v-col>
            </v-row>
          </v-card-text>
        </v-card>
      </v-container>
    </v-main>
  </v-app>
</template>

<script>
import { ref, onMounted, computed } from "vue";
import { VDateInput } from "vuetify/lib/labs/VDateInput/VDateInput.js";
export default {
  components: { VDateInput },
  name: "SpeedDashboard",
  setup() {
    const loading = ref(true);
    const search = ref("");
    const testReports = ref([]);
    const allResults = ref([]);
    const filteredResults = ref([]);
    const currentPage = ref(1);
    const itemsPerPage = ref(10);
    const jumpToPage = ref(null);

    // 每页显示条数选项
    const itemsPerPageOptions = ref([5, 10, 20, 50, 100]);

    // 筛选条件
    const filters = ref({
      hostname: "",
      country: null,
      asn: null,
      asName: null,
      city: null,
      ipVersion: null,
      protocol: null,
      testDate: null,
      statusFilter: "success", // 'all', 'success', 'failed'
    });

    // 状态筛选选项
    const statusOptions = [
      { title: "全部", value: "all" },
      { title: "成功", value: "success" },
      { title: "失败", value: "failed" },
    ];

    // 表格头
    const headers = [
      { title: "主机域名", key: "host", sortable: true },
      { title: "目标IP", key: "target_ip", sortable: true },
      { title: "IP版本", key: "ip_version", sortable: true },
      { title: "协议", key: "protocol", sortable: true },
      { title: "状态码", key: "status_code", sortable: true },
      { title: "延迟(ms)", key: "latency_ms", sortable: true },
      { title: "服务器", key: "server_header", sortable: true },
      { title: "状态", key: "success", sortable: true },
      { title: "测试日期", key: "testDate", sortable: true },
      { title: "错误信息", key: "error_msg", sortable: false },
      { title: "ASN", key: "asn", sortable: true },
      { title: "国家", key: "country", sortable: true },
      { title: "ISP", key: "asName", sortable: true },
      { title: "城市", key: "city", sortable: true },
    ];

    // 统计信息
    const statistics = computed(() => {
      const totalTests = allResults.value.length;
      const successTests = allResults.value.filter((r) => r.success).length;
      const failedTests = totalTests - successTests;
      // 只计算成功测试的平均延迟
      const successTestResults = allResults.value.filter((r) => r.success);
      const avgLatency =
        successTestResults.reduce((sum, r) => sum + (r.latency_ms || 0), 0) /
          successTestResults.length || 0;

      return [
        { title: "总测试数", value: totalTests, subtitle: "个" },
        { title: "成功测试", value: successTests, subtitle: "个" },
        { title: "失败测试", value: failedTests, subtitle: "个" },
        { title: "平均延迟", value: avgLatency.toFixed(2), subtitle: "ms" },
      ];
    });

    // 筛选选项
    const countryOptions = computed(() => {
      const countries = [
        ...new Set(allResults.value.map((r) => r.country).filter(Boolean)),
      ];
      console.log("Country options:", countries);
      return countries;
    });

    const asnOptions = computed(() => {
      const asns = [
        ...new Set(allResults.value.map((r) => r.asn).filter(Boolean)),
      ];
      console.log("ASN options:", asns);
      return asns;
    });

    const asNameOptions = computed(() => {
      const asNames = [
        ...new Set(allResults.value.map((r) => r.asName).filter(Boolean)),
      ];
      console.log("AS Name options:", asNames);
      return asNames;
    });

    const protocolOptions = computed(() => {
      const protocols = [
        ...new Set(allResults.value.map((r) => r.protocol).filter(Boolean)),
      ];
      return protocols;
    });

    const cityOptions = computed(() => {
      const cities = [
        ...new Set(allResults.value.map((r) => r.city).filter(Boolean)),
      ];
      console.log("City options:", cities);
      return cities;
    });

    // 计算总页数
    const totalPages = computed(() => {
      return Math.ceil(filteredResults.value.length / itemsPerPage.value) || 1;
    });

    // 跳转到指定页
    const goToPage = () => {
      if (jumpToPage.value && jumpToPage.value >= 1 && jumpToPage.value <= totalPages.value) {
        currentPage.value = jumpToPage.value;
        jumpToPage.value = null;
      }
    };

    // 跳转到第一页
    const goToFirstPage = () => {
      currentPage.value = 1;
    };

    // 跳转到最后一页
    const goToLastPage = () => {
      currentPage.value = totalPages.value;
    };

    // 更新每页显示条数
    const updateItemsPerPage = (newValue) => {
      itemsPerPage.value = newValue;
      // 重新计算当前页，确保不超出总页数
      const newTotalPages = Math.ceil(filteredResults.value.length / newValue) || 1;
      if (currentPage.value > newTotalPages) {
        currentPage.value = newTotalPages;
      }
    };

    // 获取延迟颜色
    const getLatencyColor = (latency) => {
      if (latency < 50) return "green";
      if (latency < 100) return "light-green";
      if (latency < 200) return "lime";
      if (latency < 500) return "orange";
      return "red";
    };

    // 格式化日期
    const formatDate = (dateString) => {
      try {
        const date = new Date(dateString);
        const year = date.getFullYear();
        const month = String(date.getMonth() + 1).padStart(2, '0');
        const day = String(date.getDate()).padStart(2, '0');
        return `${year}年${month}月${day}日`;
      } catch (error) {
        console.error("Date formatting error:", error);
        return dateString || "-";
      }
    };

    // 加载数据
    const loadData = async () => {
      try {
        loading.value = true;

        // 使用 import.meta.glob 读取所有 failed-test-report-*.json 文件
        const reportFiles = import.meta.glob(
          "../../failed-test-report-*.json",
          { query: "?raw", import: "default" },
        );

        console.log(
          "Loading report files in parallel:",
          Object.keys(reportFiles).length,
        );

        // 并行加载所有报告文件
        const filePromises = Object.entries(reportFiles).map(
          async ([path, loadFile]) => {
            try {
              const content = await loadFile();
              const report = JSON.parse(content);
              console.log(`Loaded report: ${path}`);

              const results = [];

              // 添加成功测试
              if (
                report.top_latency_records &&
                Array.isArray(report.top_latency_records)
              ) {
                report.top_latency_records.forEach((test) => {
                  const result = {
                    ...test,
                    testEnvironment: `${report.test_environment?.ip_info?.country || "Unknown"} (${report.test_environment?.ip_info?.as_name || "Unknown"})`,
                    country: report.test_environment?.ip_info?.country,
                    asn: report.test_environment?.ip_info?.asn,
                    asName: report.test_environment?.ip_info?.as_name,
                    city: report.test_environment?.ip_info?.city,
                    generatedAt: report.report_info?.generated_at,
                  };
                  results.push(result);
                });
              }

              // 添加失败测试
              if (report.failed_tests && Array.isArray(report.failed_tests)) {
                report.failed_tests.forEach((test) => {
                  const result = {
                    ...test,
                    testEnvironment: `${report.test_environment?.ip_info?.country || "Unknown"} (${report.test_environment?.ip_info?.as_name || "Unknown"})`,
                    country: report.test_environment?.ip_info?.country,
                    asn: report.test_environment?.ip_info?.asn,
                    asName: report.test_environment?.ip_info?.as_name,
                    city: report.test_environment?.ip_info?.city,
                    generatedAt: report.report_info?.generated_at,
                  };
                  results.push(result);
                });
              }

              console.log(
                `Processed ${results.length} test results from ${path}`,
              );
              return { report, results };
            } catch (error) {
              console.error(`Error loading report ${path}:`, error);
              return { report: null, results: [] };
            }
          },
        );

        // 等待所有文件加载完成
        const fileResults = await Promise.all(filePromises);

        // 合并所有结果
        const reports = [];
        const results = [];

        fileResults.forEach(({ report, results: testResults }) => {
          if (report) {
            reports.push(report);
          }
          results.push(...testResults);
        });

        console.log("All loaded results:", results);
        console.log("Unique countries:", [
          ...new Set(results.map((r) => r.country)),
        ]);
        console.log("Unique ASNs:", [...new Set(results.map((r) => r.asn))]);
        console.log("Unique AS Names:", [
          ...new Set(results.map((r) => r.asName)),
        ]);
        console.log(
          `Parallel loading completed: ${results.length} total results from ${reports.length} reports`,
        );

        testReports.value = reports;
        allResults.value = results;

        // 应用默认筛选条件
        applyFilters();
      } catch (error) {
        console.error("Error loading data:", error);
      } finally {
        loading.value = false;
      }
    };

    // 应用筛选
    const applyFilters = () => {
      let results = [...allResults.value];

      console.log("Applying filters:", filters.value);
      console.log("Results before filtering:", results.length);

      // 按主机域名搜索
      if (filters.value.hostname && filters.value.hostname.trim()) {
        const hostnameQuery = filters.value.hostname.trim().toLowerCase();
        results = results.filter((r) =>
          r.host && r.host.toLowerCase().includes(hostnameQuery)
        );
        console.log("After hostname filter:", results.length);
      }

      // 按国家筛选
      if (filters.value.country) {
        results = results.filter((r) => r.country === filters.value.country);
        console.log("After country filter:", results.length);
      }

      // 按ASN筛选
      if (filters.value.asn) {
        results = results.filter((r) => r.asn === filters.value.asn);
        console.log("After ASN filter:", results.length);
      }

      // 按ISP名称筛选
      if (filters.value.asName) {
        results = results.filter((r) => r.asName === filters.value.asName);
        console.log("After AS Name filter:", results.length);
      }

      // 按城市筛选
      if (filters.value.city) {
        results = results.filter((r) => r.city === filters.value.city);
        console.log("After city filter:", results.length);
      }

      // 按IP版本筛选
      if (filters.value.ipVersion) {
        results = results.filter(
          (r) => r.ip_version === filters.value.ipVersion,
        );
        console.log("After IP version filter:", results.length);
      }

      // 按协议筛选
      if (filters.value.protocol) {
        results = results.filter((r) => r.protocol === filters.value.protocol);
        console.log("After protocol filter:", results.length);
      }

      // 按测试日期筛选
      if (filters.value.testDate) {
        console.log("Filtering by test date:", filters.value.testDate);
        console.log(
          "Sample generated_at values:",
          results.slice(0, 3).map((r) => r.generatedAt),
        );

        results = results.filter((r) => {
          if (!r.generatedAt) {
            console.log("Missing generatedAt for result:", r);
            return false;
          }

          // 尝试多种日期格式解析
          let testDate;
          try {
            // 直接比较日期字符串格式
            const generatedDate = new Date(r.generatedAt)
              .toISOString()
              .split("T")[0];
            const filterDate = new Date(filters.value.testDate)
              .toISOString()
              .split("T")[0];

            console.log(`Comparing: ${generatedDate} === ${filterDate}`);
            testDate = generatedDate === filterDate;
          } catch (error) {
            console.error("Date parsing error:", error);
            return false;
          }

          return testDate;
        });
        console.log("After test date filter:", results.length);
      }

      // 按成功/失败筛选
      if (filters.value.statusFilter === "success") {
        results = results.filter((r) => r.success);
        console.log("After success filter:", results.length);
      } else if (filters.value.statusFilter === "failed") {
        results = results.filter((r) => !r.success);
        console.log("After failed filter:", results.length);
      }

      console.log("Final filtered results:", results.length);
      filteredResults.value = results;
    };

    // 清除筛选
    const clearFilters = () => {
      filters.value = {
        hostname: "",
        country: null,
        asn: null,
        asName: null,
        city: null,
        ipVersion: null,
        protocol: null,
        testDate: null,
        statusFilter: "success",
      };
      search.value = ""; // 清除搜索框
      filteredResults.value = [...allResults.value];
      applyFilters();
    };

    // 刷新数据
    const refreshData = () => {
      loadData();
    };

    // 组件挂载时加载数据
    onMounted(() => {
      loadData();
    });

    return {
      loading,
      search,
      headers,
      filteredResults,
      statistics,
      filters,
      countryOptions,
      asnOptions,
      asNameOptions,
      cityOptions,
      protocolOptions,
      statusOptions,
      currentPage,
      itemsPerPage,
      itemsPerPageOptions,
      jumpToPage,
      totalPages,
      getLatencyColor,
      formatDate,
      applyFilters,
      clearFilters,
      refreshData,
      goToPage,
      goToFirstPage,
      goToLastPage,
      updateItemsPerPage,
    };
  },
};
</script>
