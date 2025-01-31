# Contributing to Essex

Thank you for your interest in contributing to Essex! This document provides guidelines and setup instructions for contributing to the project.

## Development Setup

### Rust Toolchain Requirements
- Rust version: 1.83.0 or later
- Use stable toolchain features only

### Code Style and Linting

#### Rustfmt Configuration
We use a `.rustfmt.toml` with stable-only features. The configuration is:
```toml
max_width = 100
hard_tabs = false
tab_spaces = 4
newline_style = "Auto"
use_small_heuristics = "Default"
reorder_imports = true
reorder_modules = true
remove_nested_parens = true
edition = "2021"
merge_derives = true
use_field_init_shorthand = true
use_try_shorthand = true
force_explicit_abi = true
```

#### Clippy Configuration
Run clippy with these specific flags (via RUSTFLAGS):
```bash
RUSTFLAGS="-D warnings -D clippy::redundant-pattern-matching -D clippy::needless-borrows-for-generic-args"
```

## Development Workflow

### Before Submitting Changes
1. Run `cargo fmt --all -- --check` to verify formatting
2. Run clippy with required flags:
   ```bash
   RUSTFLAGS="-D warnings -D clippy::redundant-pattern-matching -D clippy::needless-borrows-for-generic-args" cargo clippy
   ```
3. Run `cargo test` to ensure all tests pass

### Pull Request Process
1. Create a new branch for your changes
2. Make your changes following the code style guidelines
3. Update documentation as needed
4. Submit a pull request
5. Ensure CI checks pass

## CI/CD Configuration
Our CI environment uses:
- Rust toolchain: stable (1.83.0)
- Same rustfmt configuration via .rustfmt.toml
- Same clippy flags via RUSTFLAGS environment variable

## Additional Resources
- [Rust Style Guide](https://rust-lang.github.io/api-guidelines/)
- [Clippy Documentation](https://rust-lang.github.io/rust-clippy/)
- [Rustfmt Documentation](https://rust-lang.github.io/rustfmt/)
