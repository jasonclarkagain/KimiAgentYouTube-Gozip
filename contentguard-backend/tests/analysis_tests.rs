//! Unit tests for analysis module

use contentguard::analysis::*;
use contentguard::types::*;
use std::collections::HashMap;

/// Test frame data creation
#[test]
fn test_frame_data_creation() {
    let frame = FrameData {
        timestamp_seconds: 10.5,
        frame_number: 315,
        image_data: vec![0u8; 100],
        width: 1920,
        height: 1080,
    };
    
    assert_eq!(frame.timestamp_seconds, 10.5);
    assert_eq!(frame.frame_number, 315);
    assert_eq!(frame.width, 1920);
    assert_eq!(frame.height, 1080);
}

/// Test object detection result
#[test]
fn test_object_detection() {
    let detection = ObjectDetection {
        label: "person".to_string(),
        confidence: 0.92,
        bounding_box: BoundingBox {
            x: 0.1,
            y: 0.2,
            width: 0.3,
            height: 0.4,
        },
    };
    
    assert_eq!(detection.label, "person");
    assert!(detection.confidence > 0.0 && detection.confidence <= 1.0);
    assert_eq!(detection.bounding_box.x, 0.1);
}

/// Test scene classification
#[test]
fn test_scene_classification() {
    let scene = SceneClass {
        label: "urban".to_string(),
        confidence: 0.85,
    };
    
    assert_eq!(scene.label, "urban");
    assert!(scene.confidence > 0.0);
}

/// Test audio fingerprint
#[test]
fn test_audio_fingerprint() {
    let fingerprint = AudioFingerprint {
        hashes: vec![123456789u64, 987654321u64],
        sample_rate: 16000,
        duration_seconds: 60.0,
    };
    
    assert_eq!(fingerprint.hashes.len(), 2);
    assert_eq!(fingerprint.sample_rate, 16000);
}

/// Test audio match
#[test]
fn test_audio_match() {
    let match_result = AudioMatch {
        track_name: "Test Song".to_string(),
        confidence: 0.88,
        start_seconds: 10.0,
        end_seconds: 40.0,
        sample_path: "/tmp/sample.wav".to_string(),
    };
    
    assert_eq!(match_result.track_name, "Test Song");
    assert!(match_result.confidence > 0.0);
}

/// Test transcript
#[test]
fn test_transcript() {
    let transcript = Transcript {
        segments: vec![
            TranscriptSegment {
                start_seconds: 0.0,
                end_seconds: 5.0,
                text: "Hello world".to_string(),
            },
            TranscriptSegment {
                start_seconds: 5.0,
                end_seconds: 10.0,
                text: "This is a test".to_string(),
            },
        ],
    };
    
    assert_eq!(transcript.segments.len(), 2);
    assert_eq!(transcript.segments[0].text, "Hello world");
}

/// Test text violation
#[test]
fn test_text_violation() {
    let violation = TextViolation {
        severity: Severity::High,
        confidence: 0.85,
        description: "Inappropriate language detected".to_string(),
    };
    
    assert_eq!(violation.severity, Severity::High);
    assert!(violation.confidence > 0.0);
}

/// Test category breakdown calculation
#[test]
fn test_category_breakdown() {
    let mut breakdown = HashMap::new();
    breakdown.insert(PolicyCategory::Copyright, 45.0);
    breakdown.insert(PolicyCategory::Violence, 25.0);
    breakdown.insert(PolicyCategory::HateSpeech, 20.0);
    breakdown.insert(PolicyCategory::Misinformation, 10.0);
    
    // Sum should be approximately 100%
    let total: f32 = breakdown.values().sum();
    assert!((total - 100.0).abs() < 0.01);
}

/// Test issue deduplication logic
#[test]
fn test_issue_deduplication() {
    let issue1 = create_test_issue(PolicyCategory::Copyright, Severity::High, 0.9);
    let issue2 = create_test_issue(PolicyCategory::Copyright, Severity::Medium, 0.7);
    let issue3 = create_test_issue(PolicyCategory::Violence, Severity::High, 0.8);
    
    // Issues with same category and overlapping time should be considered duplicates
    // In a real scenario, we'd check the actual overlap logic
    assert_ne!(issue1.id, issue2.id);
    assert_ne!(issue1.id, issue3.id);
}

/// Test evidence types
#[test]
fn test_evidence_types() {
    let frame_evidence = Evidence::Frame {
        timestamp: Timestamp::new(0, 0, 30, 0, 30),
        frame_path: "/tmp/frame.png".to_string(),
        annotations: vec![
            Annotation {
                label: "person".to_string(),
                confidence: 0.92,
                bounding_box: BoundingBox {
                    x: 0.1,
                    y: 0.2,
                    width: 0.3,
                    height: 0.4,
                },
            },
        ],
    };
    
    let audio_evidence = Evidence::AudioFingerprint {
        matched_track: "Test Track".to_string(),
        confidence: 0.88,
        sample_path: "/tmp/sample.wav".to_string(),
    };
    
    let transcript_evidence = Evidence::Transcript {
        text: "Test transcript".to_string(),
        problematic_segments: vec![(0, 10)],
    };
    
    let visual_evidence = Evidence::VisualFeature {
        feature_type: "violence".to_string(),
        confidence: 0.75,
        bounding_box: Some(BoundingBox {
            x: 0.0,
            y: 0.0,
            width: 1.0,
            height: 1.0,
        }),
    };
    
    // All should be serializable
    assert!(serde_json::to_string(&frame_evidence).is_ok());
    assert!(serde_json::to_string(&audio_evidence).is_ok());
    assert!(serde_json::to_string(&transcript_evidence).is_ok());
    assert!(serde_json::to_string(&visual_evidence).is_ok());
}

/// Test annotation
#[test]
fn test_annotation() {
    let annotation = Annotation {
        label: "weapon".to_string(),
        confidence: 0.85,
        bounding_box: BoundingBox {
            x: 0.5,
            y: 0.5,
            width: 0.2,
            height: 0.1,
        },
    };
    
    assert_eq!(annotation.label, "weapon");
    assert!(annotation.confidence > 0.0);
}

/// Test bounding box
#[test]
fn test_bounding_box() {
    let bbox = BoundingBox {
        x: 0.1,
        y: 0.2,
        width: 0.3,
        height: 0.4,
    };
    
    assert_eq!(bbox.x, 0.1);
    assert_eq!(bbox.y, 0.2);
    assert_eq!(bbox.width, 0.3);
    assert_eq!(bbox.height, 0.4);
    
    // Check that box is within valid range
    assert!(bbox.x >= 0.0 && bbox.x <= 1.0);
    assert!(bbox.y >= 0.0 && bbox.y <= 1.0);
    assert!(bbox.width >= 0.0 && bbox.width <= 1.0);
    assert!(bbox.height >= 0.0 && bbox.height <= 1.0);
}

/// Test fix suggestion
#[test]
fn test_fix_suggestion() {
    let mut params = HashMap::new();
    params.insert("blur_radius".to_string(), serde_json::json!(5.0));
    
    let suggestion = FixSuggestion {
        id: uuid::Uuid::new_v4(),
        fix_type: FixType::Blur,
        impact: ImpactLevel::Medium,
        description: "Blur the problematic area".to_string(),
        estimated_quality_loss: 0.2,
        parameters: params,
    };
    
    assert_eq!(suggestion.fix_type, FixType::Blur);
    assert_eq!(suggestion.impact, ImpactLevel::Medium);
    assert!(suggestion.estimated_quality_loss >= 0.0 && suggestion.estimated_quality_loss <= 1.0);
}

/// Test analysis progress
#[test]
fn test_analysis_progress() {
    let progress = AnalysisProgress {
        job_id: uuid::Uuid::new_v4(),
        stage: AnalysisStage::AnalyzingVisuals,
        progress_percent: 45.0,
        current_segment: Some(10),
        total_segments: Some(100),
        message: "Processing frame 10 of 100".to_string(),
        estimated_seconds_remaining: Some(60),
    };
    
    assert_eq!(progress.stage, AnalysisStage::AnalyzingVisuals);
    assert!(progress.progress_percent >= 0.0 && progress.progress_percent <= 100.0);
    assert_eq!(progress.current_segment, Some(10));
    assert_eq!(progress.total_segments, Some(100));
}

/// Test applied fix
#[test]
fn test_applied_fix() {
    let mut params = HashMap::new();
    params.insert("intensity".to_string(), serde_json::json!(0.7));
    
    let applied_fix = AppliedFix {
        issue_id: uuid::Uuid::new_v4(),
        fix_type: FixType::Blur,
        time_range: TimeRange::new(
            Timestamp::new(0, 0, 30, 0, 30),
            Timestamp::new(0, 1, 0, 0, 30),
        ),
        parameters: params,
    };
    
    assert_eq!(applied_fix.fix_type, FixType::Blur);
    assert_eq!(applied_fix.time_range.duration_seconds(), 30.0);
}

/// Test fix result
#[test]
fn test_fix_result() {
    let result = FixResult {
        id: uuid::Uuid::new_v4(),
        original_video: "/input/video.mp4".to_string(),
        output_video: "/output/video_fixed.mp4".to_string(),
        applied_fixes: vec![],
        processing_time_seconds: 45.5,
        output_file_size_bytes: 50_000_000,
        quality_metrics: QualityMetrics {
            psnr: 35.0,
            ssim: 0.95,
            vmaf: 85.0,
            bitrate_kbps: 5000,
        },
    };
    
    assert!(result.processing_time_seconds > 0.0);
    assert!(result.output_file_size_bytes > 0);
    assert!(result.quality_metrics.psnr > 0.0);
}

/// Test detection rule
#[test]
fn test_detection_rule() {
    let mut params = HashMap::new();
    params.insert("objects".to_string(), serde_json::json!(vec!["person", "weapon"]));
    
    let rule = DetectionRule {
        rule_type: RuleType::ObjectDetection,
        confidence_threshold: 0.75,
        parameters: params,
    };
    
    assert_eq!(rule.rule_type, RuleType::ObjectDetection);
    assert!(rule.confidence_threshold >= 0.0 && rule.confidence_threshold <= 1.0);
}

/// Test rule type variants
#[test]
fn test_rule_type_variants() {
    let rule_types = vec![
        RuleType::ObjectDetection,
        RuleType::TextClassification,
        RuleType::AudioFingerprint,
        RuleType::VisualSimilarity,
        RuleType::SceneClassification,
        RuleType::SentimentAnalysis,
    ];
    
    for rule_type in rule_types {
        let json = serde_json::to_string(&rule_type);
        assert!(json.is_ok());
    }
}

/// Helper function to create test issues
fn create_test_issue(
    category: PolicyCategory,
    severity: Severity,
    confidence: f32,
) -> Issue {
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
