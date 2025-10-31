# Contributing to GenFuncHash

Thank you for your interest in contributing to GenFuncHash! We welcome contributions of all kinds, from bug reports and feature requests to code improvements and documentation updates.

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Environment](#development-environment)
- [Making Changes](#making-changes)
- [Testing](#testing)
- [Submitting Changes](#submitting-changes)
- [Code Style](#code-style)
- [Documentation](#documentation)
- [Community](#community)

## 🤝 Code of Conduct

This project and everyone participating in it is governed by our Code of Conduct. By participating, you are expected to uphold this code. Please report unacceptable behavior to the project maintainers.

### Our Standards

- Use welcoming and inclusive language
- Be respectful of differing viewpoints and experiences
- Gracefully accept constructive criticism
- Focus on what is best for the community
- Show empathy towards other community members

## 🚀 Getting Started

### Prerequisites

- Rust 1.70.0 or later
- Git
- A GitHub account

### Setting Up Your Development Environment

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR-USERNAME/genfunchash.git
   cd genfunchash
   ```
3. **Add the upstream remote**:
   ```bash
   git remote add upstream https://github.com/ORIGINAL-OWNER/genfunchash.git
   ```
4. **Install dependencies and build**:
   ```bash
   cargo build
   ```
5. **Run tests to ensure everything works**:
   ```bash
   cargo test
   ```

## 🛠️ Development Environment

### Recommended Tools

- **IDE**: VS Code with rust-analyzer extension
- **Formatting**: `cargo fmt` (runs automatically on save with rust-analyzer)
- **Linting**: `cargo clippy` for additional lints
- **Documentation**: `cargo doc` for generating docs

### Useful Commands

```bash
# Build the project
cargo build

# Run in debug mode
cargo run -- -f "test_function"

# Run tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Format code
cargo fmt

# Run clippy lints
cargo clippy -- -D warnings

# Generate and open documentation
cargo doc --open

# Run benchmarks
cargo bench
```

## 🔄 Making Changes

### Workflow

1. **Create a branch** for your changes:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** following our coding standards

3. **Add tests** for your changes

4. **Update documentation** if needed

5. **Test your changes**:
   ```bash
   cargo test
   cargo clippy -- -D warnings
   cargo fmt --check
   ```

6. **Commit your changes**:
   ```bash
   git add .
   git commit -m "Add your descriptive commit message"
   ```

### Commit Message Guidelines

We follow conventional commits format:

- `feat:` for new features
- `fix:` for bug fixes
- `docs:` for documentation changes
- `test:` for test additions or modifications
- `refactor:` for code refactoring
- `perf:` for performance improvements
- `chore:` for maintenance tasks

Examples:
```
feat: add batch processing mode
fix: resolve hash collision edge case
docs: update API documentation
test: add integration tests for CLI
```

## 🧪 Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with verbose output
cargo test -- --nocapture

# Run integration tests only
cargo test --test integration_tests
```

### Writing Tests

- Add unit tests in the same file as the code being tested
- Add integration tests in the `tests/` directory
- Ensure all new functionality is tested
- Test both success and error cases

Example unit test:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_algorithm() {
        let hasher = CustomHasher::new();
        let result = hasher.compute_hash("func1");
        assert_eq!(result, 0x3D1EF464);
    }
}
```

## 📤 Submitting Changes

### Before Submitting

1. **Sync with upstream**:
   ```bash
   git fetch upstream
   git rebase upstream/master
   ```

2. **Run the full test suite**:
   ```bash
   cargo test
   cargo clippy -- -D warnings
   cargo fmt --check
   ```

3. **Update documentation** if your changes affect the public API

4. **Update CHANGELOG.md** if your changes are user-facing

### Creating a Pull Request

1. **Push your branch** to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```

2. **Create a Pull Request** on GitHub with:
   - Clear title describing the change
   - Detailed description of what was changed and why
   - Reference to any related issues
   - Screenshots if applicable (for UI changes)

3. **Address review feedback** promptly and respectfully

### Pull Request Template

When creating a PR, please fill out our template:

- [ ] I have read the contributing guidelines
- [ ] I have added tests for my changes
- [ ] All tests pass locally
- [ ] I have updated documentation as needed
- [ ] I have added an entry to CHANGELOG.md (if applicable)

## 🎨 Code Style

### Rust Style Guidelines

We follow the standard Rust style guidelines:

- Use `cargo fmt` for formatting
- Follow Rust naming conventions:
  - `snake_case` for functions and variables
  - `PascalCase` for types and traits
  - `SCREAMING_SNAKE_CASE` for constants
- Write clear, self-documenting code
- Add comments for complex logic
- Use meaningful variable and function names

### Documentation Comments

Use Rust's documentation comments (`///`) for public APIs:

```rust
/// Computes a hash value for the given function name.
/// 
/// Uses the custom algorithm: h = 5527 * h + 7 * char; v = h & 0xFFFF; h ^= v * v
/// 
/// # Arguments
/// 
/// * `function_name` - The name of the function to hash
/// 
/// # Returns
/// 
/// A 32-bit hash value
/// 
/// # Examples
/// 
/// ```
/// let hasher = CustomHasher::new();
/// let hash = hasher.compute_hash("func1");
/// assert_eq!(hash, 0x3D1EF464);
/// ```
pub fn compute_hash(&self, function_name: &str) -> u32 {
    // Implementation here
}
```

## 📚 Documentation

### Types of Documentation

1. **API Documentation**: Generated from code comments
2. **User Guide**: `doc/user-guide.md`
3. **Architecture**: `doc/architecture.md`
4. **README**: Project overview and quick start

### Updating Documentation

- Update API docs when changing public interfaces
- Update user guide for new features or changed behavior
- Update README for significant changes
- Ensure all examples in documentation work correctly

## 💬 Community

### Getting Help

- **GitHub Issues**: For bug reports and feature requests
- **GitHub Discussions**: For questions and general discussion
- **Documentation**: Check our comprehensive docs first

### Reporting Issues

When reporting issues, please include:

- GenFuncHash version
- Operating system and version
- Rust version
- Clear description of the problem
- Steps to reproduce
- Expected vs actual behavior
- Any relevant error messages

### Feature Requests

When requesting features:

- Check if it already exists or has been requested
- Provide clear use case and motivation
- Consider implementation complexity
- Be open to alternative solutions

## 🏆 Recognition

Contributors are recognized in:

- `CHANGELOG.md` for significant contributions
- GitHub contributor statistics
- Special thanks in release notes

## 📝 License

By contributing to GenFuncHash, you agree that your contributions will be licensed under the same terms as the project (MIT OR Apache-2.0).

---

Thank you for contributing to GenFuncHash! 🎉