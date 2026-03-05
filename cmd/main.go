package main

import (
	"context"
	"encoding/json"
	"flag"
	"fmt"
	"log"
	"net"
	"net/http"
	"net/url"
	"os"
	"sort"
	"strings"
	"sync"
	"time"

	dns_experiment "github.com/masx200/http3-reverse-proxy-server-experiment/dns"
	h3_experiment "github.com/masx200/http3-reverse-proxy-server-experiment/h3"
	"github.com/miekg/dns"
)

// InputTask 对应Rust中的InputTask结构
type InputTask struct {
	DohResolveDomain string   `json:"doh_resolve_domain"`
	TestSniHost      string   `json:"test_sni_host"`
	TestHostHeader   string   `json:"test_host_header"`
	DohURL           string   `json:"doh_url"`
	Port             int      `json:"port"`
	PreferIPv6       *bool    `json:"prefer_ipv6"`
	ResolveMode      string   `json:"resolve_mode"`
	DirectIPs        []string `json:"direct_ips"`
}

// TestResult 对应Rust中的TestResult结构
type TestResult struct {
	DomainUsed   string  `json:"domain_used"`
	TargetIP     string  `json:"target_ip"`
	IPVersion    string  `json:"ip_version"`
	SniHost      string  `json:"sni_host"`
	HostHeader   string  `json:"host_header"`
	Success      bool    `json:"success"`
	StatusCode   *uint16 `json:"status_code"`
	Protocol     string  `json:"protocol"`
	LatencyMs    *uint64 `json:"latency_ms"`
	ServerHeader *string `json:"server_header"`
	ErrorMessage *string `json:"error_msg"`
	DNSSource    string  `json:"dns_source"`
}

// 全局配置
var (
	configFile  = flag.String("config", "", "配置文件路径")
	domain      = flag.String("domain", "", "测试域名")
	dohURL      = flag.String("doh-url", "https://deno-dns-over-https-server.g18uibxgnb.de5.net/", "DoH服务URL")
	resolveMode = flag.String("resolve-mode", "https", "解析模式: https, a_aaaa, direct")
	testURL     = flag.String("test-url", "https://hello-world-deno-deploy.a1u06h9fe9y5bozbmgz3.qzz.io", "测试URL")
	port        = flag.Int("port", 443, "目标端口")
	verbose     = flag.Bool("verbose", false, "详细输出")
)

func main() {
	flag.Parse()

	var tasks []InputTask

	if *configFile != "" {
		// 从配置文件读取
		tasks = loadConfigFromFile(*configFile)
	} else if *domain != "" {
		// 使用命令行参数
		task := InputTask{
			DohResolveDomain: *domain,
			TestSniHost:      extractHostFromURL(*testURL),
			TestHostHeader:   extractHostFromURL(*testURL),
			DohURL:           *dohURL,
			Port:             *port,
			ResolveMode:      *resolveMode,
		}
		tasks = append(tasks, task)
	} else {
		// 使用默认配置
		tasks = getDefaultTasks()
	}

	if len(tasks) == 0 {
		log.Fatal("没有指定测试任务")
	}

	results := runTests(tasks)

	// 输出JSON格式的结果
	jsonOutput, _ := json.MarshalIndent(results, "", "  ")
	fmt.Println("\n=== 最终测试结果 (JSON) ===")
	fmt.Println(string(jsonOutput))
}

// 从URL提取主机名
func extractHostFromURL(rawURL string) string {
	parsedURL, err := url.Parse(rawURL)
	if err != nil {
		return rawURL
	}
	host := parsedURL.Hostname()
	if host == "" {
		return rawURL
	}
	return host
}

// 加载配置文件
func loadConfigFromFile(filename string) []InputTask {
	data, err := os.ReadFile(filename)
	if err != nil {
		log.Printf("无法读取配置文件: %v", err)
		return nil
	}

	var tasks []InputTask
	if err := json.Unmarshal(data, &tasks); err != nil {
		log.Printf("无法解析配置文件: %v", err)
		return nil
	}

	return tasks
}

// 默认测试任务
func getDefaultTasks() []InputTask {
	return []InputTask{
		{
			DohResolveDomain: "hello-world-deno-deploy.a1u06h9fe9y5bozbmgz3.qzz.io",
			TestSniHost:      "hello-world-deno-deploy.a1u06h9fe9y5bozbmgz3.qzz.io",
			TestHostHeader:   "hello-world-deno-deploy.a1u06h9fe9y5bozbmgz3.qzz.io",
			DohURL:           "https://deno-dns-over-https-server.g18uibxgnb.de5.net/",
			Port:             443,
			PreferIPv6:       getBoolPtr(true),
			ResolveMode:      "https",
		},
		{
			DohResolveDomain: "local-aria2-webui.masx200.ddns-ip.net",
			TestSniHost:      "local-aria2-webui.masx200.ddns-ip.net",
			TestHostHeader:   "local-aria2-webui.masx200.ddns-ip.net",
			DohURL:           "https://deno-dns-over-https-server.g18uibxgnb.de5.net/",
			Port:             443,
			PreferIPv6:       getBoolPtr(true),
			ResolveMode:      "https",
		},
		{
			DohResolveDomain: "local-aria2-webui.masx200.ddns-ip.net",
			TestSniHost:      "local-aria2-webui.masx200.ddns-ip.net",
			TestHostHeader:   "local-aria2-webui.masx200.ddns-ip.net",
			DohURL:           "https://deno-dns-over-https-server.g18uibxgnb.de5.net/",
			Port:             443,
			PreferIPv6:       getBoolPtr(true),
			DirectIPs:        []string{"162.159.140.220", "172.67.214.232", "2606:4700:7::da", "2a06:98c1:58::da"},
			ResolveMode:      "direct",
		},
	}
}

// 辅助函数：获取bool指针
func getBoolPtr(b bool) *bool {
	return &b
}

// 运行所有测试
func runTests(tasks []InputTask) []TestResult {
	var wg sync.WaitGroup
	var mu sync.Mutex
	var results []TestResult

	for _, task := range tasks {
		fmt.Printf(">>> 正在通过 %s 解析 %s 的记录 (模式: %s)...\n",
			task.DohURL, task.DohResolveDomain, task.ResolveMode)

		ips, err := resolveDomain(&task)
		if err != nil {
			fmt.Printf("    [X] DNS解析失败: %v\n", err)
			continue
		}

		if len(ips) == 0 {
			fmt.Printf("    [!] 未找到IP地址\n")
			continue
		}

		fmt.Printf("    -> 解析成功，获取到 %d 个IP地址: %v\n", len(ips), ips)

		// 对每个IP进行连通性测试
		for _, ip := range ips {
			// IPv4/IPv6 优先级过滤
			if task.PreferIPv6 != nil {
				isIPv6 := strings.Contains(ip, ":")
				if *task.PreferIPv6 != isIPv6 {
					continue
				}
			}

			wg.Add(1)
			go func(t InputTask, targetIP string) {
				defer wg.Done()

				dnsSource := t.ResolveMode
				if t.ResolveMode == "direct" {
					dnsSource = "Direct Input"
				} else {
					dnsSource = fmt.Sprintf("DoH (%s)", t.DohURL)
				}

				result := testConnectivity(t, targetIP, dnsSource)

				mu.Lock()
				results = append(results, result)
				mu.Unlock()
			}(task, ip)
		}
	}

	wg.Wait()
	return results
}

// DNS解析函数
func resolveDomain(task *InputTask) ([]string, error) {
	// 直接IP模式
	if len(task.DirectIPs) > 0 && task.ResolveMode == "direct" {
		fmt.Printf("    -> 使用直接指定的IP: %v\n", task.DirectIPs)
		return filterValidIPs(task.DirectIPs), nil
	}

	switch task.ResolveMode {
	case "https":
		// 使用DoH (RFC 8484标准) - 同时查询IPv4和IPv6记录
		fmt.Printf("    -> 使用DoH查询 (RFC 8484) - 同时查询IPv4和IPv6: %s\n", task.DohResolveDomain)
		return dohLookup(task.DohResolveDomain, task.DohURL)
	case "a_aaaa":
		// 传统A/AAAA记录查询
		fmt.Printf("    -> 使用传统DNS查询: %s\n", task.DohResolveDomain)
		return traditionalDNSLookup(task.DohResolveDomain)
	case "direct":
		// 直接模式已在开头处理
		return filterValidIPs(task.DirectIPs), nil
	default:
		return nil, fmt.Errorf("不支持的解析模式: %s", task.ResolveMode)
	}
}

// DoH查询 - 同时查询A和AAAA记录以支持IPv4和IPv6
func dohLookup(domain, dohURL string) ([]string, error) {
	var allIPs []string

	// 并发查询A和AAAA记录以提升性能
	var wg sync.WaitGroup
	var mu sync.Mutex

	// 查询A记录 (IPv4)
	wg.Add(1)
	go func() {
		defer wg.Done()
		msg := new(dns.Msg)
		msg.SetQuestion(dns.Fqdn(domain), dns.TypeA)

		ips, err := performDoHQuery(msg, dohURL)
		if err != nil && *verbose {
			fmt.Printf("    -> DoH查询A记录失败: %v\n", err)
		} else if len(ips) > 0 {
			mu.Lock()
			allIPs = append(allIPs, ips...)
			mu.Unlock()
		}
	}()

	// 查询AAAA记录 (IPv6)
	wg.Add(1)
	go func() {
		defer wg.Done()
		msg := new(dns.Msg)
		msg.SetQuestion(dns.Fqdn(domain), dns.TypeAAAA)

		ips, err := performDoHQuery(msg, dohURL)
		if err != nil && *verbose {
			fmt.Printf("    -> DoH查询AAAA记录失败: %v\n", err)
		} else if len(ips) > 0 {
			mu.Lock()
			allIPs = append(allIPs, ips...)
			mu.Unlock()
		}
	}()

	wg.Wait()

	return filterValidIPs(allIPs), nil
}

// 执行DoH查询 - 使用实验库
func performDoHQuery(msg *dns.Msg, dohURL string, dohIPs ...string) ([]string, error) {
	// 直接使用实验库的DoH客户端，支持HTTP/3和HTTP/2回退
	resp, err := dns_experiment.DohClient(msg, dohURL, dohIPs...)
	if err != nil {
		return nil, err
	}

	var ips []string
	for _, ans := range resp.Answer {
		switch a := ans.(type) {
		case *dns.A:
			ips = append(ips, a.A.String())
		case *dns.AAAA:
			ips = append(ips, a.AAAA.String())
		}
	}

	return filterValidIPs(ips), nil
}

// 传统DNS查询
func traditionalDNSLookup(domain string) ([]string, error) {
	// 使用标准库进行DNS查询
	ips, err := net.LookupIP(domain)
	if err != nil {
		return nil, err
	}

	var ipStrs []string
	for _, ip := range ips {
		ipStrs = append(ipStrs, ip.String())
	}

	return filterValidIPs(ipStrs), nil
}

// 过滤有效IP地址
func filterValidIPs(ips []string) []string {
	var validIPs []string

	for _, ip := range ips {
		if isValidIPv4Address(ip) || isValidIPv6Address(ip) {
			if !isBadIPv4Address(ip) {
				validIPs = append(validIPs, ip)
			}
		}
	}

	// 排序：IPv4在前，IPv6在后
	sort.Slice(validIPs, func(i, j int) bool {
		isIPv6_i := strings.Contains(validIPs[i], ":")
		isIPv6_j := strings.Contains(validIPs[j], ":")
		return !isIPv6_i && isIPv6_j
	})

	return validIPs
}

// IPv4地址验证
func isValidIPv4Address(ipStr string) bool {
	ip := net.ParseIP(ipStr)
	if ip == nil || ip.To4() == nil {
		return false
	}

	// 排除无效的IPv4地址
	switch ipStr {
	case "0.0.0.0", "127.0.0.1", "255.255.255.255":
		return false
	}

	return true
}

// IPv6地址验证
func isValidIPv6Address(ipStr string) bool {
	ip := net.ParseIP(ipStr)
	return ip != nil && ip.To4() == nil
}

// 检查是否为已知的错误IPv4地址
func isBadIPv4Address(ipStr string) bool {
	return ipStr == "183.192.65.101"
}

// 测试HTTP/3连接性
func testConnectivity(task InputTask, targetIP, dnsSource string) TestResult {
	start := time.Now()

	result := TestResult{
		DomainUsed: task.DohResolveDomain,
		TargetIP:   targetIP,
		IPVersion:  "IPv4",
		SniHost:    task.TestSniHost,
		HostHeader: task.TestHostHeader,
		DNSSource:  dnsSource,
	}

	if strings.Contains(targetIP, ":") {
		result.IPVersion = "IPv6"
	}

	// 构建测试URL
	testURL := fmt.Sprintf("https://%s:%d/", task.TestSniHost, task.Port)

	// 首先尝试HTTP/3
	success, protocol, statusCode, serverHeader, latencyMs, err := testHTTP3Connection(testURL, task.TestHostHeader, targetIP, task.Port, start)

	if err != nil {
		if *verbose {
			fmt.Printf("    -> HTTP/3连接失败: %v, 尝试HTTP/2...\n", err)
		}
		// 回退到HTTP/2
		success, protocol, statusCode, serverHeader, latencyMs, err = testHTTP2Connection(testURL, task.TestHostHeader, targetIP, task.Port, start)
	}

	result.Success = success
	result.Protocol = protocol
	result.LatencyMs = &latencyMs

	if err != nil {
		errMsg := err.Error()
		result.ErrorMessage = &errMsg
	} else {
		result.StatusCode = &statusCode
		result.ServerHeader = &serverHeader
	}

	return result
}

// 测试HTTP/3连接
func testHTTP3Connection(testURL, hostHeader, targetIP string, port int, start time.Time) (bool, string, uint16, string, uint64, error) {
	// 创建HTTP/3客户端
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	// 使用H3实验传输器
	transport := h3_experiment.CreateHTTP3TransportWithIP(targetIP)

	client := &http.Client{
		Transport: transport,
		Timeout:   10 * time.Second,
	}

	req, err := http.NewRequestWithContext(ctx, "GET", testURL, nil)
	if err != nil {
		return false, "none", 0, "", 0, err
	}

	req.Header.Set("Host", hostHeader)
	req.Header.Set("User-Agent", "curl/8.12.1")

	resp, err := client.Do(req)
	if err != nil {
		if *verbose {
			fmt.Printf("    -> HTTP/3连接失败: %v\n", err)
		}
		return false, "h3", 0, "", 0, err
	}
	defer resp.Body.Close()

	latencyMs := uint64(time.Since(start).Milliseconds())
	statusCode := uint16(resp.StatusCode)
	serverHeader := resp.Header.Get("Server")

	return resp.StatusCode < 500, "h3", statusCode, serverHeader, latencyMs, nil
}

// 测试HTTP/2连接
func testHTTP2Connection(testURL, hostHeader, targetIP string, port int, start time.Time) (bool, string, uint16, string, uint64, error) {
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	// 构建目标地址
	targetAddr := fmt.Sprintf("[%s]:%d", targetIP, port)
	if !strings.Contains(targetIP, ":") {
		targetAddr = fmt.Sprintf("%s:%d", targetIP, port)
	}

	req, err := http.NewRequestWithContext(ctx, "GET", testURL, nil)
	if err != nil {
		return false, "none", 0, "", 0, err
	}

	req.Header.Set("Host", hostHeader)
	req.Header.Set("User-Agent", "curl/8.12.1")

	client := &http.Client{
		Transport: &http.Transport{
			DialContext: func(ctx context.Context, network, addr string) (net.Conn, error) {
				dialer := &net.Dialer{}
				return dialer.DialContext(ctx, "tcp", targetAddr)
			},
			ForceAttemptHTTP2: true,
		},
		Timeout: 10 * time.Second,
	}

	resp, err := client.Do(req)
	if err != nil {
		return false, "none", 0, "", 0, err
	}
	defer resp.Body.Close()

	latencyMs := uint64(time.Since(start).Milliseconds())
	statusCode := uint16(resp.StatusCode)
	serverHeader := resp.Header.Get("Server")

	protocol := "http/1.1"
	if resp.ProtoMajor == 2 {
		protocol = "h2"
	}

	return resp.StatusCode < 500, protocol, statusCode, serverHeader, latencyMs, nil
}
