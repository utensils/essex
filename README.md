# Essex Boilerplate for Docker Based Projects

A Docker project template generator that helps create consistent and well-structured Docker projects.

## Features

- Generate Docker projects from templates
- Consistent project structure with best practices
- Makefile-driven workflow
- OCI-compliant labels
- Templates using Tera (Jinja2-like syntax)

## Installation

### From Source

```bash
cargo install --path .
```

## Usage

```bash
# List available templates
essex list

# Create a new project
essex new basic namespace/project-name --username your-username --vendor "Your Company"
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