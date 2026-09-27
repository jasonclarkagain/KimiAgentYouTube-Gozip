//! Policy management for YouTube Community Guidelines

use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::fs;
use tracing::{info, warn, instrument};
use uuid::Uuid;

use crate::config::AppConfig;
use crate::types::*;

/// Policy manager for loading and updating YouTube policies
pub struct PolicyManager {
    config: Arc<AppConfig>,
    policies: HashMap<String, Policy>,
    last_updated: DateTime<Utc>,
}

impl PolicyManager {
    /// Create a new policy manager
    pub async fn new(config: &Arc<AppConfig>) -> Result<Self> {
        let mut manager = Self {
            config: config.clone(),
            policies: HashMap::new(),
            last_updated: Utc::now(),
        };
        
        // Load policies from disk
        manager.load_policies().await?;
        
        // If no policies loaded, create defaults
        if manager.policies.is_empty() {
            info!("No policies found, creating defaults");
            manager.create_default_policies().await?;
        }
        
        Ok(manager)
    }
    
    /// Load policies from disk
    #[instrument(skip(self))]
    async fn load_policies(&mut self) -> Result<()> {
        let policies_dir = &self.config.policies_dir;
        
        if !policies_dir.exists() {
            fs::create_dir_all(policies_dir).await?;
            return Ok(());
        }
        
        let mut entries = fs::read_dir(policies_dir).await?;
        
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            
            if path.extension().map(|e| e == "json").unwrap_or(false) {
                match self.load_policy_file(&path).await {
                    Ok(policy) => {
                        self.policies.insert(policy.id.clone(), policy);
                    }
                    Err(e) => {
                        warn!("Failed to load policy from {:?}: {}", path, e);
                    }
                }
            }
        }
        
        info!("Loaded {} policies", self.policies.len());
        Ok(())
    }
    
    /// Load a single policy file
    async fn load_policy_file(&self, path: &Path) -> Result<Policy> {
        let content = fs::read_to_string(path).await?;
        let policy: Policy = serde_json::from_str(&content)?;
        Ok(policy)
    }
    
    /// Create default policies
    async fn create_default_policies(&mut self) -> Result<()> {
        let defaults = vec![
            self.create_copyright_policy(),
            self.create_violence_policy(),
            self.create_hate_speech_policy(),
            self.create_misinformation_policy(),
            self.create_harassment_policy(),
            self.create_privacy_policy(),
        ];
        
        for policy in defaults {
            self.save_policy(&policy).await?;
            self.policies.insert(policy.id.clone(), policy);
        }
        
        Ok(())
    }
    
    /// Save policy to disk
    async fn save_policy(&self, policy: &Policy) -> Result<()> {
        let path = self.config.policies_dir.join(format!("{}.json", policy.id));
        let content = serde_json::to_string_pretty(policy)?;
        fs::write(&path, content).await?;
        Ok(())
    }
    
    /// Create copyright policy
    fn create_copyright_policy(&self) -> Policy {
        Policy {
            id: "copyright".to_string(),
            category: PolicyCategory::Copyright,
            name: "Copyright Infringement".to_string(),
            description: "Detects use of copyrighted material without proper authorization".to_string(),
            guidelines_url: "https://www.youtube.com/howyoutubeworks/policies/copyright/".to_string(),
            detection_rules: vec![
                DetectionRule {
                    rule_type: RuleType::AudioFingerprint,
                    confidence_threshold: 0.7,
                    parameters: HashMap::new(),
                },
                DetectionRule {
                    rule_type: RuleType::VisualSimilarity,
                    confidence_threshold: 0.8,
                    parameters: HashMap::new(),
                },
            ],
            version: "1.0.0".to_string(),
            updated_at: Utc::now(),
        }
    }
    
    /// Create violence policy
    fn create_violence_policy(&self) -> Policy {
        Policy {
            id: "violence".to_string(),
            category: PolicyCategory::Violence,
            name: "Violent or Graphic Content".to_string(),
            description: "Detects graphic violence, gore, or disturbing imagery".to_string(),
            guidelines_url: "https://www.youtube.com/howyoutubeworks/policies/community-guidelines/#violent-or-graphic-content".to_string(),
            detection_rules: vec![
                DetectionRule {
                    rule_type: RuleType::ObjectDetection,
                    confidence_threshold: 0.75,
                    parameters: {
                        let mut params = HashMap::new();
                        params.insert("objects".to_string(), serde_json::json!([
                            "blood", "weapon", "injury", "corpse"
                        ]));
                        params
                    },
                },
                DetectionRule {
                    rule_type: RuleType::SceneClassification,
                    confidence_threshold: 0.8,
                    parameters: {
                        let mut params = HashMap::new();
                        params.insert("scenes".to_string(), serde_json::json!([
                            "violence", "gore", "disturbing"
                        ]));
                        params
                    },
                },
            ],
            version: "1.0.0".to_string(),
            updated_at: Utc::now(),
        }
    }
    
    /// Create hate speech policy
    fn create_hate_speech_policy(&self) -> Policy {
        Policy {
            id: "hate-speech".to_string(),
            category: PolicyCategory::HateSpeech,
            name: "Hate Speech".to_string(),
            description: "Detects content that promotes hatred against protected groups".to_string(),
            guidelines_url: "https://www.youtube.com/howyoutubeworks/policies/community-guidelines/#hate-speech".to_string(),
            detection_rules: vec![
                DetectionRule {
                    rule_type: RuleType::TextClassification,
                    confidence_threshold: 0.8,
                    parameters: HashMap::new(),
                },
                DetectionRule {
                    rule_type: RuleType::SentimentAnalysis,
                    confidence_threshold: 0.75,
                    parameters: HashMap::new(),
                },
            ],
            version: "1.0.0".to_string(),
            updated_at: Utc::now(),
        }
    }
    
    /// Create misinformation policy
    fn create_misinformation_policy(&self) -> Policy {
        Policy {
            id: "misinformation".to_string(),
            category: PolicyCategory::Misinformation,
            name: "Misinformation".to_string(),
            description: "Detects content that spreads false or misleading information".to_string(),
            guidelines_url: "https://www.youtube.com/howyoutubeworks/policies/community-guidelines/#misinformation".to_string(),
            detection_rules: vec![
                DetectionRule {
                    rule_type: RuleType::TextClassification,
                    confidence_threshold: 0.7,
                    parameters: HashMap::new(),
                },
            ],
            version: "1.0.0".to_string(),
            updated_at: Utc::now(),
        }
    }
    
    /// Create harassment policy
    fn create_harassment_policy(&self) -> Policy {
        Policy {
            id: "harassment".to_string(),
            category: PolicyCategory::Harassment,
            name: "Harassment and Cyberbullying".to_string(),
            description: "Detects content targeting individuals for harassment".to_string(),
            guidelines_url: "https://www.youtube.com/howyoutubeworks/policies/community-guidelines/#harassment-cyberbullying".to_string(),
            detection_rules: vec![
                DetectionRule {
                    rule_type: RuleType::TextClassification,
                    confidence_threshold: 0.75,
                    parameters: HashMap::new(),
                },
            ],
            version: "1.0.0".to_string(),
            updated_at: Utc::now(),
        }
    }
    
    /// Create privacy policy
    fn create_privacy_policy(&self) -> Policy {
        Policy {
            id: "privacy".to_string(),
            category: PolicyCategory::Privacy,
            name: "Privacy Violations".to_string(),
            description: "Detects content that violates personal privacy".to_string(),
            guidelines_url: "https://www.youtube.com/howyoutubeworks/policies/community-guidelines/#privacy".to_string(),
            detection_rules: vec![
                DetectionRule {
                    rule_type: RuleType::ObjectDetection,
                    confidence_threshold: 0.8,
                    parameters: {
                        let mut params = HashMap::new();
                        params.insert("objects".to_string(), serde_json::json!([
                            "license_plate", "personal_document", "phone_number"
                        ]));
                        params
                    },
                },
            ],
            version: "1.0.0".to_string(),
            updated_at: Utc::now(),
        }
    }
    
    /// Update policies from remote source
    #[instrument(skip(self))]
    pub async fn update_policies(&mut self, force: bool) -> Result<bool> {
        // Check if update is needed
        if !force {
            let elapsed = Utc::now().signed_duration_since(self.last_updated);
            let hours_since_update = elapsed.num_hours() as u64;
            
            if hours_since_update < self.config.policy_updates.check_interval_hours {
                return Ok(false);
            }
        }
        
        info!("Checking for policy updates...");
        
        // In a real implementation, this would fetch from a remote API
        // For now, just update the timestamp
        self.last_updated = Utc::now();
        
        Ok(true)
    }
    
    /// Get all policies
    pub fn get_all_policies(&self) -> Vec<&Policy> {
        self.policies.values().collect()
    }
    
    /// Get policy by ID
    pub fn get_policy(&self, id: &str) -> Option<&Policy> {
        self.policies.get(id)
    }
    
    /// Get policies for visual analysis
    pub async fn get_visual_policies(&self) -> Result<Vec<VisualPolicy>> {
        let visual_policies: Vec<VisualPolicy> = self.policies
            .values()
            .filter(|p| p.detection_rules.iter().any(|r| {
                matches!(r.rule_type, RuleType::ObjectDetection | RuleType::SceneClassification | RuleType::VisualSimilarity)
            }))
            .map(|p| VisualPolicy::from_policy(p))
            .collect();
        
        Ok(visual_policies)
    }
    
    /// Get policies for text analysis
    pub async fn get_text_policies(&self) -> Result<Vec<TextPolicy>> {
        let text_policies: Vec<TextPolicy> = self.policies
            .values()
            .filter(|p| p.detection_rules.iter().any(|r| {
                matches!(r.rule_type, RuleType::TextClassification | RuleType::SentimentAnalysis)
            }))
            .map(|p| TextPolicy::from_policy(p))
            .collect();
        
        Ok(text_policies)
    }
    
    /// Get last update time
    pub fn last_updated(&self) -> DateTime<Utc> {
        self.last_updated
    }
}

/// Visual policy wrapper
pub struct VisualPolicy {
    policy_id: String,
    category: PolicyCategory,
    target_objects: Vec<String>,
    target_scenes: Vec<String>,
    min_confidence: f32,
}

impl VisualPolicy {
    fn from_policy(policy: &Policy) -> Self {
        let mut target_objects = Vec::new();
        let mut target_scenes = Vec::new();
        let mut min_confidence = 0.7f32;
        
        for rule in &policy.detection_rules {
            match rule.rule_type {
                RuleType::ObjectDetection => {
                    if let Some(objects) = rule.parameters.get("objects") {
                        if let Ok(objs) = serde_json::from_value::<Vec<String>>(objects.clone()) {
                            target_objects.extend(objs);
                        }
                    }
                    min_confidence = min_confidence.min(rule.confidence_threshold);
                }
                RuleType::SceneClassification => {
                    if let Some(scenes) = rule.parameters.get("scenes") {
                        if let Ok(scns) = serde_json::from_value::<Vec<String>>(scenes.clone()) {
                            target_scenes.extend(scns);
                        }
                    }
                    min_confidence = min_confidence.min(rule.confidence_threshold);
                }
                _ => {}
            }
        }
        
        Self {
            policy_id: policy.id.clone(),
            category: policy.category,
            target_objects,
            target_scenes,
            min_confidence,
        }
    }
    
    pub fn id(&self) -> &str {
        &self.policy_id
    }
    
    pub fn category(&self) -> PolicyCategory {
        self.category
    }
    
    pub fn matches_object(&self, label: &str) -> bool {
        self.target_objects.iter().any(|obj| {
            label.to_lowercase().contains(&obj.to_lowercase())
        })
    }
    
    pub fn matches_scene(&self, label: &str) -> bool {
        self.target_scenes.iter().any(|scene| {
            label.to_lowercase().contains(&scene.to_lowercase())
        })
    }
    
    pub fn min_confidence(&self) -> f32 {
        self.min_confidence
    }
    
    pub fn severity_for_confidence(&self, confidence: f32) -> Severity {
        if confidence >= 0.9 {
            Severity::Critical
        } else if confidence >= 0.75 {
            Severity::High
        } else if confidence >= 0.6 {
            Severity::Medium
        } else {
            Severity::Low
        }
    }
    
    pub fn suggested_fixes(&self) -> Vec<FixSuggestion> {
        match self.category {
            PolicyCategory::Copyright => vec![
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
                    fix_type: FixType::Remove,
                    impact: ImpactLevel::High,
                    description: "Remove the segment entirely".to_string(),
                    estimated_quality_loss: 0.5,
                    parameters: HashMap::new(),
                },
            ],
            PolicyCategory::Violence => vec![
                FixSuggestion {
                    id: Uuid::new_v4(),
                    fix_type: FixType::Blur,
                    impact: ImpactLevel::Medium,
                    description: "Blur violent content".to_string(),
                    estimated_quality_loss: 0.2,
                    parameters: HashMap::new(),
                },
                FixSuggestion {
                    id: Uuid::new_v4(),
                    fix_type: FixType::AddContext,
                    impact: ImpactLevel::Low,
                    description: "Add educational context card".to_string(),
                    estimated_quality_loss: 0.05,
                    parameters: HashMap::new(),
                },
            ],
            _ => vec![
                FixSuggestion {
                    id: Uuid::new_v4(),
                    fix_type: FixType::Remove,
                    impact: ImpactLevel::High,
                    description: "Remove the segment".to_string(),
                    estimated_quality_loss: 0.5,
                    parameters: HashMap::new(),
                },
            ],
        }
    }
}

/// Text policy wrapper
pub struct TextPolicy {
    policy_id: String,
    category: PolicyCategory,
    min_confidence: f32,
}

impl TextPolicy {
    fn from_policy(policy: &Policy) -> Self {
        let min_confidence = policy.detection_rules
            .iter()
            .filter(|r| matches!(r.rule_type, RuleType::TextClassification | RuleType::SentimentAnalysis))
            .map(|r| r.confidence_threshold)
            .fold(1.0f32, |a, b| a.min(b));
        
        Self {
            policy_id: policy.id.clone(),
            category: policy.category,
            min_confidence: min_confidence.max(0.7),
        }
    }
    
    pub fn id(&self) -> &str {
        &self.policy_id
    }
    
    pub fn category(&self) -> PolicyCategory {
        self.category
    }
    
    pub async fn check_text(&self, text: &str) -> Result<Option<TextViolation>> {
        // This would use an actual text classification model
        // For now, return None (no violation detected)
        
        // Mock implementation - check for some trigger words
        let trigger_words = vec!["hate", "kill", "attack", "violence"];
        
        for word in trigger_words {
            if text.to_lowercase().contains(word) {
                return Ok(Some(TextViolation {
                    severity: Severity::Medium,
                    confidence: 0.75,
                    description: format!("Potentially problematic language detected: {}", word),
                }));
            }
        }
        
        Ok(None)
    }
    
    pub fn suggested_fixes(&self) -> Vec<FixSuggestion> {
        vec![
            FixSuggestion {
                id: Uuid::new_v4(),
                fix_type: FixType::Mute,
                impact: ImpactLevel::Medium,
                description: "Mute audio containing problematic language".to_string(),
                estimated_quality_loss: 0.3,
                parameters: HashMap::new(),
            },
        ]
    }
}

/// Policy update manifest from remote
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyUpdateManifest {
    pub version: String,
    pub published_at: DateTime<Utc>,
    pub policies: Vec<Policy>,
    pub changelog: String,
}
