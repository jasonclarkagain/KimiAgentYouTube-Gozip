//! Integration tests for ContentGuard AI

use std::time::Duration;

use contentguard::types::*;
use tokio::time::timeout;

/// Test configuration loading
#[tokio::test]
async fn test_config_loading() {
    let config = contentguard::config::AppConfig::load();
    assert!(config.is_ok());
}

/// Test timestamp calculations
#[test]
fn test_timestamp_conversions() {
    let ts = Timestamp::new(0, 1, 30, 15, 30);
    assert_eq!(ts.to_seconds(), 90.5);
    
    let ts2 = Timestamp::from_seconds(90.5, 30);
    assert_eq!(ts2.hours, 0);
    assert_eq!(ts2.minutes, 1);
    assert_eq!(ts2.seconds, 30);
}

/// Test time range operations
#[test]
fn test_time_range_operations() {
    let start = Timestamp::new(0, 0, 30, 0, 30);
    let end = Timestamp::new(0, 1, 0, 0, 30);
    let range = TimeRange::new(start, end);
    
    assert_eq!(range.duration_seconds(), 30.0);
    
    let inside = Timestamp::new(0, 0, 45, 0, 30);
    assert!(range.contains(&inside));
    
    let outside = Timestamp::new(0, 2, 0, 0, 30);
    assert!(!range.contains(&outside));
}

/// Test analysis report calculations
#[test]
fn test_strike_probability_calculation() {
    let mut report = AnalysisReport::new(
        "test.mp4".to_string(),
        AnalysisDepth::Standard,
    );
    
    // Add some test issues
    report.issues.push(create_test_issue(
        PolicyCategory::Copyright,
        Severity::High,
        0.85,
    ));
    
    report.issues.push(create_test_issue(
        PolicyCategory::Violence,
        Severity::Medium,
        0.70,
    ));
    
    report.calculate_strike_probability();
    
    // Strike probability should be between 0 and 100
    assert!(report.strike_probability >= 0.0);
    assert!(report.strike_probability <= 100.0);
    assert!(report.strike_probability > 0.0); // Should have some probability with issues
}

/// Test policy category display
#[test]
fn test_policy_category_display() {
    assert_eq!(PolicyCategory::Copyright.display_name(), "Copyright");
    assert_eq!(PolicyCategory::HateSpeech.display_name(), "Hate Speech");
    assert_eq!(PolicyCategory::Violence.display_name(), "Violence");
}

/// Test severity ordering
#[test]
fn test_severity_ordering() {
    assert!(Severity::Low < Severity::Medium);
    assert!(Severity::Medium < Severity::High);
    assert!(Severity::High < Severity::Critical);
}

/// Test utility functions
#[test]
fn test_format_bytes() {
    use contentguard::utils::format_bytes;
    
    assert_eq!(format_bytes(0), "0 B");
    assert_eq!(format_bytes(1024), "1.00 KB");
    assert_eq!(format_bytes(1024 * 1024), "1.00 MB");
    assert_eq!(format_bytes(1024 * 1024 * 1024), "1.00 GB");
}

#[test]
fn test_format_duration() {
    use contentguard::utils::format_duration;
    
    assert_eq!(format_duration(30.0), "30.0s");
    assert_eq!(format_duration(120.0), "2.0m");
    assert_eq!(format_duration(3660.0), "1.0h");
}

/// Test video path validation
#[test]
fn test_video_path_validation() {
    use contentguard::utils::validate_video_path;
    
    // These should fail (files don't exist)
    assert!(validate_video_path("/nonexistent/file.mp4").is_err());
    assert!(validate_video_path("test.txt").is_err());
}

/// Test hash utilities
#[test]
fn test_sha256_hash() {
    use contentguard::utils::hash::sha256;
    
    let hash = sha256(b"test");
    assert_eq!(hash.len(), 64); // SHA256 produces 32 bytes = 64 hex chars
    
    // Same input should produce same hash
    let hash2 = sha256(b"test");
    assert_eq!(hash, hash2);
    
    // Different input should produce different hash
    let hash3 = sha256(b"different");
    assert_ne!(hash, hash3);
}

/// Test compression utilities
#[test]
fn test_compression_roundtrip() {
    use contentguard::utils::compression;
    
    let original = b"This is test data for compression. ".repeat(100);
    
    let compressed = compression::compress_zstd(&original).unwrap();
    assert!(compressed.len() < original.len());
    
    let decompressed = compression::decompress_zstd(&compressed).unwrap();
    assert_eq!(decompressed, original);
}

/// Test validation utilities
#[test]
fn test_validation_functions() {
    use contentguard::utils::validation;
    
    // Valid UUID
    assert!(validation::validate_uuid("550e8400-e29b-41d4-a716-446655440000").is_ok());
    
    // Invalid UUID
    assert!(validation::validate_uuid("not-a-uuid").is_err());
    
    // Valid confidence
    assert!(validation::validate_confidence(0.5).is_ok());
    assert!(validation::validate_confidence(0.0).is_ok());
    assert!(validation::validate_confidence(1.0).is_ok());
    
    // Invalid confidence
    assert!(validation::validate_confidence(1.5).is_err());
    assert!(validation::validate_confidence(-0.1).is_err());
    
    // Valid intensity
    assert!(validation::validate_intensity(50).is_ok());
    assert!(validation::validate_intensity(0).is_ok());
    assert!(validation::validate_intensity(100).is_ok());
}

/// Helper function to create test issues
fn create_test_issue(
    category: PolicyCategory,
    severity: Severity,
    confidence: f32,
) -> Issue {
    use std::collections::HashMap;
    
    Issue {
        id: uuid::Uuid::new_v4(),
        category,
        severity,
        confidence,
        time_range: TimeRange::new(
            Timestamp::new(0, 0, 0, 0, 30),
            Timestamp::new(0, 0, 10, 0, 30),
        ),
        description: "Test issue".to_string(),
        evidence: vec![],
        suggested_fixes: vec![],
        policy_reference: None,
    }
}

/// Test fix type display
#[test]
fn test_fix_type_variants() {
    let fix_types = vec![
        FixType::Remove,
        FixType::Blur,
        FixType::ReplaceAudio,
        FixType::Mute,
        FixType::AddContext,
        FixType::SpeedUp,
        FixType::ColorGrade,
        FixType::Crop,
    ];
    
    // All fix types should be serializable
    for fix_type in fix_types {
        let json = serde_json::to_string(&fix_type);
        assert!(json.is_ok());
    }
}

/// Test impact level ordering
#[test]
fn test_impact_level_ordering() {
    assert!(ImpactLevel::Low < ImpactLevel::Medium);
    assert!(ImpactLevel::Medium < ImpactLevel::High);
}

/// Test analysis depth variants
#[test]
fn test_analysis_depth_variants() {
    let depths = vec![
        AnalysisDepth::Fast,
        AnalysisDepth::Standard,
        AnalysisDepth::Deep,
    ];
    
    for depth in depths {
        let json = serde_json::to_string(&depth);
        assert!(json.is_ok());
    }
}

/// Test analysis stage display
#[test]
fn test_analysis_stage_display() {
    assert_eq!(AnalysisStage::Initializing.display_name(), "Initializing");
    assert_eq!(AnalysisStage::AnalyzingVisuals.display_name(), "Analyzing Visuals");
    assert_eq!(AnalysisStage::Complete.display_name(), "Complete");
}

/// Test API request/response serialization
#[test]
fn test_api_types_serialization() {
    use contentguard::types::api::*;
    
    let request = AnalyzeRequest {
        video_path: "/path/to/video.mp4".to_string(),
        depth: Some(AnalysisDepth::Standard),
        callback_url: None,
    };
    
    let json = serde_json::to_string(&request);
    assert!(json.is_ok());
    
    let response = AnalyzeResponse {
        job_id: uuid::Uuid::new_v4(),
        status: "accepted".to_string(),
        estimated_seconds: 120,
    };
    
    let json = serde_json::to_string(&response);
    assert!(json.is_ok());
}

/// Test quality metrics
#[test]
fn test_quality_metrics() {
    let metrics = QualityMetrics {
        psnr: 35.0,
        ssim: 0.95,
        vmaf: 85.0,
        bitrate_kbps: 5000,
    };
    
    assert!(metrics.psnr > 0.0);
    assert!(metrics.ssim >= 0.0 && metrics.ssim <= 1.0);
    assert!(metrics.vmaf >= 0.0 && metrics.vmaf <= 100.0);
}

/// Test memory estimation
#[test]
fn test_memory_estimation() {
    use contentguard::utils::estimate_memory_usage;
    
    let memory = estimate_memory_usage(
        600.0,           // 10 minute video
        (1920, 1080),    // 1080p
        30.0,            // 30 fps
        0.5,             // 2 frames per second
    );
    
    // Should be a reasonable amount of memory
    assert!(memory > 0);
    assert!(memory < 10_000_000_000); // Less than 10GB
}

/// Test sanitize filename
#[test]
fn test_sanitize_filename() {
    use contentguard::utils::sanitize_filename;
    
    assert_eq!(sanitize_filename("test.mp4"), "test.mp4");
    assert_eq!(sanitize_filename("test file.mp4"), "test_file.mp4");
    assert_eq!(sanitize_filename("test@#$%.mp4"), "test___.mp4");
    assert_eq!(sanitize_filename("__test.mp4"), "test.mp4");
}

/// Test get extension
#[test]
fn test_get_extension() {
    use contentguard::utils::get_extension;
    
    assert_eq!(get_extension("test.mp4"), "mp4");
    assert_eq!(get_extension("test.MP4"), "mp4");
    assert_eq!(get_extension("/path/to/test.mkv"), "mkv");
    assert_eq!(get_extension("test"), "");
}
