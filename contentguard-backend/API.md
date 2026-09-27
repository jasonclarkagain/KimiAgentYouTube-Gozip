# ContentGuard AI - API Documentation

## Base URL

```
http://localhost:8080
```

## Authentication

Currently, the API does not require authentication for local use. For production deployments, JWT authentication can be enabled via configuration.

## Endpoints

### Health Check

Check if the server is running and get system status.

```http
GET /health
```

**Response:**

```json
{
  "status": "healthy",
  "version": "1.0.0",
  "gpu_available": true,
  "models_loaded": ["object_detection", "scene_classification"]
}
```

### Start Analysis

Submit a video for analysis.

```http
POST /api/v1/analyze
Content-Type: application/json
```

**Request Body:**

```json
{
  "video_path": "/path/to/video.mp4",
  "depth": "standard",
  "callback_url": "https://example.com/webhook"
}
```

**Parameters:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `video_path` | string | Yes | Absolute path to video file |
| `depth` | string | No | Analysis depth: `fast`, `standard`, `deep` (default: `standard`) |
| `callback_url` | string | No | URL to receive progress webhooks |

**Response:**

```json
{
  "job_id": "550e8400-e29b-41d4-a716-446655440000",
  "status": "accepted",
  "estimated_seconds": 120
}
```

### Get Analysis Status

Check the status of an analysis job.

```http
GET /api/v1/analyze/{job_id}
```

**Response (Processing):**

```json
{
  "job_id": "550e8400-e29b-41d4-a716-446655440000",
  "report": null,
  "status": "running",
  "progress": {
    "job_id": "550e8400-e29b-41d4-a716-446655440000",
    "stage": "AnalyzingVisuals",
    "progress_percent": 45.0,
    "current_segment": 10,
    "total_segments": 100,
    "message": "Processing frame 10 of 100",
    "estimated_seconds_remaining": 60
  }
}
```

**Response (Complete):**

```json
{
  "job_id": "550e8400-e29b-41d4-a716-446655440000",
  "report": null,
  "status": "completed",
  "progress": null
}
```

### Get Analysis Report

Retrieve the full analysis report for a completed job.

```http
GET /api/v1/analyze/{job_id}/report
```

**Response:**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "created_at": "2026-01-15T10:30:00Z",
  "video_path": "/path/to/video.mp4",
  "video_duration_seconds": 600.0,
  "video_resolution": [1920, 1080],
  "video_fps": 30.0,
  "strike_probability": 73.5,
  "overall_risk_score": 73.5,
  "issues": [
    {
      "id": "660e8400-e29b-41d4-a716-446655440001",
      "category": "Copyright",
      "severity": "High",
      "confidence": 0.88,
      "time_range": {
        "start": {
          "hours": 0,
          "minutes": 0,
          "seconds": 34,
          "frames": 12,
          "fps": 30
        },
        "end": {
          "hours": 0,
          "minutes": 0,
          "seconds": 48,
          "frames": 0,
          "fps": 30
        }
      },
      "description": "Audio matches copyrighted track: 'Test Song' (confidence: 88.0%)",
      "evidence": [
        {
          "AudioFingerprint": {
            "matched_track": "Test Song",
            "confidence": 0.88,
            "sample_path": "/tmp/sample.wav"
          }
        }
      ],
      "suggested_fixes": [
        {
          "id": "770e8400-e29b-41d4-a716-446655440002",
          "fix_type": "ReplaceAudio",
          "impact": "Low",
          "description": "Replace with royalty-free alternative",
          "estimated_quality_loss": 0.1,
          "parameters": {}
        }
      ],
      "policy_reference": "copyright"
    }
  ],
  "category_breakdown": {
    "Copyright": 45.0,
    "Violence": 25.0,
    "HateSpeech": 20.0,
    "Misinformation": 10.0
  },
  "processing_time_seconds": 125.5,
  "analysis_depth": "Standard",
  "metadata": {}
}
```

### Apply Fixes

Apply automatic fixes to a video based on analysis report.

```http
POST /api/v1/fix
Content-Type: application/json
```

**Request Body:**

```json
{
  "video_path": "/path/to/video.mp4",
  "report_id": "550e8400-e29b-41d4-a716-446655440000",
  "selected_fixes": ["770e8400-e29b-41d4-a716-446655440002"],
  "intensity": 70,
  "preserve_audio": true
}
```

**Parameters:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `video_path` | string | Yes | Path to original video |
| `report_id` | string | Yes | UUID of analysis report |
| `selected_fixes` | array | No | Specific fix IDs to apply (default: all) |
| `intensity` | number | No | Fix intensity 0-100 (default: 70) |
| `preserve_audio` | boolean | No | Prioritize audio preservation (default: true) |

**Response:**

```json
{
  "id": "880e8400-e29b-41d4-a716-446655440003",
  "original_video": "/path/to/video.mp4",
  "output_video": "/path/to/video_fixed.mp4",
  "applied_fixes": [
    {
      "issue_id": "660e8400-e29b-41d4-a716-446655440001",
      "fix_type": "ReplaceAudio",
      "time_range": {
        "start": { "hours": 0, "minutes": 0, "seconds": 34, "frames": 12, "fps": 30 },
        "end": { "hours": 0, "minutes": 0, "seconds": 48, "frames": 0, "fps": 30 }
      },
      "parameters": {}
    }
  ],
  "processing_time_seconds": 45.2,
  "output_file_size_bytes": 52428800,
  "quality_metrics": {
    "psnr": 35.0,
    "ssim": 0.95,
    "vmaf": 85.0,
    "bitrate_kbps": 5000
  }
}
```

### Get Policies

Retrieve all active policies.

```http
GET /api/v1/policies
```

**Response:**

```json
[
  {
    "id": "copyright",
    "category": "Copyright",
    "name": "Copyright Infringement",
    "description": "Detects use of copyrighted material without proper authorization",
    "guidelines_url": "https://www.youtube.com/howyoutubeworks/policies/copyright/",
    "detection_rules": [
      {
        "rule_type": "AudioFingerprint",
        "confidence_threshold": 0.7,
        "parameters": {}
      }
    ],
    "version": "1.0.0",
    "updated_at": "2026-01-01T00:00:00Z"
  }
]
```

### Update Policies

Manually trigger a policy update check.

```http
POST /api/v1/policies/update
```

**Response:**

```json
{
  "status": "updated",
  "message": "Policies updated successfully"
}
```

### Preview Fix

Generate a preview of a fix without applying it.

```http
POST /api/v1/preview
Content-Type: application/json
```

**Request Body:**

```json
{
  "video_path": "/path/to/video.mp4",
  "issue_id": "660e8400-e29b-41d4-a716-446655440001",
  "preview_path": "/tmp/preview.mp4"
}
```

**Response:**

```json
{
  "status": "success",
  "preview_path": "/tmp/preview.mp4"
}
```

### Metrics

Get Prometheus-compatible metrics.

```http
GET /metrics
```

**Response:**

```
# HELP contentguard_analysis_total Total number of analyses performed
# TYPE contentguard_analysis_total counter
contentguard_analysis_total 42

# HELP contentguard_fixes_total Total number of fixes applied
# TYPE contentguard_fixes_total counter
contentguard_fixes_total 15

# HELP contentguard_active_jobs Number of active analysis jobs
# TYPE contentguard_active_jobs gauge
contentguard_active_jobs 3
```

## WebSocket API

Connect to WebSocket for real-time progress updates:

```javascript
const ws = new WebSocket('ws://localhost:8080/ws');

ws.onmessage = (event) => {
  const progress = JSON.parse(event.data);
  console.log(`Progress: ${progress.progress_percent}%`);
};
```

## Error Responses

All errors follow this format:

```json
{
  "error": "Error message description"
}
```

**Common HTTP Status Codes:**

| Code | Description |
|------|-------------|
| 200 | Success |
| 202 | Accepted (async operation started) |
| 400 | Bad Request |
| 404 | Not Found |
| 500 | Internal Server Error |

## Rate Limiting

API requests are rate-limited to:
- 100 requests per minute for analysis endpoints
- 1000 requests per minute for status queries

## SDK Examples

### Python

```python
import requests

# Start analysis
response = requests.post('http://localhost:8080/api/v1/analyze', json={
    'video_path': '/path/to/video.mp4',
    'depth': 'standard'
})
job_id = response.json()['job_id']

# Poll for completion
import time
while True:
    status = requests.get(f'http://localhost:8080/api/v1/analyze/{job_id}').json()
    if status['status'] == 'completed':
        break
    time.sleep(1)

# Get report
report = requests.get(f'http://localhost:8080/api/v1/analyze/{job_id}/report').json()
print(f"Strike probability: {report['strike_probability']}%")
```

### JavaScript/Node.js

```javascript
const axios = require('axios');

async function analyzeVideo(videoPath) {
  // Start analysis
  const { data: { job_id } } = await axios.post(
    'http://localhost:8080/api/v1/analyze',
    { video_path: videoPath, depth: 'standard' }
  );
  
  // Poll for completion
  const poll = async () => {
    const { data } = await axios.get(`http://localhost:8080/api/v1/analyze/${job_id}`);
    if (data.status === 'completed') {
      return axios.get(`http://localhost:8080/api/v1/analyze/${job_id}/report`);
    }
    await new Promise(r => setTimeout(r, 1000));
    return poll();
  };
  
  const { data: report } = await poll();
  console.log(`Strike probability: ${report.strike_probability}%`);
}
```

### cURL

```bash
# Start analysis
curl -X POST http://localhost:8080/api/v1/analyze \
  -H "Content-Type: application/json" \
  -d '{"video_path": "/path/to/video.mp4", "depth": "standard"}'

# Check status
curl http://localhost:8080/api/v1/analyze/{job_id}

# Get report
curl http://localhost:8080/api/v1/analyze/{job_id}/report

# Apply fixes
curl -X POST http://localhost:8080/api/v1/fix \
  -H "Content-Type: application/json" \
  -d '{
    "video_path": "/path/to/video.mp4",
    "report_id": "{job_id}",
    "output_path": "/path/to/fixed.mp4",
    "intensity": 70
  }'
```
