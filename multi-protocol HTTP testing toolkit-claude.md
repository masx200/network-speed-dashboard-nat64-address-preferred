# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with
code in this repository.

## Project Overview

This is a **multi-protocol HTTP testing toolkit** focused on HTTP/3 connectivity
and Cloudflare performance testing. The project has been successfully migrated
from Rust to Go and includes two main components:

1. **HTTP/3 Testing Tool** (Root directory) - Primary testing client for HTTP/3
   connectivity
2. **Advanced HTTP/3 Reverse Proxy**
   (`http3-reverse-proxy-server-experiment-master/`) - Full-featured reverse
   proxy with load balancing

## Key Directories and Their Purposes

### Root Level (`/`)

- `main.go` - Main HTTP/3 testing client with DNS resolution, IP filtering, and
  protocol negotiation
- `go.mod` & `go.sum` - Go module dependencies
- `README.md` & `config.json` - Project documentation and configuration examples
- `CLAUDE.md` - This file

### HTTP/3 Reverse Proxy (`http3-reverse-proxy-server-experiment-master/`)

- `h3/` - HTTP/3 protocol implementation using QUIC
- `dns/` - DNS resolution (DoH, DoQ, DoT support)
- `load_balance/` - Load balancing system with health checks
- `adapter/` - HTTP transport adapters and round-trippers
- `print/` - Debug logging utilities
- `test/` - Test suite for proxy server

## Main Entry Points and Execution Flows

### 1. HTTP/3 Testing Tool (`main.go`)

```go
// Execution Flow:
1. Parse command-line flags (config file, domain, DoH URL, resolution mode)
2. Load test tasks from JSON config or use built-in defaults
3. For each task:
   - DNS resolution via DoH (RFC 8484), traditional DNS, or direct IP
   - IP filtering and validation (invalid IP exclusion)
   - HTTP/3 connection testing using experimental QUIC library
   - HTTP/2 fallback for failed HTTP/3 connections
4. Concurrent goroutine execution for multi-IP testing
5. JSON output with comprehensive results
```

### 2. HTTP/3 Reverse Proxy (`http3-reverse-proxy-server-experiment-master/main.go`)

```go
// Execution Flow:
1. Parse extensive configuration (ports, protocols, TLS, load balancing)
2. Setup Gin HTTP engine with middleware (loop detection, forwarding)
3. Start multiple protocol servers concurrently:
   - HTTP/3 server on HTTPS port
   - HTTPS/HTTP server
   - HTTP/1.1 and HTTP/2 servers
4. Handle requests with load balancing and health checking
```

## Dependencies and External Libraries

### Core HTTP/3 Dependencies

- `github.com/quic-go/quic-go` - HTTP/3 and QUIC protocol implementation
- `github.com/miekg/dns` - DNS library for DoH/DoQ/DoT support
- `github.com/masx200/doq-go` - DNS over QUIC implementation
- `github.com/masx200/http3-reverse-proxy-server-experiment` - Custom HTTP/3
  proxy library

### Secondary Dependencies

- `github.com/gin-gonic/gin` - HTTP web framework
- `golang.org/x/net` - Extended networking libraries
- `golang.org/x/crypto` - Cryptography support
- Various Cloudflare and ByteDance libraries for caching and utilities

## Configuration System and File Formats

### Command-Line Configuration

```bash
# Basic usage
./http3-test-tool.exe -domain "example.com" -verbose

# Configuration file usage
./http3-test-tool.exe -config "config.json" -verbose

# Available modes
-resolve-mode "https"    # DoH (RFC 8484) - default
-resolve-mode "a_aaaa"  # Traditional A/AAAA records
-resolve-mode "direct"    # Direct IP bypass

# Protocol selection
-test-url "https://target.com"  # Custom test URL
-port 8443                     # Custom port
```

### Configuration File Format

```json
[
  {
    "doh_resolve_domain": "hello-world-deno-deploy.a1u06h9fe9y5bozbmgz3.qzz.io",
    "test_sni_host": "hello-world-deno-deploy.a1u06h9fe9y5bozbmgz3.qzz.io",
    "test_host_header": "hello-world-deno-deploy.a1u06h9fe9y5bozbmgz3.qzz.io",
    "doh_url": "https://deno-dns-over-https-server.g18uibxgnb.de5.net/",
    "port": 443,
    "prefer_ipv6": false,
    "resolve_mode": "https", // "https", "a_aaaa", "direct"
    "direct_ips": ["1.1.1.1", "2606:4700:4700::1"] // direct mode only
  }
]
```

## Testing Approach and Build Processes

### Build Commands

```bash
# Build main testing tool
go build main.go

# Build proxy server (if needed)
cd http3-reverse-proxy-server-experiment-master
go build main.go

# Run tests
go test -v ./...

# Run with debug profiling
go run main.go -debug-pprof
```

### Test Structure

- Unit tests in `test/` directories for individual components
- Integration tests for end-to-end functionality
- Manual testing capabilities via command-line interface

### Key Testing Features

- Multi-protocol support (HTTP/1.1, HTTP/2, HTTP/3)
- IPv4/IPv6 address validation and filtering
- DNS resolution via multiple methods
- Connection pooling and rate limiting avoidance
- Comprehensive JSON output with metrics

## Architectural Decisions

### 1. Multi-Protocol Architecture

- **Design**: Simultaneous support for HTTP/1.1, HTTP/2, and HTTP/3
- **Implementation**: Protocol negotiation with fallback strategies
- **Benefits**: Maximum compatibility with different server capabilities

### 2. Advanced DNS Resolution

- **Primary**: RFC 8484-compliant DoH with HTTPS records
- **Fallback**: Traditional A/AAAA queries via hickory-dns
- **Bypass**: Direct IP specification for testing specific servers
- **Benefits**: Robust DNS resolution with multiple fallback options

### 3. HTTP/3 Implementation Strategy

- **Experimental**: Uses local `http3-reverse-proxy-server-experiment` library
- **Transport**: Custom QUIC transport with IP address binding
- **Fallback**: Automatic HTTP/2 and HTTP/1.1 fallback
- **Benefits**: Cutting-edge HTTP/3 with reliable fallbacks

### 4. Connection Management

- **Rotation**: IP rotation to avoid UDP rate limiting
- **Pooling**: Connection reuse for performance
- **Health Monitoring**: Active and passive health checks
- **Loop Detection**: Middleware to prevent infinite proxy chains

## Special Patterns and Conventions

### 1. Protocol-agnostic Design

The codebase maintains clean separation between protocol implementations,
allowing easy addition of new protocols or modification of existing ones.

### 2. Configuration Flexibility

Multiple configuration sources (CLI flags, JSON files) with sensible defaults
and extensive validation.

### 3. Error Handling Strategy

Comprehensive error handling with detailed logging and graceful fallbacks
between protocols.

### 4. Performance Optimization

Connection pooling, IP rotation, and concurrent execution for efficient testing.

## Development Workflow

### Common Development Tasks

#### Adding New Test Domains

1. Add domain to default configuration in `getDefaultTasks()`
2. Update configuration file examples in README.md
3. Test with different resolution modes

#### Implementing New DoH Providers

1. Add DoH URL template in `InputTask` structure
2. Ensure URL format supports proper query parameters
3. Test response format compatibility

#### Extending Protocol Support

1. Add protocol in HTTP/3 transport configuration
2. Implement ALPN negotiation for new protocol
3. Add performance tests and benchmarks

### Build and Deployment

#### Local Development

```bash
# Build with HTTP/3 support (requires experimental flag)
RUSTFLAGS='--cfg reqwest_unstable' cargo build --release

# Run with verbose output
./http3-test-tool.exe -domain "example.com" -verbose
```

#### Production Considerations

- Use TLS certificate validation
- Configure appropriate timeout settings
- Monitor connection pools and resource usage
- Use production DoH endpoints

## Deployment Notes

### Environment Variables

```bash
# Enable HTTP/3 experimental features
export RUSTFLAGS='--cfg reqwest_unstable'
```

### Security Considerations

- Validate DNS responses for integrity
- Use secure HTTP client configurations
- Exclude known invalid IP addresses
- Implement proper TLS certificate verification
