//! Content analysis engine for detecting policy violations

use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;

use anyhow::{Context, Result};
use tokio::sync::{mpsc, RwLock};
use tracing::{debug, info, instrument, warn};
use uuid::Uuid;

use crate::config::AppConfig;
use crate::engine::{AnalysisEngine, FrameExtractor};
use crate::policy::PolicyManager;
use crate::types::*;

/// Main content analyzer
pub struct ContentAnalyzer {
    config: Arc<AppConfig>,
    engine: AnalysisEngine,
    policy_manager: Arc<RwLock<PolicyManager>>,
    progress_tx: mpsc::Sender<AnalysisProgress>,
}

impl ContentAnalyzer {
    /// Create a new content analyzer
    pub async fn new(config: &AppConfig, enable_gpu: bool) -> Result<Self> {
        let config = Arc::new(config.clone());
        
        // Initialize analysis engine
        let engine = AnalysisEngine::new(&config, enable_gpu).await
            .context("Failed to initialize analysis engine")?;
        
        // Initialize policy manager
        let policy_manager = PolicyManager::new(&config).await?;
        
        // Create progress channel
        let (progress_tx, _progress_rx) = mpsc::channel(100);
        
        Ok(Self {
            config,
            engine,
            policy_manager: Arc::new(RwLock::new(policy_manager)),
            progress_tx,
        })
    }
    
    /// Analyze a video file
    #[instrument(skip(self, video_path))]
    pub async fn analyze_video(
        &self,
        video_path: &str,
        depth: AnalysisDepth,
    ) -> Result<AnalysisReport> {
        let start_time = Instant::now();
        let job_id = Uuid::new_v4();
        
        info!("Starting video analysis: job_id={}, path={}", job_id, video_path);
        
        // Send initial progress
        self.send_progress(AnalysisProgress {
            job_id,
            stage: AnalysisStage::Initializing,
            progress_percent: 0.0,
            current_segment: None,
            total_segments: None,
            message: "Initializing analysis...".to_string(),
            estimated_seconds_remaining: None,
        }).await?;
        
        // Get video metadata
        let video_metadata = self.engine.get_video_metadata(video_path).await?;
        
        let mut report = AnalysisReport::new(video_path.to_string(), depth);
        report.video_duration_seconds = video_metadata.duration_seconds;
        report.video_resolution = video_metadata.resolution;
        report.video_fps = video_metadata.fps;
        
        // Determine frame extraction interval based on depth
        let frame_interval = match depth {
            AnalysisDepth::Fast => self.config.analysis.frame_interval_fast,
            AnalysisDepth::Standard => self.config.analysis.frame_interval_standard,
            AnalysisDepth::Deep => self.config.analysis.frame_interval_deep,
        };
        
        // Extract frames
        self.send_progress(AnalysisProgress {
            job_id,
            stage: AnalysisStage::ExtractingFrames,
            progress_percent: 5.0,
            current_segment: None,
            total_segments: None,
            message: "Extracting frames...".to_string(),
            estimated_seconds_remaining: Some((video_metadata.duration_seconds / 10.0) as u32),
        }).await?;
        
        let frames = self.engine.extract_frames(video_path, frame_interval).await?;
        let total_frames = frames.len();
        
        info!("Extracted {} frames from video", total_frames);
        
        // Analyze visuals
        self.send_progress(AnalysisProgress {
            job_id,
            stage: AnalysisStage::AnalyzingVisuals,
            progress_percent: 15.0,
            current_segment: Some(0),
            total_segments: Some(total_frames),
            message: "Analyzing visual content...".to_string(),
            estimated_seconds_remaining: Some((total_frames as f64 * 0.5) as u32),
        }).await?;
        
        let visual_issues = self.analyze_visuals(&frames, video_metadata.fps).await?;
        report.issues.extend(visual_issues);
        
        // Analyze audio
        self.send_progress(AnalysisProgress {
            job_id,
            stage: AnalysisStage::AnalyzingAudio,
            progress_percent: 50.0,
            current_segment: None,
            total_segments: None,
            message: "Analyzing audio content...".to_string(),
            estimated_seconds_remaining: Some((video_metadata.duration_seconds / 5.0) as u32),
        }).await?;
        
        let audio_issues = self.analyze_audio(video_path).await?;
        report.issues.extend(audio_issues);
        
        // Analyze text/transcript
        self.send_progress(AnalysisProgress {
            job_id,
            stage: AnalysisStage::AnalyzingText,
            progress_percent: 75.0,
            current_segment: None,
            total_segments: None,
            message: "Analyzing text content...".to_string(),
            estimated_seconds_remaining: Some(30),
        }).await?;
        
        let text_issues = self.analyze_text(video_path).await?;
        report.issues.extend(text_issues);
        
        // Cross-reference and deduplicate issues
        self.send_progress(AnalysisProgress {
            job_id,
            stage: AnalysisStage::CrossReferencing,
            progress_percent: 90.0,
            current_segment: None,
            total_segments: None,
            message: "Cross-referencing results...".to_string(),
            estimated_seconds_remaining: Some(10),
        }).await?;
        
        self.deduplicate_issues(&mut report.issues);
        
        // Calculate category breakdown
        report.category_breakdown = self.calculate_category_breakdown(&report.issues);
        
        // Calculate strike probability
        report.calculate_strike_probability();
        
        // Finalize report
        report.processing_time_seconds = start_time.elapsed().as_secs_f64();
        
        self.send_progress(AnalysisProgress {
            job_id,
            stage: AnalysisStage::Complete,
            progress_percent: 100.0,
            current_segment: None,
            total_segments: None,
            message: "Analysis complete".to_string(),
            estimated_seconds_remaining: Some(0),
        }).await?;
        
        info!(
            "Analysis complete: job_id={}, duration={:.2}s, issues={}",
            job_id,
            report.processing_time_seconds,
            report.issues.len()
        );
        
        Ok(report)
    }
    
    /// Analyze visual content in frames
    #[instrument(skip(self, frames))]
    async fn analyze_visuals(
        &self,
        frames: &[FrameData],
        fps: f64,
    ) -> Result<Vec<Issue>> {
        let mut issues = Vec::new();
        
        // Get active policies
        let policy_manager = self.policy_manager.read().await;
        let visual_policies = policy_manager.get_visual_policies().await?;
        drop(policy_manager);
        
        // Process frames in batches
        let batch_size = self.config.analysis.gpu_batch_size;
        
        for (batch_idx, batch) in frames.chunks(batch_size).enumerate() {
            debug!("Processing visual batch {}", batch_idx);
            
            // Run object detection
            let detections = self.engine.detect_objects(batch).await?;
            
            // Run scene classification
            let scenes = self.engine.classify_scenes(batch).await?;
            
            // Check against policies
            for (frame_idx, (detection, scene)) in detections.iter().zip(scenes.iter()).enumerate() {
                let global_frame_idx = batch_idx * batch_size + frame_idx;
                let frame = &frames[global_frame_idx];
                let timestamp = Timestamp::from_seconds(frame.timestamp_seconds, fps as u32);
                
                // Check object detections
                for obj in &detection.objects {
                    if let Some(policy) = visual_policies.iter()
                        .find(|p| p.matches_object(&obj.label)) {
                        
                        if obj.confidence >= policy.min_confidence() {
                            let issue = Issue {
                                id: Uuid::new_v4(),
                                category: policy.category(),
                                severity: policy.severity_for_confidence(obj.confidence),
                                confidence: obj.confidence,
                                time_range: TimeRange::new(
                                    timestamp.clone(),
                                    Timestamp::from_seconds(
                                        frame.timestamp_seconds + 1.0,
                                        fps as u32
                                    ),
                                ),
                                description: format!(
                                    "Detected {} with {:.1}% confidence",
                                    obj.label,
                                    obj.confidence * 100.0
                                ),
                                evidence: vec![Evidence::VisualFeature {
                                    feature_type: obj.label.clone(),
                                    confidence: obj.confidence,
                                    bounding_box: Some(obj.bounding_box),
                                }],
                                suggested_fixes: policy.suggested_fixes(),
                                policy_reference: Some(policy.id().to_string()),
                            };
                            issues.push(issue);
                        }
                    }
                }
                
                // Check scene classifications
                for scene_class in &scene.classes {
                    if let Some(policy) = visual_policies.iter()
                        .find(|p| p.matches_scene(&scene_class.label)) {
                        
                        if scene_class.confidence >= policy.min_confidence() {
                            let issue = Issue {
                                id: Uuid::new_v4(),
                                category: policy.category(),
                                severity: policy.severity_for_confidence(scene_class.confidence),
                                confidence: scene_class.confidence,
                                time_range: TimeRange::new(
                                    timestamp.clone(),
                                    Timestamp::from_seconds(
                                        frame.timestamp_seconds + 2.0,
                                        fps as u32
                                    ),
                                ),
                                description: format!(
                                    "Scene classified as {} with {:.1}% confidence",
                                    scene_class.label,
                                    scene_class.confidence * 100.0
                                ),
                                evidence: vec![Evidence::VisualFeature {
                                    feature_type: scene_class.label.clone(),
                                    confidence: scene_class.confidence,
                                    bounding_box: None,
                                }],
                                suggested_fixes: policy.suggested_fixes(),
                                policy_reference: Some(policy.id().to_string()),
                            };
                            issues.push(issue);
                        }
                    }
                }
            }
        }
        
        Ok(issues)
    }
    
    /// Analyze audio content
    #[instrument(skip(self, video_path))]
    async fn analyze_audio(&self, video_path: &str) -> Result<Vec<Issue>> {
        let mut issues = Vec::new();
        
        if !self.config.analysis.enable_audio_fingerprint {
            return Ok(issues);
        }
        
        // Extract audio fingerprint
        let fingerprint = self.engine.extract_audio_fingerprint(video_path).await?;
        
        // Match against known copyrighted tracks
        let matches = self.engine.match_audio_fingerprint(&fingerprint).await?;
        
        for match_result in matches {
            if match_result.confidence >= self.config.analysis.min_confidence_threshold {
                let issue = Issue {
                    id: Uuid::new_v4(),
                    category: PolicyCategory::Copyright,
                    severity: Severity::High,
                    confidence: match_result.confidence,
                    time_range: TimeRange::new(
                        Timestamp::from_seconds(match_result.start_seconds, 30),
                        Timestamp::from_seconds(match_result.end_seconds, 30),
                    ),
                    description: format!(
                        "Audio matches copyrighted track: {} (confidence: {:.1}%)",
                        match_result.track_name,
                        match_result.confidence * 100.0
                    ),
                    evidence: vec![Evidence::AudioFingerprint {
                        matched_track: match_result.track_name,
                        confidence: match_result.confidence,
                        sample_path: match_result.sample_path,
                    }],
                    suggested_fixes: vec![
                        FixSuggestion {
                            id: Uuid::new_v4(),
                            fix_type: FixType::ReplaceAudio,
                            impact: ImpactLevel::Low,
                            description: "Replace with royalty-free alternative".to_string(),
                            estimated_quality_loss: 0.1,
                            parameters: HashMap::new(),
                        },
                        FixSuggestion {
                            id: Uuid::new_v4(),
                            fix_type: FixType::Mute,
                            impact: ImpactLevel::Medium,
                            description: "Mute audio in this segment".to_string(),
                            estimated_quality_loss: 0.3,
                            parameters: HashMap::new(),
                        },
                    ],
                    policy_reference: Some("copyright-audio".to_string()),
                };
                issues.push(issue);
            }
        }
        
        Ok(issues)
    }
    
    /// Analyze text/transcript content
    #[instrument(skip(self, video_path))]
    async fn analyze_text(&self, video_path: &str) -> Result<Vec<Issue>> {
        let mut issues = Vec::new();
        
        if !self.config.analysis.enable_text_analysis {
            return Ok(issues);
        }
        
        // Extract transcript
        let transcript = self.engine.extract_transcript(video_path).await?;
        
        // Get text analysis policies
        let policy_manager = self.policy_manager.read().await;
        let text_policies = policy_manager.get_text_policies().await?;
        drop(policy_manager);
        
        // Analyze transcript segments
        for segment in &transcript.segments {
            for policy in &text_policies {
                if let Some(violation) = policy.check_text(&segment.text).await? {
                    let issue = Issue {
                        id: Uuid::new_v4(),
                        category: policy.category(),
                        severity: violation.severity,
                        confidence: violation.confidence,
                        time_range: TimeRange::new(
                            Timestamp::from_seconds(segment.start_seconds, 30),
                            Timestamp::from_seconds(segment.end_seconds, 30),
                        ),
                        description: violation.description,
                        evidence: vec![Evidence::Transcript {
                            text: segment.text.clone(),
                            problematic_segments: vec![(0, segment.text.len())],
                        }],
                        suggested_fixes: policy.suggested_fixes(),
                        policy_reference: Some(policy.id().to_string()),
                    };
                    issues.push(issue);
                }
            }
        }
        
        Ok(issues)
    }
    
    /// Deduplicate overlapping issues
    fn deduplicate_issues(&self, issues: &mut Vec<Issue>) {
        // Sort by confidence (highest first)
        issues.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
        
        // Remove overlapping issues with lower confidence
        let mut result = Vec::new();
        
        for issue in issues.drain(..) {
            let overlaps = result.iter().any(|existing: &Issue| {
                self.issues_overlap(existing, &issue)
            });
            
            if !overlaps {
                result.push(issue);
            }
        }
        
        *issues = result;
    }
    
    /// Check if two issues overlap in time
    fn issues_overlap(&self, a: &Issue, b: &Issue) -> bool {
        // Check if categories are the same
        if a.category != b.category {
            return false;
        }
        
        // Check time overlap
        let a_start = a.time_range.start.to_seconds();
        let a_end = a.time_range.end.to_seconds();
        let b_start = b.time_range.start.to_seconds();
        let b_end = b.time_range.end.to_seconds();
        
        a_start < b_end && a_end > b_start
    }
    
    /// Calculate category breakdown
    fn calculate_category_breakdown(&self, issues: &[Issue]) -> HashMap<PolicyCategory, f32> {
        let mut breakdown = HashMap::new();
        
        for issue in issues {
            let weight = match issue.severity {
                Severity::Low => 0.25,
                Severity::Medium => 0.5,
                Severity::High => 0.75,
                Severity::Critical => 1.0,
            } * issue.confidence;
            
            *breakdown.entry(issue.category).or_insert(0.0) += weight;
        }
        
        // Normalize to percentages
        let total: f32 = breakdown.values().sum();
        if total > 0.0 {
            for value in breakdown.values_mut() {
                *value = (*value / total * 100.0).min(100.0);
            }
        }
        
        breakdown
    }
    
    /// Send progress update
    async fn send_progress(&self, progress: AnalysisProgress) -> Result<()> {
        // In a real implementation, this would broadcast to WebSocket clients
        // For now, just log the progress
        debug!(
            "Progress: {}% - {} - {}",
            progress.progress_percent,
            progress.stage.display_name(),
            progress.message
        );
        Ok(())
    }
}

/// Frame data extracted from video
#[derive(Debug, Clone)]
pub struct FrameData {
    pub timestamp_seconds: f64,
    pub frame_number: usize,
    pub image_data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

/// Object detection result
#[derive(Debug, Clone)]
pub struct ObjectDetection {
    pub label: String,
    pub confidence: f32,
    pub bounding_box: BoundingBox,
}

/// Scene classification result
#[derive(Debug, Clone)]
pub struct SceneClassification {
    pub classes: Vec<SceneClass>,
}

#[derive(Debug, Clone)]
pub struct SceneClass {
    pub label: String,
    pub confidence: f32,
}

/// Audio fingerprint match result
#[derive(Debug, Clone)]
pub struct AudioMatch {
    pub track_name: String,
    pub confidence: f32,
    pub start_seconds: f64,
    pub end_seconds: f64,
    pub sample_path: String,
}

/// Transcript data
#[derive(Debug, Clone)]
pub struct Transcript {
    pub segments: Vec<TranscriptSegment>,
}

#[derive(Debug, Clone)]
pub struct TranscriptSegment {
    pub start_seconds: f64,
    pub end_seconds: f64,
    pub text: String,
}

/// Text violation detection
#[derive(Debug, Clone)]
pub struct TextViolation {
    pub severity: Severity,
    pub confidence: f32,
    pub description: String,
}
