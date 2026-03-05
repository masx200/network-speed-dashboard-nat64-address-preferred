package main

import (
	"context"
	"encoding/json"
	"flag"
	"fmt"
	"log"
	"net"
	"net/http"
	"os"
	"strings"
	"sync"
	"time"

	dns_experiment "github.com/masx200/http3-reverse-proxy-server-experiment/dns"
	h3_experiment "github.com/masx200/http3-reverse-proxy-server-experiment/h3"
	"github.com/miekg/dns"
)

type HostEntry struct {
	Host string `json:"host"`
}

type TestResult struct {
	Host         string  `json:"host"`
	TargetIP     string  `json:"target_ip"`
	IPVersion    string  `json:"ip_version"`
	Success      bool    `json:"success"`
	StatusCode   *uint16 `json:"status_code"`
	Protocol     string  `json:"protocol"`
	LatencyMs    *uint64 `json:"latency_ms"`
	ServerHeader *string `json:"server_header"`
	ErrorMessage *string `json:"error_msg"`
}

// IP版本类型
type IPVersionFilter string

const (
	IPVersionAll IPVersionFilter = "all"
	IPVersionV4  IPVersionFilter = "ipv4"
	IPVersionV6  IPVersionFilter = "ipv6"
)

// 全局配置
var (
	verbose     = flag.Bool("verbose", false, "详细输出")
	concurrency = flag.Int("concurrency", 10, "并发测试数量")
	timeout     = flag.Int("timeout", 10, "超时时间(秒)")
	inputFile   = flag.String("input", "hosts.json", "输入文件路径")
	SERVERSNI   = flag.String("sni", "local-aria2-webui.masx200.ddns-ip.net", "SNI名称")
	DOHURL      = flag.String("doh-url", "https://deno-dns-over-https-server.g18uibxgnb.de5.net/", "DoH查询URL")
	PORT        = flag.Int("port", 443, "目标端口")
	DOHIP       = flag.String("dohip", "", "强制解析 ykxkqhbc8x.apuk83ea3z.de5.net 到指定IP")
	ipVersion   = flag.String("ip-version", string(IPVersionAll), "IP版本过滤 (ipv4/ipv6/all), 默认为all")
)

func main() {
	flag.Parse()

	// 显示使用说明
	if len(os.Args) == 1 {
		fmt.Fprintf(os.Stderr, "使用方法: %s [选项]\n", os.Args[0])
		flag.PrintDefaults()
		fmt.Fprintf(os.Stderr, "\n示例:\n")
		fmt.Fprintf(os.Stderr, "  %s -verbose -input custom_hosts.json\n", os.Args[0])
		fmt.Fprintf(os.Stderr, "  %s -concurrency 20 -timeout 15\n", os.Args[0])
		fmt.Fprintf(os.Stderr, "  %s -ip-version ipv4 -verbose\n", os.Args[0])
		fmt.Fprintf(os.Stderr, "  %s -ip-version ipv6\n", os.Args[0])
		os.Exit(1)
	}

	// 读取hosts文件
	hosts, err := loadHosts(*inputFile)
	if err != nil {
		log.Fatalf("加载hosts文件失败: %v", err)
	}

	fmt.Printf("成功加载 %d 个host，开始测试连通性...\n", len(hosts))

	// 测试所有host
	results := testHostsConnectivity(hosts)

	// 生成带时间戳的文件名
	timestamp := time.Now().Format("20060102-150405")
	outputFile := fmt.Sprintf("connectivity_results-%s.json", timestamp)

	// 输出结果到JSON文件
	if err := saveResults(results, outputFile); err != nil {
		log.Fatalf("保存结果失败: %v", err)
	}

	fmt.Printf("\n测试完成！结果已保存到 %s\n", outputFile)

	// 统计结果
	successResults := filterResults(results, true)
	failedResults := filterResults(results, false)

	// 统计每个host的成功情况
	hostStats := make(map[string]int)
	successHosts := make(map[string]bool)

	for _, result := range results {
		hostStats[result.Host]++
		if result.Success {
			successHosts[result.Host] = true
		}
	}

	fmt.Printf("总测试次数: %d (成功: %d, 失败: %d)\n", len(results), len(successResults), len(failedResults))
	fmt.Printf("测试主机数: %d (至少一个IP成功: %d)\n", len(hostStats), len(successHosts))
}

// 加载hosts.json文件
func loadHosts(filename string) ([]string, error) {
	data, err := os.ReadFile(filename)
	if err != nil {
		return nil, err
	}

	var hosts []HostEntry
	if err := json.Unmarshal(data, &hosts); err != nil {
		return nil, err
	}

	var hostList []string
	for _, h := range hosts {
		hostList = append(hostList, h.Host)
	}

	return hostList, nil
}

// 测试所有host的连通性
func testHostsConnectivity(hosts []string) []TestResult {
	var wg sync.WaitGroup
	var mu sync.Mutex
	var allResults []TestResult
	sem := make(chan struct{}, *concurrency)

	for _, host := range hosts {
		wg.Add(1)
		go func(host string) {
			sem <- struct{}{}
			defer func() {
				<-sem
				wg.Done()
			}()

			if *verbose {
				fmt.Printf("测试 %s...\n", host)
			}

			hostResults := testSingleHost(host)

			mu.Lock()
			allResults = append(allResults, hostResults...)
			mu.Unlock()
		}(host)
	}

	wg.Wait()
	return allResults
}

// 测试单个host的连通性
func testSingleHost(host string) []TestResult {
	// 判断是否为IP地址
	isIP := isIPAddress(host)

	var targetIPs []string
	var err error
	var originalHost string = host // 保存原始host用于显示

	if isIP {
		// 如果是IP，去掉方括号后使用（如果有的话）
		targetIP := host
		if strings.HasPrefix(host, "[") && strings.HasSuffix(host, "]") {
			targetIP = host[1 : len(host)-1]
		}
		// 检查IP版本是否符合过滤条件
		ip := net.ParseIP(targetIP)
		if ip == nil {
			return []TestResult{{
				Host:         host,
				Success:      false,
				ErrorMessage: stringPtr(fmt.Sprintf("无效的IP地址: %s", targetIP)),
			}}
		}

		// 根据ipVersion过滤
		isIPv4 := ip.To4() != nil
		filtered := false
		if IPVersionFilter(*ipVersion) == IPVersionV4 && !isIPv4 {
			filtered = true
			if *verbose {
				fmt.Printf("  %s 是IPv6地址，已跳过(-ip-version=ipv4)\n", originalHost)
			}
		} else if IPVersionFilter(*ipVersion) == IPVersionV6 && isIPv4 {
			filtered = true
			if *verbose {
				fmt.Printf("  %s 是IPv4地址，已跳过(-ip-version=ipv6)\n", originalHost)
			}
		}

		if filtered {
			return []TestResult{{
				Host:         host,
				TargetIP:     targetIP,
				IPVersion:    map[bool]string{true: "IPv4", false: "IPv6"}[isIPv4],
				Success:      false,
				ErrorMessage: stringPtr("因IP版本过滤而跳过"),
			}}
		}

		targetIPs = []string{targetIP}
		if *verbose {
			fmt.Printf("  %s 是IP地址，直接使用\n", originalHost)
		}
	} else {
		// 如果是域名，进行DNS解析
		if *verbose {
			fmt.Printf("  %s 是域名，进行DoH解析...\n", host)
		}
		targetIPs, err = dohLookup(host, *DOHURL, *DOHIP, IPVersionFilter(*ipVersion))
		if err != nil {
			return []TestResult{{
				Host:         host,
				Success:      false,
				ErrorMessage: stringPtr(fmt.Sprintf("DNS解析失败: %v", err)),
			}}
		}
		if len(targetIPs) == 0 {
			return []TestResult{{
				Host:         host,
				Success:      false,
				ErrorMessage: stringPtr("DNS解析无结果"),
			}}
		}
		if *verbose {
			fmt.Printf("  解析到 %d 个IP: %v\n", len(targetIPs), targetIPs)
		}
	}

	// 为每个IP创建测试结果
	results := make([]TestResult, len(targetIPs))
	for i, targetIP := range targetIPs {
		// 构建测试URL - 使用SERVERSNI而不是host，因为SNI和IP可以是不同的
		serverHost := *SERVERSNI
		if serverHost == "" {
			serverHost = host // 如果没有指定SNI，回退到host
		}
		testURL := fmt.Sprintf("https://%s:%d/", serverHost, *PORT)

		// 测试HTTP/3连接
		success, protocol, statusCode, serverHeader, latencyMs, err := testHTTP3Connection(
			testURL, serverHost, targetIP, *PORT, *timeout)

		if err != nil {
			if *verbose {
				fmt.Printf("  IP %s HTTP/3失败: %v，尝试HTTP/2...\n", targetIP, err)
			}
			// 回退到HTTP/2
			success, protocol, statusCode, serverHeader, latencyMs, err = testHTTP2Connection(
				testURL, serverHost, targetIP, *PORT, *timeout)
		}

		ipVersion := "IPv4"
		if strings.Contains(targetIP, ":") {
			ipVersion = "IPv6"
		}

		result := TestResult{
			Host:      host,
			TargetIP:  targetIP,
			IPVersion: ipVersion,
			Success:   success,
			Protocol:  protocol,
			LatencyMs: uint64Ptr(latencyMs),
		}

		if err != nil {
			result.ErrorMessage = stringPtr(err.Error())
		} else {
			result.StatusCode = uint16Ptr(statusCode)
			result.ServerHeader = stringPtr(serverHeader)
		}

		results[i] = result

		if *verbose {
			if success {
				fmt.Printf("  IP %s 测试成功 (%s, %dms)\n", targetIP, protocol, latencyMs)
			} else {
				fmt.Printf("  IP %s 测试失败\n", targetIP)
			}
		}
	}

	return results
}

// 判断是否为IP地址（处理带方括号的IPv6地址）
func isIPAddress(host string) bool {
	// 去掉首尾的方括号（如果有的话）
	trimmedHost := host
	if strings.HasPrefix(host, "[") && strings.HasSuffix(host, "]") {
		trimmedHost = host[1 : len(host)-1]
	}

	ip := net.ParseIP(trimmedHost)
	return ip != nil
}

// DoH查询
func dohLookup(domain, dohURL string, dohip string, ipVersion IPVersionFilter) ([]string, error) {
	var allIPs []string

	// 准备DoH服务器的IP地址
	var dohIPs []string
	if dohip != "" {
		// 如果提供了dohip参数，使用指定的IP
		dohIPs = append(dohIPs, dohip)
		if *verbose {
			fmt.Printf("  使用强制DoH服务器IP: %s\n", dohip)
		}
	}

	// 根据IP版本设置查询相应的DNS记录
	msg := new(dns.Msg)

	// 查询A记录（IPv4）
	if ipVersion == IPVersionAll || ipVersion == IPVersionV4 {
		msg.SetQuestion(dns.Fqdn(domain), dns.TypeA)
		aIPs, err := performDoHQuery(msg, dohURL, dohIPs...)
		if err != nil && *verbose {
			fmt.Printf("  DoH查询A记录失败: %v\n", err)
		} else if len(aIPs) > 0 {
			allIPs = append(allIPs, aIPs...)
			if *verbose {
				fmt.Printf("  查询到A记录: %v\n", aIPs)
			}
		}
	}

	// 查询AAAA记录（IPv6）
	if ipVersion == IPVersionAll || ipVersion == IPVersionV6 {
		msg.SetQuestion(dns.Fqdn(domain), dns.TypeAAAA)
		aaaaIPs, err := performDoHQuery(msg, dohURL, dohIPs...)
		if err != nil && *verbose {
			fmt.Printf("  DoH查询AAAA记录失败: %v\n", err)
		} else if len(aaaaIPs) > 0 {
			allIPs = append(allIPs, aaaaIPs...)
			if *verbose {
				fmt.Printf("  查询到AAAA记录: %v\n", aaaaIPs)
			}
		}
	}

	// 如果没有查询到任何IP，返回错误
	if len(allIPs) == 0 {
		return nil, fmt.Errorf("未查询到任何IP记录")
	}

	return allIPs, nil
}

// 执行DoH查询
func performDoHQuery(msg *dns.Msg, dohURL string, dohIPs ...string) ([]string, error) {
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

	return ips, nil
}

// 测试HTTP/3连接
func testHTTP3Connection(testURL, hostHeader, targetIP string, port int, timeoutSec int) (
	bool, string, uint16, string, uint64, error) {

	ctx, cancel := context.WithTimeout(context.Background(), time.Duration(timeoutSec)*time.Second)
	defer cancel()

	// 使用H3实验传输器，返回可关闭的接口
	transport := h3_experiment.CreateHTTP3TransportWithIPGetter(func() (string, error) {
		return targetIP, nil
	})

	client := &http.Client{
		Transport: transport,
		Timeout:   time.Duration(timeoutSec) * time.Second,
	}

	req, err := http.NewRequestWithContext(ctx, "GET", testURL, nil)
	if err != nil {
		transport.Close() // 关闭传输器
		return false, "none", 0, "", 0, err
	}

	req.Header.Set("Host", hostHeader)
	req.Header.Set("User-Agent", "curl/8.12.1")

	start := time.Now()
	resp, err := client.Do(req)
	if err != nil {
		transport.Close() // 关闭传输器
		return false, "h3", 0, "", 0, err
	}
	defer resp.Body.Close()

	latencyMs := uint64(time.Since(start).Milliseconds())
	statusCode := uint16(resp.StatusCode)
	serverHeader := resp.Header.Get("Server")

	// 成功后也要关闭传输器
	defer transport.Close()

	return (resp.StatusCode < 300 && resp.StatusCode >= 200), "h3", statusCode, serverHeader, latencyMs, nil
}

// 测试HTTP/2连接
func testHTTP2Connection(testURL, hostHeader, targetIP string, port int, timeoutSec int) (
	bool, string, uint16, string, uint64, error) {

	ctx, cancel := context.WithTimeout(context.Background(), time.Duration(timeoutSec)*time.Second)
	defer cancel()

	targetAddr := fmt.Sprintf("[%s]:%d", targetIP, port)
	if !strings.Contains(targetIP, ":") {
		targetAddr = fmt.Sprintf("%s:%d", targetIP, port)
	}

	dialer := &net.Dialer{
		Timeout: time.Duration(timeoutSec) * time.Second,
	}

	conn, err := dialer.Dial("tcp", targetAddr)
	if err != nil {
		return false, "none", 0, "", 0, err
	}
	defer conn.Close()

	// 构建HTTP/2请求
	req, err := http.NewRequestWithContext(ctx, "GET", testURL, nil)
	if err != nil {
		return false, "none", 0, "", 0, err
	}

	req.Header.Set("Host", hostHeader)
	req.Header.Set("User-Agent", "curl/8.12.1")

	start := time.Now()
	client := &http.Client{
		Transport: &http.Transport{
			Dial: func(network, addr string) (net.Conn, error) {
				return conn, nil
			},
		},
		Timeout: time.Duration(timeoutSec) * time.Second,
	}

	resp, err := client.Do(req)
	if err != nil {
		return false, "h2", 0, "", 0, err
	}
	defer resp.Body.Close()

	latencyMs := uint64(time.Since(start).Milliseconds())
	statusCode := uint16(resp.StatusCode)
	serverHeader := resp.Header.Get("Server")

	return (resp.StatusCode < 300 && resp.StatusCode >= 200), "h2", statusCode, serverHeader, latencyMs, nil
}

// 保存结果到JSON文件
func saveResults(results []TestResult, filename string) error {
	jsonData, err := json.MarshalIndent(results, "", "  ")
	if err != nil {
		return err
	}

	return os.WriteFile(filename, jsonData, 0644)
}

// 辅助函数
func stringPtr(s string) *string {
	return &s
}

func uint16Ptr(n uint16) *uint16 {
	return &n
}

func uint64Ptr(n uint64) *uint64 {
	return &n
}

func filterResults(results []TestResult, success bool) []TestResult {
	var filtered []TestResult
	for _, r := range results {
		if r.Success == success {
			filtered = append(filtered, r)
		}
	}
	return filtered
}
