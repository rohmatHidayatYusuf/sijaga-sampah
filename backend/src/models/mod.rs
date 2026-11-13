use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub full_name: String,
    pub email: String,
    pub phone: Option<String>,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Role {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Region {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub code: Option<String>,
    pub name: String,
    pub region_type: String,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WasteCategory {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Report {
    pub id: Uuid,
    pub ticket_number: String,
    pub reporter_id: Uuid,
    pub category_id: Uuid,
    pub region_id: Option<Uuid>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ReportAttachment {
    pub id: Uuid,
    pub report_id: Uuid,
    pub uploaded_by: Uuid,
    pub object_key: String,
    pub original_filename: Option<String>,
    pub content_type: String,
    pub file_size_bytes: i64,
    pub attachment_type: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ReportStatusHistory {
    pub id: Uuid,
    pub report_id: Uuid,
    pub changed_by: Option<Uuid>,
    pub from_status: Option<String>,
    pub to_status: String,
    pub note: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Assignment {
    pub id: Uuid,
    pub report_id: Uuid,
    pub assignee_id: Uuid,
    pub assigned_by: Uuid,
    pub instructions: Option<String>,
    pub assigned_at: DateTime<Utc>,
    pub accepted_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CollectionSchedule {
    pub id: Uuid,
    pub region_id: Uuid,
    pub responsible_user_id: Option<Uuid>,
    pub title: String,
    pub scheduled_at: DateTime<Utc>,
    pub status: String,
    pub notes: Option<String>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Notification {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub body: String,
    pub target_type: Option<String>,
    pub target_id: Option<Uuid>,
    pub read_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AuditLog {
    pub id: Uuid,
    pub actor_id: Option<Uuid>,
    pub action: String,
    pub entity_type: String,
    pub entity_id: Option<Uuid>,
    pub metadata: serde_json::Value,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
}
