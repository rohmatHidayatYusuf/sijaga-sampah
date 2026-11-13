use crate::{
    dto::*,
    errors::AppError,
    models::{Region, User, WasteCategory},
};
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

pub struct AuthRepository;

impl AuthRepository {
    pub async fn find_user_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT id, full_name, email, phone, password_hash, is_active, created_at, updated_at
            FROM users
            WHERE email = $1
            "#,
        )
        .bind(email)
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    pub async fn find_user_roles(pool: &PgPool, user_id: Uuid) -> Result<Vec<String>, AppError> {
        let roles = sqlx::query_scalar::<_, String>(
            r#"
            SELECT r.code
            FROM roles r
            JOIN user_roles ur ON ur.role_id = r.id
            WHERE ur.user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;

        Ok(roles)
    }

    pub async fn create_user_with_role(
        pool: &PgPool,
        full_name: &str,
        email: &str,
        phone: Option<&str>,
        password_hash: &str,
        role_code: &str,
    ) -> Result<User, AppError> {
        let mut tx = pool.begin().await?;

        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (full_name, email, phone, password_hash)
            VALUES ($1, $2, $3, $4)
            RETURNING id, full_name, email, phone, password_hash, is_active, created_at, updated_at
            "#,
        )
        .bind(full_name)
        .bind(email)
        .bind(phone)
        .bind(password_hash)
        .fetch_one(&mut *tx)
        .await?;

        // Assign role
        sqlx::query(
            r#"
            INSERT INTO user_roles (user_id, role_id)
            SELECT $1, id FROM roles WHERE code = $2
            "#,
        )
        .bind(user.id)
        .bind(role_code)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(user)
    }
}

pub struct ReportRepository;

impl ReportRepository {
    pub async fn generate_ticket_number(tx: &mut Transaction<'_, Postgres>) -> Result<String, AppError> {
        let year = chrono::Utc::now().format("%Y").to_string();
        let count: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*) FROM reports WHERE ticket_number LIKE $1
            "#,
        )
        .bind(format!("SJG-{}-%", year))
        .fetch_one(&mut **tx)
        .await?;

        Ok(format!("SJG-{}-{:06}", year, count + 1))
    }

    pub async fn create_report(
        pool: &PgPool,
        reporter_id: Uuid,
        req: &CreateReportRequest,
    ) -> Result<ReportCreatedResponse, AppError> {
        let mut tx = pool.begin().await?;

        let ticket_number = Self::generate_ticket_number(&mut tx).await?;

        let row = sqlx::query!(
            r#"
            INSERT INTO reports (
                ticket_number, reporter_id, category_id, region_id,
                title, description, address_text, latitude, longitude,
                status, priority
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, 'submitted', 'normal')
            RETURNING id, ticket_number, status, submitted_at
            "#,
            ticket_number,
            reporter_id,
            req.category_id,
            req.region_id,
            req.title,
            req.description,
            req.address_text,
            req.latitude.map(rust_decimal::Decimal::from_f64_retain).flatten(),
            req.longitude.map(rust_decimal::Decimal::from_f64_retain).flatten(),
        )
        .fetch_one(&mut *tx)
        .await?;

        // Record initial status history
        sqlx::query(
            r#"
            INSERT INTO report_status_history (report_id, changed_by, to_status, note)
            VALUES ($1, $2, 'submitted', 'Laporan pertama kali dikirim oleh warga.')
            "#,
        )
        .bind(row.id)
        .bind(reporter_id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(ReportCreatedResponse {
            id: row.id,
            ticket_number: row.ticket_number,
            status: row.status,
            submitted_at: row.submitted_at,
        })
    }

    pub async fn record_status_history(
        tx: &mut Transaction<'_, Postgres>,
        report_id: Uuid,
        actor_id: Option<Uuid>,
        from_status: Option<&str>,
        to_status: &str,
        note: Option<&str>,
    ) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO report_status_history (report_id, changed_by, from_status, to_status, note)
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(report_id)
        .bind(actor_id)
        .bind(from_status)
        .bind(to_status)
        .bind(note)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
}

pub struct MasterRepository;

impl MasterRepository {
    pub async fn list_categories(pool: &PgPool) -> Result<Vec<WasteCategory>, AppError> {
        let categories = sqlx::query_as::<_, WasteCategory>(
            r#"
            SELECT id, code, name, description, is_active
            FROM waste_categories
            WHERE is_active = TRUE
            ORDER BY name ASC
            "#,
        )
        .fetch_all(pool)
        .await?;

        Ok(categories)
    }

    pub async fn list_regions(pool: &PgPool) -> Result<Vec<Region>, AppError> {
        let regions = sqlx::query_as::<_, Region>(
            r#"
            SELECT id, parent_id, code, name, region_type, is_active
            FROM regions
            WHERE is_active = TRUE
            ORDER BY name ASC
            "#,
        )
        .fetch_all(pool)
        .await?;

        Ok(regions)
    }
}
