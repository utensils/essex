# Essex Boilerplate for Docker Based Projects

A Docker project template generator that helps create consistent and well-structured Docker projects.

## Features

- Generate Docker projects from templates
- Consistent project structure with best practices
- Makefile-driven workflow
- OCI-compliant labels
- Templates using Tera (Jinja2-like syntax)
- Shell completion support (bash, zsh)

## Installation

### Quick Install (Linux and macOS)

```bash
curl -fsSL -o essex-install.sh https://github.com/utensils/essex/releases/latest/download/download_cli.sh && bash essex-install.sh
```

This script will:
- Detect your OS and architecture
- Download the appropriate binary
- Install it to `~/.local/bin`
- Provide instructions for adding to your PATH

The installer supports:
- macOS (Apple Silicon and Intel)
- Linux (x86_64 and ARM64)

Note: You may need to add `~/.local/bin` to your PATH if you haven't already:
```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc  # or restart your terminal
```

### From Source

```bash
cargo install --path .
```

### Shell Completion

Essex supports command-line completion for bash and zsh shells. You can enable it by following these steps:

#### Bash

1. Generate and install the completion script:
```bash
# For current user
mkdir -p ~/.local/share/bash-completion/completions
essex completion bash > ~/.local/share/bash-completion/completions/essex.bash

# Or for system-wide installation (requires sudo)
sudo essex completion bash > /usr/share/bash-completion/completions/essex
```

2. Add to your `~/.bashrc`:
```bash
source ~/.local/share/bash-completion/completions/essex.bash
```

3. Reload your shell or source the file:
```bash
source ~/.bashrc
```

#### Zsh

1. Create the completion directory and generate the script:
```bash
mkdir -p ~/.zfunc
essex completion zsh > ~/.zfunc/_essex
```

2. Add to your `~/.zshrc`:
```zsh
fpath=(~/.zfunc $fpath)
autoload -Uz compinit
compinit
```

3. Reload your shell or source the file:
```bash
source ~/.zshrc
```

#### Testing Completion

After installation, you can test the completion by typing:
```bash
essex <TAB>           # Shows available commands (list, new, completion, help)
essex new <TAB>       # Shows available templates
essex completion <TAB> # Shows supported shells (bash, zsh)
```

## Usage

```bash
# List available templates
essex list

# Create a new project
essex new basic namespace/project-name --username your-username --vendor "Your Company"

# Generate shell completion
essex completion bash  # For bash
essex completion zsh   # For zsh
```

### Template Structure

The basic template includes:
- Dockerfile with best practices
- Makefile for common Docker operations
- README.md
- Entrypoint script
- OCI-compliant labels

### Project Structure

```
project-name/
├── Dockerfile
├── Makefile
├── README.md
└── runtime-assets/
    └── usr/
        └── local/
            └── bin/
                └── entrypoint.sh
```

## Development

### Requirements

- Rust 1.70 or later
- Cargo

### Building

```bash
cargo build --release
```

### Testing

```bash
cargo test                # Run Rust tests
./test_install.sh        # Run installer tests
```

### Release Process

The project follows semantic versioning and maintains synchronization between Cargo.toml versions and git tags.

To create a new release:

1. Use the version bump script:
```bash
./scripts/bump_version.sh 1.0.0  # Replace with your version
```

2. Review the changes:
```bash
git show HEAD
```

3. Push the changes:
```bash
git push origin main v1.0.0  # Replace with your version
```

The CI/CD pipeline will:
- Verify that Cargo.toml version matches the git tag
- Run all tests
- Build binaries for all supported platforms
- Generate SHA256 checksums
- Create a GitHub release
- Upload all assets

### Development Guidelines

- Follow Test-Driven Development (TDD) practices
  - Write tests first before implementing new features
  - Ensure each new feature or bug fix has corresponding tests
  - Keep test coverage high and meaningful
- All code changes should maintain backward compatibility with existing templates
- Follow Rust best practices and idiomatic code patterns
- Ensure all templates follow Docker best practices:
  - Use multi-stage builds where appropriate
  - Include proper OCI labels
  - Follow principle of least privilege (run as non-root user)
  - Include proper documentation
- Always ensure Cargo version matches git tag for releases

## License

MIT License