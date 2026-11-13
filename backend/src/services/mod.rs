use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use crate::errors::AppError;

pub struct AuthService;

impl AuthService {
    pub fn hash_password(password: &str) -> Result<String, AppError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| AppError::Internal(format!("Gagal hashing password: {}", e)))?
            .to_string();
        Ok(password_hash)
    }

    pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, AppError> {
        let parsed_hash = PasswordHash::new(password_hash)
            .map_err(|e| AppError::Internal(format!("Hash password tidak valid: {}", e)))?;
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}

pub struct ReportService;

impl ReportService {
    /// Validasi siklus transisi status laporan berdasarkan PRD Bagian 6.1
    pub fn validate_status_transition(current_status: &str, next_status: &str) -> Result<(), AppError> {
        let is_valid = match (current_status, next_status) {
            // Jalur Utama
            ("submitted", "under_review") => true,
            ("submitted", "verified") => true,
            ("submitted", "rejected") => true,
            ("under_review", "verified") => true,
            ("under_review", "rejected") => true,
            ("verified", "assigned") => true,
            ("assigned", "in_progress") => true,
            ("in_progress", "resolved") => true,
            ("resolved", "closed") => true,
            
            // Reopened & Alternatif
            ("resolved", "reopened") => true,
            ("closed", "reopened") => true,
            ("reopened", "assigned") => true,
            ("reopened", "in_progress") => true,
            _ => false,
        };

        if is_valid {
            Ok(())
        } else {
            Err(AppError::BadRequest(format!(
                "Transisi status tidak diizinkan: dari '{}' ke '{}'.",
                current_status, next_status
            )))
        }
    }
}
