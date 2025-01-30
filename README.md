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
cargo test
```

## License

MIT License