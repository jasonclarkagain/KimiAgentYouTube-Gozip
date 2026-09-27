//! Core type definitions for ContentGuard AI

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Analysis depth level
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AnalysisDepth {
    /// Quick scan (~30s for 10min video)
    Fast,
    /// Standard analysis (~2min for 10min video)
    Standard,
    /// Deep analysis (~5min for 10min video)
    Deep,
}

/// Content type being analyzed
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ContentType {
    Video,
    Audio,
    Text,
    Image,
}

/// Severity level for violations
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

/// Policy category for violations
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PolicyCategory {
    Copyright,
    Misinformation,
    HateSpeech,
    Violence,
    Harassment,
    Spam,
    Privacy,
    ChildSafety,
    SelfHarm,
    DangerousContent,
}

impl PolicyCategory {
    pub fn display_name(&self) -> &'static str {
        match self {
            PolicyCategory::Copyright => "Copyright",
            PolicyCategory::Misinformation => "Misinformation",
            PolicyCategory::HateSpeech => "Hate Speech",
            PolicyCategory::Violence => "Violence",
            PolicyCategory::Harassment => "Harassment",
            PolicyCategory::Spam => "Spam",
            PolicyCategory::Privacy => "Privacy",
            PolicyCategory::ChildSafety => "Child Safety",
            PolicyCategory::SelfHarm => "Self-Harm",
            PolicyCategory::DangerousContent => "Dangerous Content",
        }
    }
    
    pub fn description(&self) -> &'static str {
        match self {
            PolicyCategory::Copyright => "Content that infringes on intellectual property rights",
            PolicyCategory::Misinformation => "Content that spreads false or misleading information",
            PolicyCategory::HateSpeech => "Content that promotes hatred against protected groups",
            PolicyCategory::Violence => "Content depicting graphic violence or gore",
            PolicyCategory::Harassment => "Content targeting individuals for harassment",
            PolicyCategory::Spam => "Deceptive practices, scams, or repetitive content",
            PolicyCategory::Privacy => "Content that violates personal privacy",
            PolicyCategory::ChildSafety => "Content that endangers minors",
            PolicyCategory::SelfHarm => "Content promoting self-harm or suicide",
            PolicyCategory::DangerousContent => "Content showing dangerous acts or challenges",
        }
    }
}

/// Timestamp for a specific frame/segment
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Timestamp {
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
    pub frames: u32,
    pub fps: u32,
}

impl Timestamp {
    pub fn new(hours: u32, minutes: u32, seconds: u32, frames: u32, fps: u32) -> Self {
        Self { hours, minutes, seconds, frames, fps }
    }
    
    pub fn from_seconds(total_seconds: f64, fps: u32) -> Self {
        let hours = (total_seconds / 3600.0) as u32;
        let minutes = ((total_seconds % 3600.0) / 60.0) as u32;
        let seconds = (total_seconds % 60.0) as u32;
        let frames = ((total_seconds.fract() * fps as f64) as u32).min(fps - 1);
        
        Self { hours, minutes, seconds, frames, fps }
    }
    
    pub fn to_seconds(&self) -> f64 {
        (self.hours * 3600 + self.minutes * 60 + self.seconds) as f64
            + (self.frames as f64 / self.fps as f64)
    }
    
    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}:{:02}.{:02}", self.hours, self.minutes, self.seconds, self.frames)
    }
}

/// A time range representing a segment
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct TimeRange {
    pub start: Timestamp,
    pub end: Timestamp,
}

impl TimeRange {
    pub fn new(start: Timestamp, end: Timestamp) -> Self {
        Self { start, end }
    }
    
    pub fn duration_seconds(&self) -> f64 {
        self.end.to_seconds() - self.start.to_seconds()
    }
    
    pub fn contains(&self, timestamp: &Timestamp) -> bool {
        let ts = timestamp.to_seconds();
        ts >= self.start.to_seconds() && ts <= self.end.to_seconds()
    }
}

/// A detected issue/violation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub id: Uuid,
    pub category: PolicyCategory,
    pub severity: Severity,
    pub confidence: f32, // 0.0 to 1.0
    pub time_range: TimeRange,
    pub description: String,
    pub evidence: Vec<Evidence>,
    pub suggested_fixes: Vec<FixSuggestion>,
    pub policy_reference: Option<String>,
}

/// Evidence supporting an issue detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Evidence {
    /// Frame screenshot with bounding boxes
    Frame {
        timestamp: Timestamp,
        frame_path: String,
        annotations: Vec<Annotation>,
    },
    /// Audio fingerprint match
    AudioFingerprint {
        matched_track: String,
        confidence: f32,
        sample_path: String,
    },
    /// Transcribed text with problematic segments
    Transcript {
        text: String,
        problematic_segments: Vec<(usize, usize)>,
    },
    /// Visual feature detection
    VisualFeature {
        feature_type: String,
        confidence: f32,
        bounding_box: Option<BoundingBox>,
    },
}

/// Annotation for frame evidence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Annotation {
    pub label: String,
    pub confidence: f32,
    pub bounding_box: BoundingBox,
}

/// Bounding box for object detection
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BoundingBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// Suggested fix for an issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixSuggestion {
    pub id: Uuid,
    pub fix_type: FixType,
    pub impact: ImpactLevel,
    pub description: String,
    pub estimated_quality_loss: f32, // 0.0 to 1.0
    pub parameters: HashMap<String, serde_json::Value>,
}

/// Type of fix
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum FixType {
    /// Remove the segment entirely
    Remove,
    /// Blur the video in the time range
    Blur,
    /// Replace audio track
    ReplaceAudio,
    /// Mute audio in the segment
    Mute,
    /// Add context card overlay
    AddContext,
    /// Speed up the segment
    SpeedUp,
    /// Apply color grading to reduce impact
    ColorGrade,
    /// Crop to remove problematic area
    Crop,
}

/// Impact level of a fix
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ImpactLevel {
    Low,
    Medium,
    High,
}

impl ImpactLevel {
    pub fn display_name(&self) -> &'static str {
        match self {
            ImpactLevel::Low => "Low Impact",
            ImpactLevel::Medium => "Medium Impact",
            ImpactLevel::High => "High Impact",
        }
    }
}

/// Complete analysis report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisReport {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub video_path: String,
    pub video_duration_seconds: f64,
    pub video_resolution: (u32, u32),
    pub video_fps: f64,
    pub strike_probability: f32, // 0.0 to 100.0
    pub overall_risk_score: f32, // 0.0 to 100.0
    pub issues: Vec<Issue>,
    pub category_breakdown: HashMap<PolicyCategory, f32>,
    pub processing_time_seconds: f64,
    pub analysis_depth: AnalysisDepth,
    pub metadata: HashMap<String, serde_json::Value>,
}

impl AnalysisReport {
    pub fn new(video_path: String, depth: AnalysisDepth) -> Self {
        Self {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            video_path,
            video_duration_seconds: 0.0,
            video_resolution: (0, 0),
            video_fps: 0.0,
            strike_probability: 0.0,
            overall_risk_score: 0.0,
            issues: Vec::new(),
            category_breakdown: HashMap::new(),
            processing_time_seconds: 0.0,
            analysis_depth: depth,
            metadata: HashMap::new(),
        }
    }
    
    pub fn calculate_strike_probability(&mut self) {
        if self.issues.is_empty() {
            self.strike_probability = 0.0;
            return;
        }
        
        // Weight issues by severity and confidence
        let mut total_weight = 0.0;
        let mut weighted_sum = 0.0;
        
        for issue in &self.issues {
            let severity_weight = match issue.severity {
                Severity::Low => 0.25,
                Severity::Medium => 0.5,
                Severity::High => 0.75,
                Severity::Critical => 1.0,
            };
            
            weighted_sum += severity_weight * issue.confidence;
            total_weight += 1.0;
        }
        
        // Apply non-linear scaling for multiple issues
        let base_probability = (weighted_sum / total_weight) * 100.0;
        let issue_count_factor = 1.0 + (self.issues.len() as f32 - 1.0) * 0.1;
        
        self.strike_probability = (base_probability * issue_count_factor).min(100.0);
        self.overall_risk_score = self.strike_probability;
    }
    
    pub fn issues_by_severity(&self, severity: Severity) -> Vec<&Issue> {
        self.issues.iter()
            .filter(|i| i.severity == severity)
            .collect()
    }
    
    pub fn issues_by_category(&self, category: PolicyCategory) -> Vec<&Issue> {
        self.issues.iter()
            .filter(|i| i.category == category)
            .collect()
    }
}

/// Analysis progress update
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisProgress {
    pub job_id: Uuid,
    pub stage: AnalysisStage,
    pub progress_percent: f32,
    pub current_segment: Option<usize>,
    pub total_segments: Option<usize>,
    pub message: String,
    pub estimated_seconds_remaining: Option<u32>,
}

/// Analysis processing stage
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AnalysisStage {
    Initializing,
    ExtractingFrames,
    AnalyzingVisuals,
    AnalyzingAudio,
    AnalyzingText,
    CrossReferencing,
    GeneratingReport,
    Complete,
    Error,
}

impl AnalysisStage {
    pub fn display_name(&self) -> &'static str {
        match self {
            AnalysisStage::Initializing => "Initializing",
            AnalysisStage::ExtractingFrames => "Extracting Frames",
            AnalysisStage::AnalyzingVisuals => "Analyzing Visuals",
            AnalysisStage::AnalyzingAudio => "Analyzing Audio",
            AnalysisStage::AnalyzingText => "Analyzing Text",
            AnalysisStage::CrossReferencing => "Cross-Referencing",
            AnalysisStage::GeneratingReport => "Generating Report",
            AnalysisStage::Complete => "Complete",
            AnalysisStage::Error => "Error",
        }
    }
}

/// Fix application result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixResult {
    pub id: Uuid,
    pub original_video: String,
    pub output_video: String,
    pub applied_fixes: Vec<AppliedFix>,
    pub processing_time_seconds: f64,
    pub output_file_size_bytes: u64,
    pub quality_metrics: QualityMetrics,
}

/// An applied fix
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppliedFix {
    pub issue_id: Uuid,
    pub fix_type: FixType,
    pub time_range: TimeRange,
    pub parameters: HashMap<String, serde_json::Value>,
}

/// Quality metrics for fixed video
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetrics {
    pub psnr: f32,
    pub ssim: f32,
    pub vmaf: f32,
    pub bitrate_kbps: u32,
}

/// Policy definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub id: String,
    pub category: PolicyCategory,
    pub name: String,
    pub description: String,
    pub guidelines_url: String,
    pub detection_rules: Vec<DetectionRule>,
    pub version: String,
    pub updated_at: DateTime<Utc>,
}

/// Detection rule for policy enforcement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionRule {
    pub rule_type: RuleType,
    pub confidence_threshold: f32,
    pub parameters: HashMap<String, serde_json::Value>,
}

/// Type of detection rule
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RuleType {
    ObjectDetection,
    TextClassification,
    AudioFingerprint,
    VisualSimilarity,
    SceneClassification,
    SentimentAnalysis,
}

/// API request/response types
pub mod api {
    use super::*;
    
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AnalyzeRequest {
        pub video_path: String,
        pub depth: Option<AnalysisDepth>,
        pub callback_url: Option<String>,
    }
    
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AnalyzeResponse {
        pub job_id: Uuid,
        pub status: String,
        pub estimated_seconds: u32,
    }
    
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ReportResponse {
        pub job_id: Uuid,
        pub report: Option<AnalysisReport>,
        pub status: String,
        pub progress: Option<AnalysisProgress>,
    }
    
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct FixRequest {
        pub video_path: String,
        pub report_id: Uuid,
        pub selected_fixes: Vec<Uuid>,
        pub intensity: Option<u8>,
        pub preserve_audio: Option<bool>,
    }
    
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct HealthResponse {
        pub status: String,
        pub version: String,
        pub gpu_available: bool,
        pub models_loaded: Vec<String>,
    }
}
