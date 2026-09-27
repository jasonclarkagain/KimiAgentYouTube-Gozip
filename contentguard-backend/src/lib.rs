//! ContentGuard AI - Library exports
//!
//! This library provides the core functionality for content analysis
//! and policy violation detection.

pub mod analysis;
pub mod config;
pub mod engine;
pub mod fix;
pub mod policy;
pub mod server;
pub mod types;
pub mod utils;

// Re-export commonly used types
pub use types::*;
pub use config::AppConfig;
pub use analysis::ContentAnalyzer;
pub use engine::AnalysisEngine;
pub use fix::FixEngine;
pub use policy::PolicyManager;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Library name
pub const NAME: &str = env!("CARGO_PKG_NAME");
