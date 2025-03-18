# Essex CLI Development Guide

## Commands
- Build: `cargo build --release`
- Install: `cargo install --path .`
- Test (all): `cargo test`
- Test (single): `cargo test -- test_name`
- Integration test: `./test_install.sh`
- Format: `cargo fmt --all`
- Lint: `RUSTFLAGS="-D warnings -D clippy::redundant-pattern-matching -D clippy::needless-borrows-for-generic-args" cargo clippy`

## Code Style
- **Formatting**: 100 char line width, 4 spaces indentation
- **Imports**: Group by module (std → external → project)
- **Error handling**: Custom Error enum with specific variants, uses thiserror/anyhow
- **Naming**: snake_case for functions/variables, CamelCase for types/enums
- **Testing**: TDD approach, unit tests with code, integration tests in tests/
- **Types**: Rust 2021 edition, strongly typed error handling

Always run the formatter and linter before committing changes.

## AI Agent Sync
When this file is modified, copy its contents to:
- .cursorrules
- .windsurfrules
- .goosehints 
This ensures all AI coding assistants have the same information.