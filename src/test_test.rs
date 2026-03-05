#[cfg(test)]
mod tests {
    use base64::{engine::general_purpose, Engine as _};
    use reqwest;
    use std::net::IpAddr;
    use trust_dns_proto::op::{Message, Query};
    use trust_dns_proto::rr::{Name, RecordType};
    use trust_dns_proto::serialize::binary::BinEncodable;

    // DoH 服务器 URL
    const DOH_SERVER: &str =
        "https://deno-dns-over-https-server.g18uibxgnb.de5.net/";

    // 要查询的域名
    const DOMAIN_TO_QUERY: &str = "hello-world-deno-deploy.a1u06h9fe9y5bozbmgz3.qzz.io";

    // 期望的 IPv4 地址
    const EXPECTED_IPV4: [&str; 2] = ["104.21.33.118", "172.67.162.86"];

    // 期望的 IPv6 地址
    const EXPECTED_IPV6: [&str; 2] = ["2606:4700:3030::ac43:a256", "2606:4700:3031::6815:2176"];

    #[tokio::test]
    async fn test_doh_domain_resolution() {
        // 创建 HTTP 客户端
        let client = reqwest::Client::builder()
            .user_agent("rust-doh-test/1.0")
            .build()
            .expect("Failed to build HTTP client");

        // 查询 A 记录 (IPv4)
        let ipv4_addresses = query_dns_record(&client, DOMAIN_TO_QUERY, RecordType::A).await;

        // 查询 AAAA 记录 (IPv6)
        let ipv6_addresses = query_dns_record(&client, DOMAIN_TO_QUERY, RecordType::AAAA).await;

        // 验证结果
        assert!(!ipv4_addresses.is_empty(), "No IPv4 addresses found");
        assert!(!ipv6_addresses.is_empty(), "No IPv6 addresses found");

        // 检查是否包含所有期望的 IPv4 地址
        for expected_ip in EXPECTED_IPV4.iter() {
            let ip_addr: IpAddr = expected_ip
                .parse()
                .expect("Invalid IPv4 address in expected list");
            assert!(
                ipv4_addresses.contains(&ip_addr),
                "Expected IPv4 address {} not found in response",
                expected_ip
            );
        }

        // 检查是否包含所有期望的 IPv6 地址
        for expected_ip in EXPECTED_IPV6.iter() {
            let ip_addr: IpAddr = expected_ip
                .parse()
                .expect("Invalid IPv6 address in expected list");
            assert!(
                ipv6_addresses.contains(&ip_addr),
                "Expected IPv6 address {} not found in response",
                expected_ip
            );
        }

        println!("Successfully resolved domain with expected IP addresses:");
        for ip in ipv4_addresses.iter().chain(ipv6_addresses.iter()) {
            println!("  {}", ip);
        }
    }

    async fn query_dns_record(
        client: &reqwest::Client,
        domain: &str,
        record_type: RecordType,
    ) -> Vec<IpAddr> {
        // 创建 DNS 查询
        let name = Name::from_ascii(domain).expect("Invalid domain name");
        let query = Query::query(name, record_type);

        // 创建 DNS 消息
        let mut message = Message::new();
        message.set_id(0); // RFC 8484 建议使用 ID 为 0 以提高缓存效率
        message.set_recursion_desired(true);
        message.add_query(query);

        // 序列化 DNS 查询
        let mut request_bytes = Vec::new();
        {
            let mut encoder =
                trust_dns_proto::serialize::binary::BinEncoder::new(&mut request_bytes);
            message
                .emit(&mut encoder)
                .expect("Failed to serialize DNS query");
        }

        // 使用 base64url 编码（不包含填充）
        let encoded_query = general_purpose::URL_SAFE_NO_PAD.encode(&request_bytes);

        // 构建 DoH 请求 URL
        let url = format!("{}?dns={}", DOH_SERVER, encoded_query);

        // 发送 HTTPS GET 请求
        let response = client
            .get(&url)
            .header("Accept", "application/dns-message")
            .send()
            .await
            .expect("Failed to send DoH request");

        // 检查响应状态
        assert_eq!(
            response.status(),
            reqwest::StatusCode::OK,
            "DoH server returned non-200 status: {}",
            response.status()
        );

        // 获取响应体
        let response_bytes = response
            .bytes()
            .await
            .expect("Failed to read response body");

        // 解析 DNS 响应
        let dns_response =
            Message::from_vec(&response_bytes).expect("Failed to parse DNS response");

        // 提取 IP 地址
        let mut ip_addresses = Vec::new();

        let answers = dns_response.answers();
        if !answers.is_empty() {
            for record in answers {
                if record.record_type() == record_type {
                    if let Some(rdata) = record.data() {
                        match record.record_type() {
                            RecordType::A => {
                                if let trust_dns_proto::rr::RData::A(ipv4) = rdata {
                                    ip_addresses.push(IpAddr::V4(*ipv4));
                                }
                            }
                            RecordType::AAAA => {
                                if let trust_dns_proto::rr::RData::AAAA(ipv6) = rdata {
                                    ip_addresses.push(IpAddr::V6(*ipv6));
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        ip_addresses
    }
}
