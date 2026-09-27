//! Utility functions for ContentGuard AI

use std::path::Path;

use anyhow::{Context, Result};

/// Format bytes to human-readable string
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    
    if bytes == 0 {
        return "0 B".to_string();
    }
    
    let exp = (bytes as f64).log(1024.0).min(UNITS.len() as f64 - 1.0) as usize;
    let value = bytes as f64 / 1024f64.powi(exp as i32);
    
    format!("{:.2} {}", value, UNITS[exp])
}

/// Format duration to human-readable string
pub fn format_duration(seconds: f64) -> String {
    if seconds < 60.0 {
        format!("{:.1}s", seconds)
    } else if seconds < 3600.0 {
        format!("{:.1}m", seconds / 60.0)
    } else {
        format!("{:.1}h", seconds / 3600.0)
    }
}

/// Validate video file path
pub fn validate_video_path(path: &str) -> Result<()> {
    let path = Path::new(path);
    
    if !path.exists() {
        anyhow::bail!("Video file does not exist: {}", path.display());
    }
    
    if !path.is_file() {
        anyhow::bail!("Path is not a file: {}", path.display());
    }
    
    // Check file extension
    let ext = path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    
    let valid_extensions = vec![
        "mp4", "avi", "mkv", "mov", "wmv", "flv", "webm", "m4v", "3gp"
    ];
    
    if !valid_extensions.contains(&ext.as_str()) {
        anyhow::bail!("Unsupported video format: {}", ext);
    }
    
    Ok(())
}

/// Get file extension
pub fn get_extension(path: &str) -> String {
    Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase()
}

/// Sanitize filename
pub fn sanitize_filename(filename: &str) -> String {
    filename
        .replace(|c: char| !c.is_alphanumeric() && c != '.' && c != '-' && c != '_', "_")
        .trim_start_matches('_')
        .to_lowercase()
}

/// Calculate memory usage estimate for analysis
pub fn estimate_memory_usage(
    video_duration_seconds: f64,
    resolution: (u32, u32),
    fps: f64,
    frame_interval: f64,
) -> usize {
    let num_frames = (video_duration_seconds / frame_interval) as usize;
    let bytes_per_frame = (resolution.0 * resolution.1 * 3) as usize; // RGB
    let frame_memory = num_frames * bytes_per_frame;
    
    // Add overhead for processing
    let overhead = frame_memory / 4;
    
    frame_memory + overhead
}

/// Check if system has sufficient memory
pub fn check_memory_available(required_mb: usize) -> Result<()> {
    use sysinfo::{System, SystemExt};
    
    let mut sys = System::new_all();
    sys.refresh_all();
    
    let available_mb = sys.available_memory() / 1024;
    
    if available_mb < required_mb as u64 {
        anyhow::bail!(
            "Insufficient memory: {} MB required, {} MB available",
            required_mb,
            available_mb
        );
    }
    
    Ok(())
}

/// Rate limiter for API requests
pub struct RateLimiter {
    max_requests: u32,
    window_seconds: u64,
}

impl RateLimiter {
    pub fn new(max_requests: u32, window_seconds: u64) -> Self {
        Self {
            max_requests,
            window_seconds,
        }
    }
    
    pub fn check_rate_limit(&self, _client_id: &str) -> Result<()> {
        // Simplified implementation - would use Redis or in-memory store
        Ok(())
    }
}

/// Secure temporary file handling
pub struct TempFile {
    path: std::path::PathBuf,
}

impl TempFile {
    pub fn new() -> Result<Self> {
        let path = tempfile::NamedTempFile::new()?.into_temp_path().to_path_buf();
        Ok(Self { path })
    }
    
    pub fn path(&self) -> &std::path::Path {
        &self.path
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

/// Async file operations
pub mod async_fs {
    use super::*;
    
    /// Copy file asynchronously
    pub async fn copy_file(src: &str, dst: &str) -> Result<u64> {
        tokio::fs::copy(src, dst).await
            .with_context(|| format!("Failed to copy {} to {}", src, dst))
    }
    
    /// Move file asynchronously
    pub async fn move_file(src: &str, dst: &str) -> Result<()> {
        tokio::fs::rename(src, dst).await
            .with_context(|| format!("Failed to move {} to {}", src, dst))
    }
    
    /// Remove file if exists
    pub async fn remove_file_if_exists(path: &str) -> Result<()> {
        if Path::new(path).exists() {
            tokio::fs::remove_file(path).await
                .with_context(|| format!("Failed to remove {}", path))?;
        }
        Ok(())
    }
}

/// Hash utilities
pub mod hash {
    use ring::digest::{Context, SHA256};
    
    /// Calculate SHA256 hash of data
    pub fn sha256(data: &[u8]) -> String {
        let mut context = Context::new(&SHA256);
        context.update(data);
        let digest = context.finish();
        hex::encode(digest.as_ref())
    }
    
    /// Calculate file hash
    pub fn file_hash(path: &str) -> Result<String> {
        let data = std::fs::read(path)?;
        Ok(sha256(&data))
    }
}

/// Compression utilities
pub mod compression {
    use super::*;
    
    /// Compress data using zstd
    pub fn compress_zstd(data: &[u8]) -> Result<Vec<u8>> {
        zstd::encode_all(data, 3)
            .context("Failed to compress data with zstd")
    }
    
    /// Decompress zstd data
    pub fn decompress_zstd(data: &[u8]) -> Result<Vec<u8>> {
        zstd::decode_all(data)
            .context("Failed to decompress zstd data")
    }
}

/// Logging utilities
pub mod logging {
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;
    
    /// Initialize structured logging
    pub fn init_logging(level: &str) -> Result<()> {
        let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(level));
        
        tracing_subscriber::registry()
            .with(env_filter)
            .with(tracing_subscriber::fmt::layer().json())
            .init();
        
        Ok(())
    }
}

/// Time utilities
pub mod time {
    use chrono::{DateTime, Utc};
    
    /// Format timestamp to ISO 8601
    pub fn format_iso(timestamp: DateTime<Utc>) -> String {
        timestamp.to_rfc3339()
    }
    
    /// Parse ISO 8601 timestamp
    pub fn parse_iso(s: &str) -> Result<DateTime<Utc>> {
        DateTime::parse_from_rfc3339(s)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(|e| anyhow::anyhow!("Failed to parse timestamp: {}", e))
    }
}

/// Validation utilities
pub mod validation {
    use super::*;
    
    /// Validate UUID string
    pub fn validate_uuid(s: &str) -> Result<()> {
        uuid::Uuid::parse_str(s)
            .map_err(|e| anyhow::anyhow!("Invalid UUID: {}", e))?;
        Ok(())
    }
    
    /// Validate confidence value (0.0 to 1.0)
    pub fn validate_confidence(value: f32) -> Result<()> {
        if value < 0.0 || value > 1.0 {
            anyhow::bail!("Confidence must be between 0.0 and 1.0");
        }
        Ok(())
    }
    
    /// Validate intensity value (0 to 100)
    pub fn validate_intensity(value: u8) -> Result<()> {
        if value > 100 {
            anyhow::bail!("Intensity must be between 0 and 100");
        }
        Ok(())
    }
}
