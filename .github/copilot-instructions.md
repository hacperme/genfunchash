# GenFuncHash - AI Coding Agent Instructions

## Project Overview
这个项目是一个名为 GenFuncHash 的命令行工具，用于根据函数名字生成函数的哈希值，使用rust语言实现。

## Development Setup
- This is a fresh Git repository on the `master` branch
- Rust project (use `cargo init` to initialize)
- Target: Command-line tool for function name hashing

## Key Patterns & Conventions
- Use `clap` for command-line argument parsing
- Implement custom hash algorithm: `h = 5527 * h + 7 * char; v = h & 0xFFFF; h ^= v * v`
- Follow Rust naming conventions: snake_case for functions/variables, PascalCase for types
- Use `Result<T, E>` for error handling throughout the application

## Architecture Notes
- CLI entry point in `src/main.rs`
- Custom hash generation logic in `src/hash/custom.rs`
- Function name parsing and validation layer
- Single optimized hash algorithm for function names

## Critical Workflows
### Initial Project Setup
```bash
cargo init
cargo add clap --features derive
# No external hash crates needed - using custom algorithm
```

### Development Process
- Use `cargo check` for fast compilation checking
- Use `cargo test` for running unit tests
- Use `cargo clippy` for linting
- Use `cargo fmt` for code formatting
- Build with `cargo build --release` for production

### Testing Strategy
- Unit tests for custom hash algorithm correctness
- Integration tests for CLI argument parsing and C array output
- Property-based testing for hash consistency
- Output format validation tests

## Integration Points
- `clap` for CLI argument parsing and help generation
- Custom hash algorithm implementation (no external hash crates)
- Standard library `std::collections::HashMap` for result caching

## Important Files & Directories
- `Cargo.toml` - Project dependencies and metadata
- `src/main.rs` - CLI entry point with argument parsing
- `src/hash/custom.rs` - Custom hash generation logic
- `src/formatter.rs` - C array output formatting
- `src/lib.rs` - Core library functions
- `tests/` - Integration tests
- `.git/` - Git repository metadata

## Output Format
Generate C language array initialization format:
```c
/*[index]*/{0xHashValue,                             (void *)function_name},
```

Example with actual hash values:
```c
/*[ 0]*/{0x3D1EF464,                             (void *)func1},
/*[ 1]*/{0x3D021EC4,                             (void *)func2},
```

---
*This file should be updated as the project structure and patterns emerge.*