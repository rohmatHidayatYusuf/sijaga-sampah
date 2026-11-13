use std::env;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expiration_hours: i64,
    pub upload_dir: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()
            .unwrap_or(8080);
        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@localhost:5432/sijaga_db".to_string()
        });
        
        // SEC-01: Enforce strong JWT secret key (minimum 32 characters)
        let jwt_secret = match env::var("JWT_SECRET") {
            Ok(secret) if secret.len() >= 32 => secret,
            Ok(short_secret) => {
                tracing::warn!("JWT_SECRET terlalu pendek (kurang dari 32 karakter). Menghasilkan secret acak yang kuat.");
                format!("{}_{}", short_secret, uuid::Uuid::new_v4().simple())
            }
            Err(_) => {
                let generated = format!("sijaga_{}_{}", uuid::Uuid::new_v4().simple(), uuid::Uuid::new_v4().simple());
                tracing::warn!("JWT_SECRET tidak ditemukan di environment. Menghasilkan secret kriptografis aman otomatis untuk sesi ini.");
                generated
            }
        };

        let jwt_expiration_hours = env::var("JWT_EXPIRATION_HOURS")
            .unwrap_or_else(|_| "24".to_string())
            .parse::<i64>()
            .unwrap_or(24);
        let upload_dir = env::var("UPLOAD_DIR").unwrap_or_else(|_| "./uploads".to_string());

        Self {
            host,
            port,
            database_url,
            jwt_secret,
            jwt_expiration_hours,
            upload_dir,
        }
    }
}
