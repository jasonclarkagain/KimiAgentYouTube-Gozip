//! Configuration management for ContentGuard AI

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Application name
    pub app_name: String,
    
    /// Application version
    pub version: String,
    
    /// Data directory path
    pub data_dir: PathBuf,
    
    /// Models directory path
    pub models_dir: PathBuf,
    
    /// Policies directory path
    pub policies_dir: PathBuf,
    
    /// Temporary directory for processing
    pub temp_dir: PathBuf,
    
    /// Maximum memory usage in MB (default: 4096)
    pub max_memory_mb: usize,
    
    /// GPU configuration
    pub gpu: GpuConfig,
    
    /// Analysis configuration
    pub analysis: AnalysisConfig,
    
    /// API server configuration
    pub server: ServerConfig,
    
    /// Policy update configuration
    pub policy_updates: PolicyUpdateConfig,
    
    /// Logging configuration
    pub logging: LoggingConfig,
}

impl AppConfig {
    /// Load configuration from file or create default
    pub fn load() -> Result<Self> {
        // Try to load from config file
        let config_paths = [
            PathBuf::from("contentguard.toml"),
            dirs::config_dir()
                .map(|d| d.join("contentguard").join("config.toml"))
                .unwrap_or_default(),
            PathBuf::from("/etc/contentguard/config.toml"),
        ];
        
        for path in &config_paths {
            if path.exists() {
                let content = std::fs::read_to_string(path)
                    .with_context(|| format!("Failed to read config from {:?}", path))?;
                let config: AppConfig = toml::from_str(&content)
                    .with_context(|| format!("Failed to parse config from {:?}", path))?;
                return Ok(config);
            }
        }
        
        // Return default configuration
        Ok(Self::default())
    }
    
    /// Save configuration to file
    pub fn save(&self, path: &Path) -> Result<()> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
    
    /// Get default data directory
    fn default_data_dir() -> PathBuf {
        dirs::data_dir()
            .map(|d| d.join("contentguard"))
            .unwrap_or_else(|| PathBuf::from("./data"))
    }
    
    /// Ensure all directories exist
    pub fn ensure_directories(&self) -> Result<()> {
        std::fs::create_dir_all(&self.data_dir)?;
        std::fs::create_dir_all(&self.models_dir)?;
        std::fs::create_dir_all(&self.policies_dir)?;
        std::fs::create_dir_all(&self.temp_dir)?;
        Ok(())
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        let data_dir = Self::default_data_dir();
        
        Self {
            app_name: "ContentGuard AI".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            models_dir: data_dir.join("models"),
            policies_dir: data_dir.join("policies"),
            temp_dir: std::env::temp_dir().join("contentguard"),
            data_dir,
            max_memory_mb: 4096,
            gpu: GpuConfig::default(),
            analysis: AnalysisConfig::default(),
            server: ServerConfig::default(),
            policy_updates: PolicyUpdateConfig::default(),
            logging: LoggingConfig::default(),
        }
    }
}

/// GPU configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuConfig {
    /// Enable GPU acceleration
    pub enabled: bool,
    
    /// Preferred GPU backend (vulkan, metal, dx12)
    pub backend: String,
    
    /// Device index (0 for primary)
    pub device_index: usize,
    
    /// Maximum VRAM usage in MB
    pub max_vram_mb: usize,
    
    /// Compute shader workgroup size
    pub workgroup_size: (u32, u32, u32),
    
    /// Enable shader caching
    pub shader_cache: bool,
    
    /// Shader cache directory
    pub shader_cache_dir: Option<PathBuf>,
}

impl Default for GpuConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            backend: "vulkan".to_string(),
            device_index: 0,
            max_vram_mb: 2048,
            workgroup_size: (256, 1, 1),
            shader_cache: true,
            shader_cache_dir: None,
        }
    }
}

/// Analysis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisConfig {
    /// Frame extraction interval in seconds (for fast mode)
    pub frame_interval_fast: f64,
    
    /// Frame extraction interval in seconds (for standard mode)
    pub frame_interval_standard: f64,
    
    /// Frame extraction interval in seconds (for deep mode)
    pub frame_interval_deep: f64,
    
    /// Audio sample rate for analysis
    pub audio_sample_rate: u32,
    
    /// Enable audio fingerprinting
    pub enable_audio_fingerprint: bool,
    
    /// Enable visual object detection
    pub enable_visual_detection: bool,
    
    /// Enable text/transcript analysis
    pub enable_text_analysis: bool,
    
    /// Enable scene classification
    pub enable_scene_classification: bool,
    
    /// Minimum confidence threshold for detections
    pub min_confidence_threshold: f32,
    
    /// Number of parallel analysis threads
    pub parallel_threads: usize,
    
    /// Batch size for GPU inference
    pub gpu_batch_size: usize,
    
    /// Maximum segment duration for analysis
    pub max_segment_duration_seconds: f64,
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            frame_interval_fast: 2.0,
            frame_interval_standard: 0.5,
            frame_interval_deep: 0.1,
            audio_sample_rate: 16000,
            enable_audio_fingerprint: true,
            enable_visual_detection: true,
            enable_text_analysis: true,
            enable_scene_classification: true,
            min_confidence_threshold: 0.6,
            parallel_threads: num_cpus::get(),
            gpu_batch_size: 32,
            max_segment_duration_seconds: 300.0,
        }
    }
}

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Bind address
    pub bind_address: String,
    
    /// Number of worker threads
    pub workers: usize,
    
    /// Request timeout in seconds
    pub request_timeout_seconds: u64,
    
    /// Maximum request size in MB
    pub max_request_size_mb: usize,
    
    /// Enable CORS
    pub enable_cors: bool,
    
    /// Allowed CORS origins
    pub cors_origins: Vec<String>,
    
    /// Enable WebSocket support
    pub enable_websocket: bool,
    
    /// JWT secret for authentication
    pub jwt_secret: Option<String>,
    
    /// Enable metrics endpoint
    pub enable_metrics: bool,
    
    /// Metrics endpoint path
    pub metrics_path: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind_address: "127.0.0.1:8080".to_string(),
            workers: num_cpus::get(),
            request_timeout_seconds: 300,
            max_request_size_mb: 1024,
            enable_cors: true,
            cors_origins: vec!["http://localhost:3000".to_string()],
            enable_websocket: true,
            jwt_secret: None,
            enable_metrics: true,
            metrics_path: "/metrics".to_string(),
        }
    }
}

/// Policy update configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyUpdateConfig {
    /// Enable automatic policy updates
    pub auto_update: bool,
    
    /// Update check interval in hours
    pub check_interval_hours: u64,
    
    /// Policy update endpoint URL
    pub update_url: String,
    
    /// Last update timestamp
    pub last_update: Option<chrono::DateTime<chrono::Utc>>,
    
    /// Verify update signatures
    pub verify_signatures: bool,
    
    /// Public key for signature verification
    pub public_key_path: Option<PathBuf>,
}

impl Default for PolicyUpdateConfig {
    fn default() -> Self {
        Self {
            auto_update: true,
            check_interval_hours: 24,
            update_url: "https://api.contentguard.ai/v1/policies".to_string(),
            last_update: None,
            verify_signatures: true,
            public_key_path: None,
        }
    }
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level (trace, debug, info, warn, error)
    pub level: String,
    
    /// Log format (json, pretty, compact)
    pub format: String,
    
    /// Log to file
    pub log_to_file: bool,
    
    /// Log file path
    pub log_file_path: Option<PathBuf>,
    
    /// Log rotation (daily, hourly, never)
    pub rotation: String,
    
    /// Enable console output
    pub console_output: bool,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            format: "pretty".to_string(),
            log_to_file: false,
            log_file_path: None,
            rotation: "daily".to_string(),
            console_output: true,
        }
    }
}

/// Model configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    /// Model name
    pub name: String,
    
    /// Model version
    pub version: String,
    
    /// Model file path
    pub path: PathBuf,
    
    /// Input shape
    pub input_shape: Vec<usize>,
    
    /// Output classes
    pub output_classes: Vec<String>,
    
    /// Confidence threshold
    pub confidence_threshold: f32,
    
    /// Preprocessing parameters
    pub preprocessing: HashMap<String, serde_json::Value>,
    
    /// Postprocessing parameters
    pub postprocessing: HashMap<String, serde_json::Value>,
}

/// Feature flags for enabling/disabling features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlags {
    pub enable_gpu: bool,
    pub enable_distributed: bool,
    pub enable_cloud_sync: bool,
    pub enable_telemetry: bool,
    pub enable_experimental_models: bool,
}

impl Default for FeatureFlags {
    fn default() -> Self {
        Self {
            enable_gpu: true,
            enable_distributed: false,
            enable_cloud_sync: false,
            enable_telemetry: false,
            enable_experimental_models: false,
        }
    }
}
