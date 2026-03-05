package main

import (
	"context"
	"encoding/base64"
	"encoding/json"
	"flag"
	"fmt"
	"log"
	"net"
	"net/http"
	"os"
	"sync"
	"time"

	"github.com/miekg/dns"
)

// NAT64Service 定义 NAT64 服务配置
type NAT64Service struct {
	Provider      string   `json:"provider"`
	Location      string   `json:"location"`
	Website       string   `json:"website,omitempty"`
	NAT64Prefixes []string `json:"nat64_prefixes"`
	DOT           string   `json:"dot,omitempty"`
}

// TestTarget 测试目标
type TestTarget struct {
	Name        string `json:"name"`
	Host        string `json:"host"`
	ExpectedIP  string `json:"expected_ip,omitempty"`
	Path        string `json:"path,omitempty"`
	Description string `json:"description,omitempty"`
}

// NAT64TestResult NAT64 测试结果
type NAT64TestResult struct {
	Timestamp        string            `json:"timestamp"`
	ServiceProvider  string            `json:"service_provider"`
	ServiceLocation  string            `json:"service_location"`
	DNSServer        string            `json:"dns_server"`
	NAT64Prefix      string            `json:"nat64_prefix"`
	TargetHost       string            `json:"target_host"`
	TargetPath       string            `json:"target_path,omitempty"`
	OriginalIPv4     string            `json:"original_ipv4,omitempty"`
	SynthesizedIPv6  string            `json:"synthesized_ipv6,omitempty"`
	DNS64Success     bool              `json:"dns64_success"`
	DNS64LatencyMs   uint64            `json:"dns64_latency_ms"`
	ConnectSuccess   bool              `json:"connect_success"`
	ConnectLatencyMs uint64            `json:"connect_latency_ms"`
	HTTPStatusCode   *uint16           `json:"http_status_code,omitempty"`
	ErrorMessage     string            `json:"error_message,omitempty"`
	TestDetails      map[string]string `json:"test_details,omitempty"`
}

// NAT64TestReport 测试报告
type NAT64TestReport struct {
	ReportInfo  map[string]interface{} `json:"report_info"`
	TestResults []NAT64TestResult      `json:"test_results"`
	Summary     TestSummary            `json:"summary"`
}

// TestSummary 测试摘要
type TestSummary struct {
	TotalTests         int     `json:"total_tests"`
	SuccessTests       int     `json:"success_tests"`
	FailedTests        int     `json:"failed_tests"`
	DNS64SuccessRate   float64 `json:"dns64_success_rate"`
	ConnectSuccessRate float64 `json:"connect_success_rate"`
	AvgDNSLatency      float64 `json:"avg_dns_latency_ms"`
	AvgConnectLatency  float64 `json:"avg_connect_latency_ms"`
}

var (
	verbose      = flag.Bool("verbose", false, "详细输出")
	concurrency  = flag.Int("concurrency", 5, "并发测试数量")
	timeout      = flag.Int("timeout", 10, "超时时间(秒)")
	servicesFile = flag.String("services", "nat64-services.json", "NAT64服务列表文件")
	targetsFile  = flag.String("targets", "test-targets.json", "测试目标文件")
	outputFile   = flag.String("output", "", "输出文件名(默认自动生成)")
	dohURL       = flag.String("doh-url", "https://pngwczx94z.cloudflare-gateway.com/dns-query", "DoH服务URL")
)

func main() {
	flag.Parse()

	if len(os.Args) == 1 {
		fmt.Fprintf(os.Stderr, "NAT64/DNS64 服务测试工具\n")
		fmt.Fprintf(os.Stderr, "使用方法: %s [选项]\n", os.Args[0])
		flag.PrintDefaults()
		fmt.Fprintf(os.Stderr, "\n示例:\n")
		fmt.Fprintf(os.Stderr, "  %s -verbose -services custom-services.json\n", os.Args[0])
		fmt.Fprintf(os.Stderr, "  %s -concurrency 10 -timeout 15\n", os.Args[0])
		os.Exit(1)
	}

	// 加载 NAT64 服务列表
	services, err := loadServices(*servicesFile)
	if err != nil {
		log.Fatalf("加载服务列表失败: %v", err)
	}
	fmt.Printf("成功加载 %d 个 NAT64 服务\n", len(services))

	// 加载测试目标
	targets, err := loadTargets(*targetsFile)
	if err != nil {
		log.Fatalf("加载测试目标失败: %v", err)
	}
	fmt.Printf("成功加载 %d 个测试目标\n", len(targets))

	// 执行测试
	fmt.Println("\n开始测试...")
	results := runTests(services, targets)

	// 生成报告
	report := generateReport(results)

	// 保存结果
	timestamp := time.Now().Format("20060102-150405")
	outputFilename := *outputFile
	if outputFilename == "" {
		outputFilename = fmt.Sprintf("nat64-test-report-%s.json", timestamp)
	}

	if err := saveReport(report, outputFilename); err != nil {
		log.Fatalf("保存报告失败: %v", err)
	}

	fmt.Printf("\n✅ 测试完成！结果已保存到 %s\n", outputFilename)
	printSummary(report.Summary)
}

// loadServices 加载 NAT64 服务配置
func loadServices(filename string) ([]NAT64Service, error) {
	data, err := os.ReadFile(filename)
	if err != nil {
		return nil, fmt.Errorf("读取文件失败: %w", err)
	}

	var config struct {
		Services []NAT64Service `json:"services"`
	}
	if err := json.Unmarshal(data, &config); err != nil {
		return nil, fmt.Errorf("解析 JSON 失败: %w", err)
	}

	return config.Services, nil
}

// loadTargets 加载测试目标
func loadTargets(filename string) ([]TestTarget, error) {
	data, err := os.ReadFile(filename)
	if err != nil {
		return nil, fmt.Errorf("读取文件失败: %w", err)
	}

	var config struct {
		Targets []TestTarget `json:"targets"`
	}
	if err := json.Unmarshal(data, &config); err != nil {
		return nil, fmt.Errorf("解析 JSON 失败: %w", err)
	}

	return config.Targets, nil
}

// runTests 运行所有测试
func runTests(services []NAT64Service, targets []TestTarget) []NAT64TestResult {
	var wg sync.WaitGroup
	var mu sync.Mutex
	var allResults []NAT64TestResult
	sem := make(chan struct{}, *concurrency)

	testCount := 0
	for _, service := range services {
		for _, prefix := range service.NAT64Prefixes {
			for _, target := range targets {
				wg.Add(1)
				testCount++
				go func(s NAT64Service, pref string, t TestTarget) {
					sem <- struct{}{}
					defer func() {
						<-sem
						wg.Done()
					}()

					result := testNAT64Service(s, pref, t)

					mu.Lock()
					allResults = append(allResults, result)
					mu.Unlock()

					if *verbose {
						status := "❌"
						if result.ConnectSuccess {
							status = "✅"
						}
						fmt.Printf("%s [%s] %s%s -> %s\n",
							status, s.Location, t.Host, t.Path, pref)
					}
				}(service, prefix, target)
			}
		}
	}

	wg.Wait()
	fmt.Printf("完成 %d 个测试\n", testCount)
	return allResults
}

// testNAT64Service 测试单个 NAT64 服务
func testNAT64Service(service NAT64Service, nat64Prefix string, target TestTarget) NAT64TestResult {
	result := NAT64TestResult{
		Timestamp:       time.Now().Format(time.RFC3339),
		ServiceProvider: service.Provider,
		ServiceLocation: service.Location,
		DNSServer:       *dohURL,
		NAT64Prefix:     nat64Prefix,
		TargetHost:      target.Host,
		TargetPath:      target.Path,
		TestDetails:     make(map[string]string),
	}

	// 步骤 1: 通过 DoH 查询 A 记录获取 IPv4 地址
	start := time.Now()
	ipv4Addr, err := queryARecordViaDoH(target.Host)
	dnsLatency := uint64(time.Since(start).Milliseconds())
	result.DNS64LatencyMs = dnsLatency

	if err != nil {
		result.DNS64Success = false
		result.ErrorMessage = fmt.Sprintf("DoH 查询失败: %v", err)
		result.TestDetails["doh_error"] = err.Error()
		return result
	}

	result.OriginalIPv4 = ipv4Addr
	result.DNS64Success = true
	result.TestDetails["resolved_ipv4"] = ipv4Addr

	// 步骤 2: 手动合成 NAT64 地址
	synthesizedIPv6, err := synthesizeNAT64Address(nat64Prefix, ipv4Addr)
	if err != nil {
		result.ErrorMessage = fmt.Sprintf("NAT64 地址合成失败: %v", err)
		result.TestDetails["synthesis_error"] = err.Error()
		return result
	}

	result.SynthesizedIPv6 = synthesizedIPv6
	result.TestDetails["synthesized_ipv6"] = synthesizedIPv6

	// 步骤 3: 测试通过 NAT64 连接
	start = time.Now()
	success, statusCode, latency, err := testNAT64Connection(synthesizedIPv6, target.Host, target.Path)
	result.ConnectLatencyMs = latency

	if err != nil {
		result.ConnectSuccess = false
		result.ErrorMessage = fmt.Sprintf("连接失败: %v", err)
		result.TestDetails["connect_error"] = err.Error()
		return result
	}

	result.ConnectSuccess = success
	result.HTTPStatusCode = &statusCode

	return result
}

// queryARecordViaDoH 通过 DoH 查询 A 记录
func queryARecordViaDoH(domain string) (string, error) {
	// 创建 DNS 查询消息
	msg := new(dns.Msg)
	msg.SetQuestion(dns.Fqdn(domain), dns.TypeA)
	msg.RecursionDesired = true

	// 打包 DNS 消息
	msgBytes, err := msg.Pack()
	if err != nil {
		return "", fmt.Errorf("打包 DNS 消息失败: %w", err)
	}

	// Base64 编码（URL 安全）
	encodedQuery := base64.RawURLEncoding.EncodeToString(msgBytes)

	// 构建 DoH 请求 URL
	dohURL := fmt.Sprintf("%s?dns=%s", *dohURL, encodedQuery)

	// 创建 HTTP 请求
	ctx, cancel := context.WithTimeout(context.Background(), time.Duration(*timeout)*time.Second)
	defer cancel()

	req, err := http.NewRequestWithContext(ctx, "GET", dohURL, nil)
	if err != nil {
		return "", fmt.Errorf("创建 HTTP 请求失败: %w", err)
	}

	req.Header.Set("Accept", "application/dns-json")

	// 发送请求
	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		return "", fmt.Errorf("DoH 请求失败: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return "", fmt.Errorf("DoH 返回非 200 状态码: %d", resp.StatusCode)
	}

	// 解析响应
	body := make([]byte, 65535)
	n, err := resp.Body.Read(body)
	if err != nil && n == 0 {
		return "", fmt.Errorf("读取 DoH 响应失败: %w", err)
	}

	dnsResp := new(dns.Msg)
	if err := dnsResp.Unpack(body[:n]); err != nil {
		return "", fmt.Errorf("解析 DNS 响应失败: %w", err)
	}

	// 提取 A 记录
	for _, ans := range dnsResp.Answer {
		if a, ok := ans.(*dns.A); ok {
			return a.A.String(), nil
		}
	}

	return "", fmt.Errorf("未找到 A 记录")
}

// synthesizeNAT64Address 手动合成 NAT64 地址
func synthesizeNAT64Address(nat64Prefix, ipv4Addr string) (string, error) {
	// 解析 IPv4 地址
	ip := net.ParseIP(ipv4Addr)
	if ip == nil {
		return "", fmt.Errorf("无效的 IPv4 地址: %s", ipv4Addr)
	}

	ipv4 := ip.To4()
	if ipv4 == nil {
		return "", fmt.Errorf("不是 IPv4 地址: %s", ipv4Addr)
	}

	// 解析 NAT64 前缀
	_, ipNet, err := net.ParseCIDR(nat64Prefix)
	if err != nil {
		return "", fmt.Errorf("无效的 NAT64 前缀: %s", nat64Prefix)
	}

	// NAT64 前缀应该是 /96，我们取前 96 位（12 字节）
	// 然后将 IPv4 的 4 字节追加到后面
	prefixBytes := ipNet.IP.To16()
	if prefixBytes == nil {
		return "", fmt.Errorf("NAT64 前缀不是有效的 IPv6 地址")
	}

	// 创建新的 IPv6 地址：前缀 (12 字节) + IPv4 (4 字节)
	synthesized := make(net.IP, 16)
	copy(synthesized[:12], prefixBytes[:12])
	copy(synthesized[12:], ipv4[:4])

	return synthesized.String(), nil
}

// isNAT64Address 检查 IPv6 地址是否在 NAT64 前缀范围内
func isNAT64Address(ipv6Addr, prefix string) bool {
	ip := net.ParseIP(ipv6Addr)
	if ip == nil {
		return false
	}

	_, ipNet, err := net.ParseCIDR(prefix)
	if err != nil {
		return false
	}

	return ipNet.Contains(ip)
}

// testNAT64Connection 测试通过 NAT64 的连接
func testNAT64Connection(ipv6Addr, hostname, path string) (bool, uint16, uint64, error) {
	ctx, cancel := context.WithTimeout(context.Background(), time.Duration(*timeout)*time.Second)
	defer cancel()

	client := &http.Client{
		Timeout: time.Duration(*timeout) * time.Second,
		Transport: &http.Transport{
			DialContext: func(ctx context.Context, network, addr string) (net.Conn, error) {
				dialer := &net.Dialer{}
				// 强制使用合成的 IPv6 地址
				targetAddr := fmt.Sprintf("[%s]:443", ipv6Addr)
				return dialer.DialContext(ctx, "tcp6", targetAddr)
			},
		},
	}

	// 构建完整 URL，包含路径
	url := fmt.Sprintf("https://%s%s", hostname, path)
	if path == "" {
		url = fmt.Sprintf("https://%s/", hostname)
	}

	req, err := http.NewRequestWithContext(ctx, "GET", url, nil)
	if err != nil {
		return false, 0, 0, err
	}

	req.Header.Set("Host", hostname)
	req.Header.Set("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/145.0.0.0 Safari/537.36")

	start := time.Now()
	resp, err := client.Do(req)
	if err != nil {
		return false, 0, 0, err
	}
	defer resp.Body.Close()

	latency := uint64(time.Since(start).Milliseconds())

	return resp.StatusCode >= 200 && resp.StatusCode < 300, uint16(resp.StatusCode), latency, nil
}

// generateReport 生成测试报告
func generateReport(results []NAT64TestResult) NAT64TestReport {
	total := len(results)
	successCount := 0
	dns64SuccessCount := 0
	connectSuccessCount := 0
	var totalDNSLatency, totalConnectLatency uint64

	for _, r := range results {
		if r.DNS64Success {
			dns64SuccessCount++
			totalDNSLatency += r.DNS64LatencyMs
		}
		if r.ConnectSuccess {
			successCount++
			connectSuccessCount++
			totalConnectLatency += r.ConnectLatencyMs
		}
	}

	summary := TestSummary{
		TotalTests:         total,
		SuccessTests:       successCount,
		FailedTests:        total - successCount,
		DNS64SuccessRate:   float64(dns64SuccessCount) / float64(total) * 100,
		ConnectSuccessRate: float64(connectSuccessCount) / float64(total) * 100,
	}

	if dns64SuccessCount > 0 {
		summary.AvgDNSLatency = float64(totalDNSLatency) / float64(dns64SuccessCount)
	}
	if connectSuccessCount > 0 {
		summary.AvgConnectLatency = float64(totalConnectLatency) / float64(connectSuccessCount)
	}

	return NAT64TestReport{
		ReportInfo: map[string]interface{}{
			"generated_at": time.Now().Format(time.RFC3339),
			"tool_version": "1.0.0",
			"test_timeout": *timeout,
		},
		TestResults: results,
		Summary:     summary,
	}
}

// saveReport 保存测试报告
func saveReport(report NAT64TestReport, filename string) error {
	data, err := json.MarshalIndent(report, "", "  ")
	if err != nil {
		return err
	}
	return os.WriteFile(filename, data, 0644)
}

// printSummary 打印测试摘要
func printSummary(summary TestSummary) {
	fmt.Println("\n=== 测试摘要 ===")
	fmt.Printf("总测试数: %d\n", summary.TotalTests)
	fmt.Printf("成功: %d | 失败: %d\n", summary.SuccessTests, summary.FailedTests)
	fmt.Printf("DNS64 成功率: %.2f%%\n", summary.DNS64SuccessRate)
	fmt.Printf("连接成功率: %.2f%%\n", summary.ConnectSuccessRate)
	fmt.Printf("平均 DNS 延迟: %.2f ms\n", summary.AvgDNSLatency)
	fmt.Printf("平均连接延迟: %.2f ms\n", summary.AvgConnectLatency)
}
