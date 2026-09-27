//! REST API server for ContentGuard AI

use std::collections::HashMap;
use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use anyhow::Result;
use tokio::sync::{mpsc, RwLock};
use tracing::{info, warn};
use uuid::Uuid;

use crate::analysis::ContentAnalyzer;
use crate::config::AppConfig;
use crate::policy::PolicyManager;
use crate::types::*;
use crate::types::api::*;

/// API server state
pub struct ApiServerState {
    config: Arc<AppConfig>,
    analyzer: Arc<RwLock<ContentAnalyzer>>,
    policy_manager: Arc<RwLock<PolicyManager>>,
    active_jobs: Arc<RwLock<HashMap<Uuid, AnalysisJob>>>,
    progress_tx: mpsc::Sender<AnalysisProgress>,
}

/// Analysis job tracking
#[derive(Debug, Clone)]
pub struct AnalysisJob {
    pub id: Uuid,
    pub status: JobStatus,
    pub report: Option<AnalysisReport>,
    pub progress: Option<AnalysisProgress>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

impl ApiServer {
    /// Create a new API server
    pub async fn new(
        config: &AppConfig,
        enable_gpu: bool,
        _workers: usize,
    ) -> Result<Self> {
        let config = Arc::new(config.clone());
        
        // Initialize analyzer
        let analyzer = ContentAnalyzer::new(&config, enable_gpu).await?;
        
        // Initialize policy manager
        let policy_manager = PolicyManager::new(&config).await?;
        
        // Create progress channel
        let (progress_tx, mut progress_rx) = mpsc::channel(100);
        
        // Spawn progress handler
        let active_jobs: Arc<RwLock<HashMap<Uuid, AnalysisJob>>> = Arc::new(RwLock::new(HashMap::new()));
        let jobs_clone = active_jobs.clone();
        
        tokio::spawn(async move {
            while let Some(progress) = progress_rx.recv().await {
                let mut jobs = jobs_clone.write().await;
                if let Some(job) = jobs.get_mut(&progress.job_id) {
                    job.progress = Some(progress.clone());
                    
                    if progress.stage == AnalysisStage::Complete {
                        job.status = JobStatus::Completed;
                    }
                }
            }
        });
        
        let state = Arc::new(ApiServerState {
            config: config.clone(),
            analyzer: Arc::new(RwLock::new(analyzer)),
            policy_manager: Arc::new(RwLock::new(policy_manager)),
            active_jobs,
            progress_tx,
        });
        
        Ok(Self { state })
    }
    
    /// Run the API server
    pub async fn run(&self, bind: &str) -> Result<()> {
        info!("Starting API server on {}", bind);
        
        let state = self.state.clone();
        
        HttpServer::new(move || {
            let cors = Cors::default()
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header()
                .max_age(3600);
            
            App::new()
                .wrap(cors)
                .app_data(web::Data::new(state.clone()))
                .route("/health", web::get().to(health_handler))
                .route("/api/v1/analyze", web::post().to(analyze_handler))
                .route("/api/v1/analyze/{job_id}", web::get().to(get_analysis_status))
                .route("/api/v1/analyze/{job_id}/report", web::get().to(get_report_handler))
                .route("/api/v1/fix", web::post().to(fix_handler))
                .route("/api/v1/policies", web::get().to(get_policies_handler))
                .route("/api/v1/policies/update", web::post().to(update_policies_handler))
                .route("/api/v1/preview", web::post().to(preview_handler))
                .route("/metrics", web::get().to(metrics_handler))
        })
        .bind(bind)?
        .run()
        .await?;
        
        Ok(())
    }
}

/// Health check handler
async fn health_handler(data: web::Data<Arc<ApiServerState>>) -> HttpResponse {
    let gpu_available = data.config.gpu.enabled;
    
    let response = HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        gpu_available,
        models_loaded: vec![
            "object_detection".to_string(),
            "scene_classification".to_string(),
        ],
    };
    
    HttpResponse::Ok().json(response)
}

/// Start analysis handler
async fn analyze_handler(
    data: web::Data<Arc<ApiServerState>>,
    req: web::Json<AnalyzeRequest>,
) -> HttpResponse {
    let job_id = Uuid::new_v4();
    let depth = req.depth.unwrap_or(AnalysisDepth::Standard);
    
    // Create job entry
    let job = AnalysisJob {
        id: job_id,
        status: JobStatus::Pending,
        report: None,
        progress: None,
        created_at: chrono::Utc::now(),
    };
    
    {
        let mut jobs = data.active_jobs.write().await;
        jobs.insert(job_id, job);
    }
    
    // Start analysis in background
    let analyzer = data.analyzer.clone();
    let video_path = req.video_path.clone();
    let job_id_clone = job_id;
    let active_jobs = data.active_jobs.clone();
    
    tokio::spawn(async move {
        let analyzer = analyzer.read().await;
        
        // Update status to running
        {
            let mut jobs = active_jobs.write().await;
            if let Some(job) = jobs.get_mut(&job_id_clone) {
                job.status = JobStatus::Running;
            }
        }
        
        match analyzer.analyze_video(&video_path, depth).await {
            Ok(report) => {
                let mut jobs = active_jobs.write().await;
                if let Some(job) = jobs.get_mut(&job_id_clone) {
                    job.status = JobStatus::Completed;
                    job.report = Some(report);
                }
            }
            Err(e) => {
                warn!("Analysis failed for job {}: {}", job_id_clone, e);
                let mut jobs = active_jobs.write().await;
                if let Some(job) = jobs.get_mut(&job_id_clone) {
                    job.status = JobStatus::Failed;
                }
            }
        }
    });
    
    let response = AnalyzeResponse {
        job_id,
        status: "accepted".to_string(),
        estimated_seconds: match depth {
            AnalysisDepth::Fast => 30,
            AnalysisDepth::Standard => 120,
            AnalysisDepth::Deep => 300,
        },
    };
    
    HttpResponse::Accepted().json(response)
}

/// Get analysis status handler
async fn get_analysis_status(
    data: web::Data<Arc<ApiServerState>>,
    path: web::Path<String>,
) -> HttpResponse {
    let job_id = match Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Invalid job ID"
        })),
    };
    
    let jobs = data.active_jobs.read().await;
    
    match jobs.get(&job_id) {
        Some(job) => {
            let status = match job.status {
                JobStatus::Pending => "pending",
                JobStatus::Running => "running",
                JobStatus::Completed => "completed",
                JobStatus::Failed => "failed",
            };
            
            let response = ReportResponse {
                job_id,
                report: None,
                status: status.to_string(),
                progress: job.progress.clone(),
            };
            
            HttpResponse::Ok().json(response)
        }
        None => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Job not found"
        })),
    }
}

/// Get report handler
async fn get_report_handler(
    data: web::Data<Arc<ApiServerState>>,
    path: web::Path<String>,
) -> HttpResponse {
    let job_id = match Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Invalid job ID"
        })),
    };
    
    let jobs = data.active_jobs.read().await;
    
    match jobs.get(&job_id) {
        Some(job) => {
            if job.status == JobStatus::Completed {
                if let Some(ref report) = job.report {
                    HttpResponse::Ok().json(report)
                } else {
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "Report not available"
                    }))
                }
            } else {
                HttpResponse::Accepted().json(serde_json::json!({
                    "status": "processing",
                    "progress": job.progress
                }))
            }
        }
        None => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Job not found"
        })),
    }
}

/// Fix handler
async fn fix_handler(
    data: web::Data<Arc<ApiServerState>>,
    req: web::Json<FixRequest>,
) -> HttpResponse {
    use crate::fix::FixEngine;
    
    // Get the analysis report
    let jobs = data.active_jobs.read().await;
    let report = match jobs.get(&req.report_id) {
        Some(job) => {
            if let Some(ref report) = job.report {
                report.clone()
            } else {
                return HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "Analysis report not available"
                }));
            }
        }
        None => return HttpResponse::NotFound().json(serde_json::json!({
            "error": "Analysis job not found"
        })),
    };
    drop(jobs);
    
    // Apply fixes
    let fix_engine = match FixEngine::new(&data.config) {
        Ok(engine) => engine,
        Err(e) => return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Failed to initialize fix engine: {}", e)
        })),
    };
    
    let intensity = req.intensity.unwrap_or(70);
    
    match fix_engine.apply_fixes(&req.video_path, &report, &req.output_path, intensity).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Fix application failed: {}", e)
        })),
    }
}

/// Get policies handler
async fn get_policies_handler(data: web::Data<Arc<ApiServerState>>) -> HttpResponse {
    let policy_manager = data.policy_manager.read().await;
    let policies = policy_manager.get_all_policies();
    
    HttpResponse::Ok().json(policies)
}

/// Update policies handler
async fn update_policies_handler(
    data: web::Data<Arc<ApiServerState>>,
) -> HttpResponse {
    let mut policy_manager = data.policy_manager.write().await;
    
    match policy_manager.update_policies(true).await {
        Ok(updated) => {
            if updated {
                HttpResponse::Ok().json(serde_json::json!({
                    "status": "updated",
                    "message": "Policies updated successfully"
                }))
            } else {
                HttpResponse::Ok().json(serde_json::json!({
                    "status": "current",
                    "message": "Policies are already up to date"
                }))
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Policy update failed: {}", e)
        })),
    }
}

/// Preview handler
async fn preview_handler(
    data: web::Data<Arc<ApiServerState>>,
    req: web::Json<serde_json::Value>,
) -> HttpResponse {
    use crate::fix::FixEngine;
    
    let video_path = req.get("video_path").and_then(|v| v.as_str()).unwrap_or("");
    let issue_id = req.get("issue_id").and_then(|v| v.as_str()).unwrap_or("");
    let preview_path = req.get("preview_path").and_then(|v| v.as_str()).unwrap_or("/tmp/preview.mp4");
    
    // Get the issue from active jobs
    let jobs = data.active_jobs.read().await;
    let issue = jobs.values()
        .filter_map(|j| j.report.as_ref())
        .flat_map(|r| r.issues.iter())
        .find(|i| i.id.to_string() == issue_id)
        .cloned();
    drop(jobs);
    
    let issue = match issue {
        Some(i) => i,
        None => return HttpResponse::NotFound().json(serde_json::json!({
            "error": "Issue not found"
        })),
    };
    
    let fix_engine = match FixEngine::new(&data.config) {
        Ok(engine) => engine,
        Err(e) => return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Failed to initialize fix engine: {}", e)
        })),
    };
    
    let fix_type = issue.suggested_fixes.first()
        .map(|f| f.fix_type)
        .unwrap_or(FixType::Blur);
    
    match fix_engine.preview_fix(video_path, &issue, fix_type, preview_path).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "preview_path": preview_path
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Preview generation failed: {}", e)
        })),
    }
}

/// Metrics handler
async fn metrics_handler() -> HttpResponse {
    // Return Prometheus-compatible metrics
    let metrics = r#"
# HELP contentguard_analysis_total Total number of analyses performed
# TYPE contentguard_analysis_total counter
contentguard_analysis_total 0

# HELP contentguard_fixes_total Total number of fixes applied
# TYPE contentguard_fixes_total counter
contentguard_fixes_total 0

# HELP contentguard_active_jobs Number of active analysis jobs
# TYPE contentguard_active_jobs gauge
contentguard_active_jobs 0
"#;
    
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(metrics)
}

/// API Server struct
pub struct ApiServer {
    state: Arc<ApiServerState>,
}
