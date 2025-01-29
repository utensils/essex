# {{ image_name }}

Docker image for {{ image_name }}.

## About

This is a Docker image for {{ image_name }}.

## Usage

```bash
docker run {{ repo_namespace }}/{{ image_name }}
```

## Building

This project uses a `Makefile` to build and test the image:

```bash
make        # Build the image
make test   # Run tests
make push   # Push to registry
```

## License

MIT License