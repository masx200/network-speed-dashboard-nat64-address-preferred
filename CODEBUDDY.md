# CODEBUDDY.md

This file provides guidance to CodeBuddy Code when working with code in this
repository.

## Project Overview

This is a **network speed testing dashboard** for NAT64 address preferred and
Cloudflare preferred IP addresses. The project consists of three main
components:

1. **Vue.js Frontend Dashboard** - Visualizes and analyzes network connectivity
   test results
2. **Go-based HTTP/3 Testing Tools** - Multi-protocol HTTP testing toolkit for
   connectivity analysis
3. **NAT64/DNS64 Testing Tool** - Tests NAT64 service providers and DNS64
   synthesis

## Key Commands

### Frontend Development (Vue.js)

```bash
# Install dependencies
npm install

# Start development server (http://localhost:5173)
npm run dev

# Type checking
npm run type-check

# Build for production
npm run build

# Preview production build
npm run preview
```

### Go Testing Tools

```bash
# HTTP/3 connectivity testing with DoH
go run host_connectivity_check.go -verbose -dohip 104.21.95.9 --doh-url https://deno-dns-over-https-server.g18uibxgnb.de5.net/

# NAT64/DNS64 service testing
go run nat64.go -verbose -services nat64-services.json -targets test-targets.json

# Extract hosts from test results
go run extract_hosts.go

# Run full test pipeline
npm start  # Runs ip-info.js then generate-test-report.js
```

### Utility Scripts

```bash
# Fetch Cloudflare IPs
npm run fetch

# Extract and merge IPs
npm run extract

# Clean old reports
npm run clean

# Merge connectivity results
npm run merge-results
```

## Architecture Overview

### Frontend Architecture

- **Main Component**: `src/components/SpeedDashboard.vue` - Single-file
  component with all dashboard logic
- **Data Loading**: Uses Vite's `import.meta.glob` to dynamically load JSON test
  report files
- **State Management**: Local reactive refs and computed properties (no external
  state library)
- **UI Framework**: Vuetify 3 with Material Design components
- **PWA Support**: Service worker with offline caching via vite-plugin-pwa

### Go Testing Tools

| File                         | Purpose                                                            |
| ---------------------------- | ------------------------------------------------------------------ |
| `main.go`                    | HTTP/3 testing client with DNS resolution and protocol negotiation |
| `host_connectivity_check.go` | Core connectivity testing with DoH/DoQ/DoT support                 |
| `nat64.go`                   | NAT64/DNS64 service provider testing tool                          |
| `extract_hosts.go`           | Utility for extracting host information from test results          |

**Protocol Stack**: HTTP/3 (via QUIC) -> HTTP/2 -> HTTP/1.1 with automatic
fallback

**DNS Resolution Modes**: RFC 8484 DoH, traditional DNS, and direct IP modes

### NAT64 Testing Flow

1. Query A record via DoH to get IPv4 address
2. Synthesize IPv6 address using NAT64 prefix (prefix:96 + IPv4:4 bytes)
3. Test HTTP connection through synthesized IPv6 address
4. Report success/failure with latency metrics

## Configuration Files

| File                  | Purpose                                                     |
| --------------------- | ----------------------------------------------------------- |
| `nat64-services.json` | NAT64 service provider configurations (prefixes, locations) |
| `test-targets.json`   | Test target hosts for NAT64 testing                         |
| `hosts.json`          | Input hosts for connectivity testing                        |
| `config.json`         | General configuration                                       |

## Data File Locations

- Test result files (`failed-test-report-*.json`) are in the **project root
  directory**
- Frontend accesses them via `../../failed-test-report-*.json` path pattern
- Connectivity results: `connectivity_results-*.json`
- NAT64 test reports: `nat64-test-report-*.json`

## Test Report Format

```json
{
  "report_info": { "generated_at": "timestamp" },
  "test_environment": {
    "ip_info": { "country": "Country", "asn": "AS12345", "as_name": "ISP Name" }
  },
  "top_latency_records": [...],  // Successful tests
  "failed_tests": [...]          // Failed tests
}
```

## NAT64 Test Result Fields

```json
{
  "timestamp": "RFC3339 timestamp",
  "service_provider": "Provider name",
  "service_location": "Geographic location",
  "nat64_prefix": "IPv6/96 prefix",
  "target_host": "Test target hostname",
  "original_ipv4": "Resolved IPv4",
  "synthesized_ipv6": "NAT64 synthesized address",
  "dns64_success": true/false,
  "connect_success": true/false,
  "dns64_latency_ms": 0,
  "connect_latency_ms": 0
}
```

## Vite Configuration Notes

- Port: 5173 in development
- Build optimization: Terser removes console.log in production
- Alias: `@` points to `src/` directory
- PWA: Auto-update and caching strategies configured

## Adding New Filters to Dashboard

1. Add filter field to `filters.value` object in `SpeedDashboard.vue`
2. Add UI component in the filter section
3. Update `clearFilters()` function to reset new field
4. Implement filter logic in `applyFilters()` function
