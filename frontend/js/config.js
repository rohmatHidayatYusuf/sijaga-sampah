// ========================================================================
// SIJAGA SAMPAH - Konfigurasi API
// ========================================================================
// Ganti URL di bawah dengan URL backend Railway Anda setelah deploy
// Contoh: "https://sijaga-backend-production.up.railway.app/api/v1"
// ========================================================================

const API_BASE_URL = "http://localhost:8080/api/v1"; // ← Ganti ini setelah backend Railway aktif

const APP_CONFIG = {
  apiBaseUrl: API_BASE_URL,
  appName: "SIJAGA SAMPAH",
  version: "1.0.0",
  isDevelopment: API_BASE_URL.includes("localhost"),
};

console.log(
  `[SIJAGA] Mode: ${APP_CONFIG.isDevelopment ? "Development" : "Production"}`
);
console.log(`[SIJAGA] API: ${APP_CONFIG.apiBaseUrl}`);

