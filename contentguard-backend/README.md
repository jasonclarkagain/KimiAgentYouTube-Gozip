# ContentGuard AI

[![Version](https://img.shields.io/badge/version-1.0.0-blue.svg)](https://github.com/contentguard/contentguard)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

**AI-powered YouTube strike prevention system** — Detect policy violations before you publish. Frame-accurate. Private. Built for creators.

## Features

- **Strike Probability Score (0-100%)** with category breakdown
- **Frame-accurate timestamp identification** for problematic segments
- **Policy violation mapping** to YouTube Community Guidelines
- **Auto-fix workflow** with three-panel comparison
- **GPU acceleration** via Vulkan (cross-platform, not vendor-locked)
- **Zero trust architecture** — all processing on-device
- **Real-time policy updates** when YouTube changes guidelines
- **Single binary deployment** with no external dependencies

## Quick Start

### Installation

```bash
# Download latest release
curl -L https://github.com/contentguard/contentguard/releases/latest/download/contentguard-x86_64-linux.gz | gunzip > contentguard
chmod +x contentguard
sudo mv contentguard /usr/local/bin/

# Verify installation
contentguard --version
```

### Analyze a Video

```bash
# Quick analysis
contentguard analyze --input video.mp4 --output report.json

# Deep analysis with GPU acceleration
contentguard analyze --input video.mp4 --output report.json --depth deep --gpu
```

### Start API Server

```bash
# Start server on default port
contentguard server

# Start with GPU acceleration
contentguard server --gpu --workers 8
```

### Apply Fixes

```bash
# Apply recommended fixes
contentguard fix --input video.mp4 --report report.json --output video_fixed.mp4

# Adjust fix intensity (0-100)
contentguard fix --input video.mp4 --report report.json --output video_fixed.mp4 --intensity 75
```

## API Documentation

### Health Check

```bash
GET /health
```

### Start Analysis

```bash
POST /api/v1/analyze
Content-Type: application/json

{
  "video_path": "/path/to/video.mp4",
  "depth": "standard"
}
```

### Get Analysis Status

```bash
GET /api/v1/analyze/{job_id}
```

### Get Analysis Report

```bash
GET /api/v1/analyze/{job_id}/report
```

### Apply Fixes

```bash
POST /api/v1/fix
Content-Type: application/json

{
  "video_path": "/path/to/video.mp4",
  "report_id": "uuid",
  "output_path": "/path/to/output.mp4",
  "intensity": 70
}
```

## Building from Source

### Prerequisites

- Rust 1.75+
- FFmpeg 4.0+
- Vulkan SDK (optional, for GPU acceleration)

### Build

```bash
# Clone repository
git clone https://github.com/contentguard/contentguard.git
cd contentguard

# Build native binary
./scripts/build.sh

# Build with GPU support
./scripts/build.sh --gpu

# Cross-compile for multiple architectures
./scripts/cross-compile.sh
```

### Docker

```bash
# Build Docker image
docker build -t contentguard:latest .

# Run container
docker run -p 8080:8080 -v $(pwd)/data:/app/data contentguard:latest
```

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    ContentGuard AI                          │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │   Analysis   │  │    Policy    │  │     Fix      │      │
│  │   Engine     │  │   Manager    │  │   Engine     │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │    Video     │  │    Audio     │  │     Text     │      │
│  │  Processor   │  │  Processor   │  │  Processor   │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐                         │
│  │    Vulkan    │  │     ML       │                         │
│  │    GPU       │  │  Inference   │                         │
│  └──────────────┘  └──────────────┘                         │
└─────────────────────────────────────────────────────────────┘
```

## Configuration

Create `contentguard.toml` in your config directory:

```toml
# ContentGuard Configuration

[data_dir]
path = "/home/user/.local/share/contentguard"

[gpu]
enabled = true
backend = "vulkan"
max_vram_mb = 2048

[analysis]
frame_interval_standard = 0.5
min_confidence_threshold = 0.6
parallel_threads = 4

[server]
bind_address = "127.0.0.1:8080"
workers = 4
enable_cors = true

[policy_updates]
auto_update = true
check_interval_hours = 24
```

## Policy Categories

| Category | Description | Detection Method |
|----------|-------------|------------------|
| Copyright | Rights-managed content | Audio fingerprint, visual similarity |
| Violence | Graphic violence/gore | Object detection, scene classification |
| Hate Speech | Promotes hatred | Text classification, sentiment analysis |
| Misinformation | False/misleading info | Text classification |
| Harassment | Targeted harassment | Text classification |
| Privacy | Privacy violations | Object detection |

## Performance Benchmarks

| Hardware | Video Length | Analysis Time | Memory Usage |
|----------|--------------|---------------|--------------|
| CPU (8 cores) | 10 min 1080p | ~2 min | ~2 GB |
| GPU (Vulkan) | 10 min 1080p | ~45 sec | ~1.5 GB |
| CPU (ARM64) | 10 min 1080p | ~3 min | ~1.8 GB |

## Testing

```bash
# Run unit tests
cargo test

# Run integration tests
cargo test --test integration_tests

# Run benchmarks
cargo bench
```

## Security

- **Zero trust**: All processing happens on-device
- **No data exfiltration**: Content never leaves your machine
- **Encrypted storage**: Policy database uses encryption at rest
- **Secure updates**: Policy updates are cryptographically signed

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [ONNX Runtime](https://onnxruntime.ai/) for ML inference
- [FFmpeg](https://ffmpeg.org/) for video processing
- [Vulkan](https://www.vulkan.org/) for GPU acceleration
- [Actix Web](https://actix.rs/) for the HTTP server

## Support

- Documentation: https://docs.contentguard.ai
- Issues: https://github.com/contentguard/contentguard/issues
- Discussions: https://github.com/contentguard/contentguard/discussions

---

**Made with ❤️ for content creators worldwide**
