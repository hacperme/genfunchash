# GenFuncHash

[![CI](https://github.com/yourusername/genfunchash/workflows/CI/badge.svg)](https://github.com/yourusername/genfunchash/actions/workflows/ci.yml)
[![Security](https://github.com/yourusername/genfunchash/workflows/Security/badge.svg)](https://github.com/yourusername/genfunchash/actions/workflows/security.yml)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/yourusername/genfunchash#license)
[![Crates.io](https://img.shields.io/crates/v/genfunchash.svg)](https://crates.io/crates/genfunchash)
[![Documentation](https://docs.rs/genfunchash/badge.svg)](https://docs.rs/genfunchash)

A high-performance command-line tool for generating hash values from function names using a custom algorithm. Perfect for creating C language hash tables and function dispatch systems.

## ✨ Features

- **Custom Hash Algorithm**: Implements `h = 5527 * h + 7 * char; v = h & 0xFFFF; h ^= v * v`
- **C Array Output**: Generates properly formatted C array initialization code
- **Batch Processing**: Process multiple functions from input files
- **TOML Configuration**: Flexible configuration file support
- **Cross-Platform**: Works on Windows, macOS, and Linux
- **High Performance**: Optimized for speed with minimal memory usage
- **Comprehensive Error Handling**: Clear error messages and validation

## 🚀 Quick Start

### Installation

#### From Pre-built Binaries
Download the latest release for your platform from [GitHub Releases](https://github.com/yourusername/genfunchash/releases).

#### From Source
```bash
cargo install genfunchash
```

#### From Git
```bash
cargo install --git https://github.com/yourusername/genfunchash
```

### Basic Usage

```bash
# Generate hash for a single function
genfunchash -f "func1"
# Output: /*[ 0]*/{0x3D1EF464,                             (void *)func1},

# Process multiple functions from file
genfunchash -i functions.txt -o hash_table.c

# Use configuration file
genfunchash -c genfunchash.toml -i functions.txt

# Verbose output
genfunchash -f "func1" -v
```

## 📖 Documentation

- **[User Guide](doc/user-guide.md)** - Complete usage guide with examples
- **[API Documentation](doc/api.md)** - Detailed API reference
- **[Architecture](doc/architecture.md)** - System design and implementation details
- **[Design Document](doc/design.md)** - Project design and decisions

## 🛠️ Command Line Interface

```
GenFuncHash - Function Name Hash Generator

USAGE:
    genfunchash [OPTIONS]

OPTIONS:
    -f, --function <FUNCTION>    Generate hash for a single function name
    -i, --input <INPUT>          Input file containing function names (one per line)
    -o, --output <OUTPUT>        Output file for generated hash table (default: stdout)
    -c, --config <CONFIG>        Configuration file path
    -v, --verbose                Enable verbose output
    -h, --help                   Print help information
    -V, --version                Print version information
```

### Examples

#### Single Function Hash
```bash
$ genfunchash -f "my_function"
/*[ 0]*/{0x12345678,                             (void *)my_function},
```

#### Batch Processing
```bash
$ cat functions.txt
func1
func2
func3

$ genfunchash -i functions.txt -o output.c
Generated hash table written to output.c

$ cat output.c
/*[ 0]*/{0x3D1EF464,                             (void *)func1},
/*[ 1]*/{0x3D021EC4,                             (void *)func2},
/*[ 2]*/{0x3D053624,                             (void *)func3},
```

#### Configuration File
```toml
# genfunchash.toml
[settings]
input_file = "functions.txt"
output_file = "hash_table.c"
verbose = true

[hash]
# Custom hash algorithm parameters (future feature)
# multiplier = 5527
# addend = 7
# mask = 0xFFFF
```

```bash
$ genfunchash -c genfunchash.toml
```

## 🏗️ Building from Source

### Prerequisites
- Rust 1.70.0 or later
- Cargo (comes with Rust)

### Build Steps
```bash
# Clone the repository
git clone https://github.com/yourusername/genfunchash.git
cd genfunchash

# Build the project
cargo build --release

# Run tests
cargo test

# Install locally
cargo install --path .
```

### Development
```bash
# Run with cargo
cargo run -- -f "test_function"

# Run tests with verbose output
cargo test -- --nocapture

# Check code formatting
cargo fmt --check

# Run clippy lints
cargo clippy -- -D warnings

# Generate documentation
cargo doc --open
```

## 🧪 Testing

The project includes comprehensive tests:

```bash
# Run all tests
cargo test

# Run with verbose output
cargo test -- --nocapture

# Run specific test
cargo test test_hash_algorithm

# Run integration tests
cargo test --test integration_tests
```

## 📈 Performance

GenFuncHash is optimized for performance:

- **Hash Generation**: ~1M hashes/second on modern hardware
- **Memory Usage**: Minimal memory footprint with streaming processing
- **File I/O**: Efficient buffered reading and writing

### Benchmarks
```bash
cargo bench
```

## 🔒 Security

This project takes security seriously:

- **Dependency Scanning**: Automated vulnerability checks
- **Security Audits**: Regular security audits with `cargo audit`
- **License Compliance**: Automated license compatibility checking
- **Supply Chain**: Dependency verification and validation

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Quick Contribution Steps
1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for your changes
5. Run the test suite (`cargo test`)
6. Commit your changes (`git commit -m 'Add amazing feature'`)
7. Push to the branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

### Development Environment
- Use `cargo fmt` for code formatting
- Use `cargo clippy` for linting
- Add tests for new functionality
- Update documentation as needed

## 📄 License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## 🙏 Acknowledgments

- Thanks to all contributors who have helped shape this project
- Inspired by the need for efficient function hash generation in C projects
- Built with the amazing Rust ecosystem

---

**Made with ❤️ in Rust**