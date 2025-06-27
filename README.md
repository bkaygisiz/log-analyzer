# Log Analyzer

A fast and efficient log analysis tool written in Rust for parsing and analyzing web server logs.

## 🚀 Features

- **Daily Log Aggregation** - Groups log entries by day for better analysis
- **HTTP Status Code Analysis** - Tracks 2xx, 3xx, 4xx, and 5xx responses
- **Error Rate Calculation** - Automatically calculates error rates
- **Clean Output Format** - Beautiful, organized console output
- **Memory Efficient** - Processes logs line by line without loading entire files
- **Fast Performance** - Built with Rust for optimal speed

## 📋 Requirements

- Rust 1.70 or higher
- Cargo (comes with Rust)

## 🛠️ Installation

1. **Clone the repository:**
   ```bash
   git clone <repository-url>
   cd log-analyzer
   ```

2. **Build the project:**
   ```bash
   cargo build --release
   ```

3. **Run the analyzer:**
   ```bash
   ./target/release/log-analyzer <log-file-path>
   ```

## 📖 Usage

### Basic Usage

```bash
# Analyze a single log file
./target/release/log-analyzer server.log

# Or using cargo run
cargo run -- server.log
```

### Expected Log Format

The analyzer expects logs in Common Log Format (CLF) or Extended Log Format (ELF):

```
192.168.0.10 - - [24/Jun/2025:08:14:32 +0000] "GET /api/ping HTTP/1.1" 200 153 "-" "curl/7.74.0" 0.003
```

**Format breakdown:**
- IP Address
- User identity (usually `-`)
- User ID (usually `-`)
- Timestamp in `[dd/MMM/yyyy:HH:mm:ss +zzzz]` format
- HTTP request in quotes
- HTTP status code
- Response size in bytes
- Referrer (usually `-`)
- User agent
- Response time

### Output Example

```
==================================================
📊 LOG ANALYSIS SUMMARY
==================================================

📈 OVERALL STATISTICS:
  • Total Requests: 1500
  • 2xx Responses:  1200
  • 3xx Responses:  50
  • 4xx Responses:  200
  • 5xx Responses:  50
  • Error Rate:     13.33%

📅 DAILY BREAKDOWN:
  Day 1: 24/Jun/2025
    └─ Requests: 500 | 2xx: 400 | 3xx: 20 | 4xx: 60 | 5xx: 20 | Error Rate: 12.00%
  Day 2: 25/Jun/2025
    └─ Requests: 1000 | 2xx: 800 | 3xx: 30 | 4xx: 140 | 5xx: 30 | Error Rate: 14.00%

==================================================
```

## 🏗️ Project Structure

```
src/
├── main.rs              # CLI interface and main entry point
└── analyzer/            # Core analysis logic
    ├── mod.rs          # Module exports
    ├── analyze.rs      # Log analysis algorithms
    └── types.rs        # Data structures and result types
```

## 🔧 Development

### Building for Development

```bash
# Debug build
cargo build

# Run with debug output
cargo run -- server.log
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture
```

### Code Formatting

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check
```

### Linting

```bash
# Run clippy linter
cargo clippy

# Run clippy with all warnings
cargo clippy -- -W clippy::all
```

### Debug Mode

Run with debug output to see detailed processing information:

```bash
RUST_LOG=debug cargo run -- server.log
```

## 🚀 Future Enhancements

- [ ] Support for multiple log formats
- [ ] Real-time log monitoring
- [ ] JSON/CSV output formats
- [ ] Parallel processing for large files
- [ ] Web interface
- [ ] Alerting capabilities
- [ ] Custom filtering options

---

**Built with ❤️ in Rust** 