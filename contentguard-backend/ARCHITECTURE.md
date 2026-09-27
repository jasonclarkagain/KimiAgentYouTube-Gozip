# ContentGuard AI - Architecture Documentation

## Overview

ContentGuard AI is a high-performance, privacy-first content analysis system built in Rust. It provides frame-accurate detection of YouTube policy violations with GPU acceleration support.

## System Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              ContentGuard AI                                  │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                         API Layer (Actix-Web)                        │   │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌────────────┐ │   │
│  │  │   /health   │  │  /analyze   │  │    /fix     │  │ /policies  │ │   │
│  │  └─────────────┘  └─────────────┘  └─────────────┘  └────────────┘ │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                    │                                        │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                      Core Analysis Engine                            │   │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐              │   │
│  │  │   Video      │  │    Audio     │  │     Text     │              │   │
│  │  │  Processor   │  │  Processor   │  │  Processor   │              │   │
│  │  └──────────────┘  └──────────────┘  └──────────────┘              │   │
│  │         │                 │                 │                       │   │
│  │         └─────────────────┼─────────────────┘                       │   │
│  │                           ▼                                         │   │
│  │  ┌─────────────────────────────────────────────────────────────┐   │   │
│  │  │              ML Inference Engine (ONNX/Tract)                │   │   │
│  │  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │   │   │
│  │  │  │   Object    │  │    Scene    │  │   Text Classifier   │  │   │   │
│  │  │  │  Detection  │  │ Classification│  │                     │  │   │   │
│  │  │  └─────────────┘  └─────────────┘  └─────────────────────┘  │   │   │
│  │  └─────────────────────────────────────────────────────────────┘   │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                    │                                        │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                      GPU Acceleration (Vulkan)                       │   │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐              │   │
│  │  │   Compute    │  │   Memory     │  │   Shader     │              │   │
│  │  │   Shaders    │  │   Manager    │  │   Cache      │              │   │
│  │  └──────────────┘  └──────────────┘  └──────────────┘              │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                    │                                        │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                      Policy Engine                                   │   │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐              │   │
│  │  │   Policy     │  │   Detection  │  │    Update    │              │   │
│  │  │   Database   │  │    Rules     │  │   Service    │              │   │
│  │  └──────────────┘  └──────────────┘  └──────────────┘              │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                    │                                        │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                      Fix Engine                                      │   │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐              │   │
│  │  │   FFmpeg     │  │   Filter     │  │   Quality    │              │   │
│  │  │   Wrapper    │  │   Builder    │  │   Metrics    │              │   │
│  │  └──────────────┘  └──────────────┘  └──────────────┘              │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Module Descriptions

### 1. API Layer (`server.rs`)

The REST API layer built with Actix-Web provides HTTP endpoints for:
- Starting analysis jobs
- Querying job status
- Retrieving reports
- Applying fixes
- Managing policies

**Key Components:**
- `ApiServer`: Main server struct
- `ApiServerState`: Shared state across handlers
- Route handlers for each endpoint

### 2. Core Analysis Engine (`analysis.rs`, `engine.rs`)

The heart of the system, responsible for:
- Frame extraction from videos
- Visual content analysis
- Audio fingerprinting
- Text/transcript analysis
- Issue detection and classification

**Key Components:**
- `ContentAnalyzer`: Main analysis coordinator
- `AnalysisEngine`: Low-level processing engine
- `FrameExtractor`: FFmpeg-based frame extraction
- `ObjectDetector`: ML-based object detection
- `SceneClassifier`: Scene classification
- `AudioProcessor`: Audio fingerprinting
- `TextProcessor`: Speech-to-text and analysis

### 3. ML Inference Engine

Uses ONNX Runtime and Tract for running ML models:
- Object detection (YOLO/Faster R-CNN)
- Scene classification (ResNet/EfficientNet)
- Text classification (BERT/RoBERTa)

**Key Features:**
- Model quantization for faster inference
- Batch processing for efficiency
- GPU acceleration support

### 4. GPU Acceleration (`vulkan` module)

Vulkan-based GPU acceleration for:
- Frame preprocessing
- ML inference
- Filter application

**Key Components:**
- `GpuContext`: Vulkan context management
- Compute shaders for parallel processing
- Memory management for GPU buffers

### 5. Policy Engine (`policy.rs`)

Manages YouTube Community Guidelines:
- Policy definitions
- Detection rules
- Update mechanism

**Key Components:**
- `PolicyManager`: Policy loading and management
- `Policy`: Individual policy definition
- `DetectionRule`: Rule for detecting violations
- `VisualPolicy`/`TextPolicy`: Policy wrappers

### 6. Fix Engine (`fix.rs`)

Applies automatic corrections:
- FFmpeg filter generation
- Fix application
- Quality metrics

**Key Components:**
- `FixEngine`: Main fix coordinator
- `FixConfiguration`: Fix parameters
- `FixPresets`: Predefined configurations

## Data Flow

### Analysis Flow

```
1. User submits video via API
   │
   ▼
2. API creates AnalysisJob
   │
   ▼
3. ContentAnalyzer processes video:
   a. Extract frames at intervals
   b. Run object detection on frames
   c. Extract and fingerprint audio
   d. Extract and analyze transcript
   │
   ▼
4. Issues are detected and classified
   │
   ▼
5. AnalysisReport is generated
   │
   ▼
6. User retrieves report via API
```

### Fix Flow

```
1. User submits fix request with report
   │
   ▼
2. FixEngine builds FFmpeg filter complex
   │
   ▼
3. FFmpeg applies fixes to video
   │
   ▼
4. Quality metrics are calculated
   │
   ▼
5. FixResult is returned
```

## Memory Management

### Memory Limits

- **Default**: 4GB maximum for 1080p analysis
- **GPU VRAM**: 2GB maximum
- **Configurable** via `max_memory_mb` setting

### Memory Pools

- Frame buffer pool for extracted frames
- GPU memory pool for inference
- Temporary file management

### Garbage Collection

- Automatic cleanup of temporary files
- Frame buffer recycling
- GPU memory deallocation

## Concurrency Model

### Async Runtime

- Tokio-based async runtime
- Multi-threaded executor
- Work-stealing scheduler

### Parallel Processing

- Frame extraction: Parallel decode
- Analysis: Batch processing with Rayon
- GPU: Async compute queues

### Synchronization

- `RwLock` for shared state
- `mpsc` channels for progress updates
- Atomic counters for statistics

## Security Architecture

### Zero Trust Principles

1. **No data exfiltration**: All processing on-device
2. **Encrypted storage**: Policy database encrypted at rest
3. **Secure updates**: Cryptographically signed policy updates
4. **No telemetry**: Optional and disabled by default

### Data Protection

- Temporary files cleaned up automatically
- Memory zeroing for sensitive data
- Sandboxed FFmpeg execution

## Performance Optimizations

### CPU Optimizations

- SIMD vectorization (AVX2/NEON)
- Multi-threading with Rayon
- Cache-friendly data structures

### GPU Optimizations

- Vulkan compute shaders
- Async memory transfers
- Pipeline parallelism

### I/O Optimizations

- Buffered file I/O
- Async I/O with Tokio
- Memory-mapped files for large videos

## Error Handling

### Error Types

- `AnalysisError`: Analysis failures
- `FixError`: Fix application failures
- `PolicyError`: Policy-related errors
- `GpuError`: GPU operation failures

### Recovery Strategies

- Retry with backoff for transient errors
- Graceful degradation (CPU fallback)
- Partial results on failure

## Testing Strategy

### Unit Tests

- Individual component testing
- Mock dependencies
- Property-based testing with Proptest

### Integration Tests

- End-to-end analysis flow
- API endpoint testing
- Cross-platform compatibility

### Benchmarks

- Criterion.rs for performance benchmarks
- Memory usage profiling
- GPU utilization metrics

## Deployment Options

### Standalone Binary

- Single static binary
- No runtime dependencies
- Cross-platform support

### Docker Container

- Multi-stage build
- Minimal runtime image
- Health checks included

### Systemd Service

- Service unit file
- Auto-restart on failure
- Log rotation

## Monitoring

### Metrics

- Prometheus-compatible metrics endpoint
- Custom metrics for analysis operations
- GPU utilization tracking

### Logging

- Structured logging with tracing
- Configurable log levels
- Rotation and compression

### Health Checks

- `/health` endpoint
- Component status reporting
- Dependency checks

## Future Enhancements

### Planned Features

1. **Distributed Processing**: Multi-node analysis
2. **Cloud Sync**: Optional policy sync
3. **Custom Models**: User-trained detectors
4. **Live Streaming**: Real-time analysis
5. **Mobile Support**: iOS/Android apps

### Research Areas

1. **Faster Inference**: TensorRT, Core ML
2. **Better Accuracy**: Transformer-based models
3. **Lower Latency**: Edge deployment
4. **More Policies**: Platform-specific rules
