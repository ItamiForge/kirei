# Contributing to Kirei

Thank you for considering contributing to Kirei! This document provides guidelines and information for contributors.

## Code of Conduct

Be respectful, inclusive, and professional. We're here to build great software together.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check existing issues to avoid duplicates. When creating a bug report, include:

- **Clear title and description**
- **Steps to reproduce** the behavior
- **Expected vs actual behavior**
- **System information** (macOS version, Rust version)
- **Logs or screenshots** if applicable

### Suggesting Enhancements

Enhancement suggestions are welcome! Please include:

- **Clear description** of the feature
- **Use cases** and why it would be useful
- **Potential implementation approach** (optional)

### Pull Requests

1. **Fork the repo** and create your branch from `main`
2. **Write clear commit messages** following conventional commits
3. **Add tests** if applicable
4. **Update documentation** as needed
5. **Ensure CI passes** (tests, clippy, formatting)

## Development Setup

### Prerequisites

- macOS 10.15 (Catalina) or later
- Rust 1.70 or later
- Git

### Getting Started

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/kirei.git
cd kirei

# Build the project
cargo build

# Run the CLI
cargo run

# Run tests
cargo test
```

### Code Style

We use Rust's standard formatting and linting tools:

```bash
# Format code
cargo fmt

# Check formatting without modifying files
cargo fmt --check

# Run clippy lints
cargo clippy --all-targets --all-features -- -D warnings
```

**Before submitting a PR, ensure:**
- Code is formatted with `cargo fmt`
- All clippy warnings are resolved
- Tests pass with `cargo test`

## Project Structure

```
kirei/
├── src-core/          # Core Rust library and CLI
│   ├── lib.rs         # Library exports
│   ├── main.rs        # CLI entry point
│   ├── scanner/       # File system scanning logic
│   ├── cleaner.rs     # Trash operations
│   ├── tui/           # Terminal UI components
│   └── config.rs      # Configuration handling
├── src-tauri/         # Tauri desktop app (work in progress)
├── src-gui/           # Web UI frontend (work in progress)
└── .github/           # CI/CD workflows
```

## Commit Messages

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add support for Homebrew cache cleaning
fix: correct permission handling for system logs
docs: update installation instructions
chore: update dependencies
test: add tests for scanner module
```

## Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run a specific test
cargo test test_name
```

## Areas for Contribution

Here are some areas where contributions are especially welcome:

- **Scanner Improvements**: Add support for more developer tools (Docker, Python venvs, etc.)
- **Performance**: Optimize scanning and cleaning operations
- **Safety Features**: Enhance file protection and recovery options
- **Documentation**: Improve user guides and code documentation
- **Testing**: Increase test coverage
- **macOS Compatibility**: Ensure support across different macOS versions
- **GUI Development**: Help build out the Tauri desktop app

## Questions?

Feel free to:
- Open an issue for discussion
- Check the [documentation](https://itamiforge.github.io/itamiforge/docs/projects/kirei)
- Review existing issues and PRs

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

**Thank you for helping make Kirei better!** 🙏
