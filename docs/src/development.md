# Development Guide

## Building from Source

### Prerequisites

- Rust and Cargo (latest stable version)
- [Ookla Speedtest CLI](https://www.speedtest.net/apps/cli) version 1.2.0 or higher

### Build Steps

```bash
# Clone the repository
git clone https://github.com/lpicanco/prometheus-speedtest-exporter.git
cd prometheus-speedtest-exporter

# Build in release mode
cargo build --release

# Run tests
cargo test

# Run linting
cargo clippy
```

## Building Documentation

The documentation is built using MkDocs with the Material theme and hosted on GitHub Pages.

### Directory Structure
```
docs/
├── mkdocs.yml          # MkDocs configuration
└── src/                # Documentation source files
    ├── index.md        # Home page
    ├── quick-start.md
    └── ...
```

### GitHub Pages Setup

1. Go to your repository's Settings
2. Navigate to "Pages" under "Code and automation" in the sidebar
3. Under "Build and deployment":
   - For "Source", select "Deploy from a branch"
   - For "Branch", select `gh-pages` and `/ (root)`
   - Click "Save"

After configuration, your documentation will be available at `https://<username>.github.io/<repository>`.

### Prerequisites

```bash
# Install Python dependencies
pip install mkdocs-material pillow cairosvg
```

### Local Development

```bash
# Start local server
cd docs
mkdocs serve
```

Visit `http://127.0.0.1:8000` to see the documentation.

### Building for Production

```bash
# Build static site
cd docs
mkdocs build
```

The built site will be in the `site` directory at the root of the project.

## Docker Build

### Local Build

```bash
# Build the image
docker build -t prometheus-speedtest-exporter .

# Run the container
docker run -p 9516:9516 prometheus-speedtest-exporter
```

### Multi-architecture Build

```bash
# Set up buildx
docker buildx create --use

# Build and push for multiple architectures
docker buildx build --platform linux/amd64,linux/arm64,linux/arm/v7 -t your-registry/prometheus-speedtest-exporter:latest .
```

## Release Process

1. Update version in `Cargo.toml`
2. Create and push a new tag:
   ```bash
   git tag -a v0.2.3 -m "Release v0.2.3"
   git push origin v0.2.3
   ```
3. The GitHub Actions workflow will automatically:
   - Create a GitHub release
   - Build and upload binaries for all supported architectures
   - Build and push Docker images
   - Deploy documentation updates

## Contributing

1. Fork the repository
2. Create your feature branch:
   ```bash
   git checkout -b feature/amazing-feature
   ```
3. Run tests and linting:
   ```bash
   cargo test
   cargo clippy
   ```
4. Commit your changes:
   ```bash
   git commit -m 'Add some amazing feature'
   ```
5. Push to the branch:
   ```bash
   git push origin feature/amazing-feature
   ```
6. Open a Pull Request 