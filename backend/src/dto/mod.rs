use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Standard API response envelope
#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub data: T,
}

#[derive(Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
}

// Auth DTOs
#[derive(Deserialize)]
pub struct RegisterRequest {
    pub full_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserProfileResponse,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserProfileResponse {
    pub id: Uuid,
    pub full_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub roles: Vec<String>,
}

#[derive(Deserialize)]
pub struct UpdateProfileRequest {
    pub full_name: Option<String>,
    pub phone: Option<String>,
}

// Report DTOs
#[derive(Deserialize)]
pub struct CreateReportRequest {
    pub category_id: Uuid,
    pub title: String,
    pub description: String,
    pub region_id: Option<Uuid>,
    pub address_text: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct ReportCreatedResponse {
    pub id: Uuid,
    pub ticket_number: String,
    pub status: String,
    pub submitted_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ReportDetailResponse {
    pub id: Uuid,
    pub ticket_number: String,
    pub reporter_id: Uuid,
    pub reporter_name: Option<String>,
    pub category_id: Uuid,
    pub category_name: Option<String>,
    pub region_id: Option<Uuid>,
    pub region_name: Option<String>,
    pub title: String,
    pub description: String,
    pub address_text: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub status: String,
    pub priority: String,
    pub rejection_reason: Option<String>,
    pub submitted_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub attachments: Vec<AttachmentResponse>,
    pub history: Vec<StatusHistoryResponse>,
    pub assignment: Option<AssignmentResponse>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AttachmentResponse {
    pub id: Uuid,
    pub object_key: String,
    pub original_filename: Option<String>,
    pub content_type: String,
    pub file_size_bytes: i64,
    pub attachment_type: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct StatusHistoryResponse {
    pub id: Uuid,
    pub changed_by: Option<Uuid>,
    pub changed_by_name: Option<String>,
    pub from_status: Option<String>,
    pub to_status: String,
    pub note: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AssignmentResponse {
    pub id: Uuid,
    pub report_id: Uuid,
    pub assignee_id: Uuid,
    pub assignee_name: Option<String>,
    pub assigned_by: Uuid,
    pub assigned_by_name: Option<String>,
    pub instructions: Option<String>,
    pub assigned_at: DateTime<Utc>,
    pub accepted_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub is_active: bool,
}

#[derive(Deserialize)]
pub struct VerifyReportRequest {
    pub priority: Option<String>,
    pub region_id: Option<Uuid>,
    pub note: Option<String>,
}

#[derive(Deserialize)]
pub struct RejectReportRequest {
    pub reason: String,
}

#[derive(Deserialize)]
pub struct AssignReportRequest {
    pub assignee_id: Uuid,
    pub instructions: Option<String>,
    pub priority: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateReportStatusRequest {
    pub status: String,
    pub note: Option<String>,
}

// Tasks DTOs
#[derive(Deserialize)]
pub struct CompleteTaskRequest {
    pub notes: Option<String>,
    pub evidence_file_key: Option<String>,
}

// Dashboard DTOs
#[derive(Serialize, Deserialize)]
pub struct DashboardSummary {
    pub total_reports: i64,
    pub submitted_reports: i64,
    pub in_progress_reports: i64,
    pub resolved_reports: i64,
    pub rejected_reports: i64,
    pub resolution_rate_percentage: f64,
    pub active_officers: i64,
}

#[derive(Serialize, Deserialize)]
pub struct TrendPoint {
    pub date: String,
    pub count: i64,
}

#[derive(Serialize, Deserialize)]
pub struct MapPoint {
    pub id: Uuid,
    pub ticket_number: String,
    pub title: String,
    pub category_name: String,
    pub status: String,
    pub priority: String,
    pub latitude: f64,
    pub longitude: f64,
}

// Schedules DTOs
#[derive(Deserialize)]
pub struct CreateScheduleRequest {
    pub region_id: Uuid,
    pub responsible_user_id: Option<Uuid>,
    pub title: String,
    pub scheduled_at: DateTime<Utc>,
    pub notes: Option<String>,
}
