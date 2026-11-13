# SIJAGA SAMPAH
### Sistem Informasi Jaga Kebersihan dan Pengelolaan Sampah Kota

[![Smart City](https://img.shields.io/badge/Domain-Smart%20Environment-emerald.svg)](#)
[![Backend](https://img.shields.io/badge/Backend-Rust%20(Axum%20%2B%20SQLx)-orange.svg)](#)
[![Database](https://img.shields.io/badge/Database-PostgreSQL%2016-blue.svg)](#)
[![Frontend](https://img.shields.io/badge/Frontend-Responsive%20Web%20%2F%20PWA-teal.svg)](#)

Aplikasi pengelolaan sampah berbasis web responsif yang menghubungkan masyarakat, petugas kebersihan lapangan, operator dinas lingkungan hidup, pengelola bank sampah, dan pemerintah daerah dalam satu sistem terpadu.

Aplikasi ini diimplementasikan secara komprehensif berdasarkan **Product Requirement Document (PRD) v1.0**.

---

## 🏛️ Identitas & Kerjasama Institusi

- **Pemerintah Daerah Mitra:** Dinas Lingkungan Hidup / Pemerintah Kota
- **Perguruan Tinggi:** Universitas Teknokrat Indonesia (*Sang Juara*)
- **Footer Resmi:** `Copyright©2026 by Yusuf dan Dr. Erliyan Redy Susanto, M.Kom`

---

## 🌟 Fitur Utama (P0 MVP)

1. **Autentikasi & Otorisasi RBAC (F-01):**
   - 5 Peran: **Warga**, **Petugas Kebersihan**, **Operator**, **Pimpinan/Viewer**, dan **Admin Sistem**.
   - Hashing password dengan **Argon2id** dan otentikasi stateless **JWT**.
2. **Pelaporan Masalah Persampahan (F-02):**
   - Kategori sampah (Sampah Liar, TPS Meluap, Saluran Tersumbat, Bangkai Binatang, Sampah B3).
   - Penentuan lokasi interaktif menggunakan peta OpenStreetMap / GPS Geolocation dan alamat manual.
   - Lampiran foto kondisi awal sampah.
3. **Tiket & Pelacakan Real-time (F-03):**
   - Nomor tiket unik otomatis dengan format: `SJG-YYYY-XXXXXX`.
   - Linimasa audit (*status history*) yang mencatat aktor, waktu, dan catatan perubahan status.
4. **Verifikasi & Manajemen Laporan (F-04):**
   - Verifikasi oleh Operator dengan penentuan skala prioritas (*Low*, *Normal*, *High*, *Urgent*).
   - Penolakan laporan dengan alasan wajib transparan bagi pelapor.
5. **Penugasan Petugas Lapangan (F-05):**
   - Operator menugaskan laporan kepada petugas kebersihan disertai instruksi penanganan.
6. **Eksekusi Tugas & Unggah Bukti (F-06):**
   - Petugas menerima tugas (`in_progress`) dan mengunggah foto serta catatan bukti penyelesaian (`resolved`).
7. **Dashboard Eksekutif & Peta Kota (F-07 & F-11):**
   - KPI total laporan, rasio penyelesaian (*resolution rate %*), antrean verifikasi.
   - Peta interaktif Leaflet dengan pin berwarna sesuai status laporan.
8. **Jadwal Pengangkutan & Edukasi (F-10 & F-14):**
   - Manajemen jadwal rotasi armada truk per kecamatan.
   - Panduan pemilahan sampah organik, anorganik, dan B3.
9. **Ekspor Data CSV (F-12):**
   - Unduh rekapitulasi data laporan ke format spreadsheet CSV.

---

## 📁 Struktur Direktori

```text
sijaga/
├── backend/
│   ├── Cargo.toml               # Dependensi Rust (Axum, SQLx, Tokio, Serde, Argon2)
│   ├── Dockerfile               # Multi-stage Docker build untuk backend Rust
│   ├── migrations/              # Skrip migrasi DDL database PostgreSQL
│   │   ├── 0001_initial_schema.sql
│   │   └── 0002_seed_data.sql
│   └── src/
│       ├── main.rs              # Entry point Axum & Graceful shutdown
│       ├── app_state.rs         # Database pool & JWT helper
│       ├── config/              # Konfigurasi environment
│       ├── dto/                 # Kontrak request/response JSON
│       ├── errors/              # Penanganan error standar PRD
│       ├── middleware/          # JWT auth & Role verification
│       ├── models/              # Model data relasional
│       ├── repositories/        # Query async PostgreSQL SQLx
│       ├── services/            # Aturan bisnis & validasi status
│       └── routes/              # Modul router REST API v1
├── frontend/
│   ├── index.html               # Halaman web responsif & PWA
│   ├── manifest.json            # Manifest Progressive Web App
│   ├── assets/                  # Logo resmi Pemda & Teknokrat (SVG)
│   ├── css/                     # Styling custom, badges, timeline
│   └── js/
│       ├── app.js               # Kontroller UI, Leaflet map, tabs, modals
│       ├── mock-data.js         # Seed data reaktif untuk pengujian cepat
│       └── api-client.js        # Abstraksi koneksi API & local persistence
├── docker-compose.yml           # Orkestrasi PostgreSQL 16 + Rust API + Frontend Nginx
├── openapi.yaml                 # Spesifikasi kontrak OpenAPI 3.0 REST API
├── start-frontend.ps1           # Script PowerShell 1-klik untuk menjalankan server web
└── README.md
```

---

## 🚀 Panduan Menjalankan Aplikasi

### Cara 1: Uji Coba Cepat Frontend (1-Klik via PowerShell)

Aplikasi web telah dilengkapi dengan simulator peran (*role switcher*) dan engine data interaktif. Anda dapat langsung menjalankannya tanpa perlu menginstal dependensi tambahan:

1. Buka PowerShell di folder `sijaga`:
   ```powershell
   .\start-frontend.ps1
   ```
2. Aplikasi akan terbuka otomatis di browser pada alamat:
   ```text
   http://localhost:3000
   ```
3. Anda dapat langsung menguji alur 5 peran secara bergantian melalui dropdown **Peran Pengguna** di sudut kanan atas header.

---

### Cara 2: Menjalankan Fullstack Lengkap dengan Docker Compose

Untuk menjalankan backend Rust, database PostgreSQL 16, dan frontend Nginx secara bersamaan dalam container Docker:

1. Pastikan Docker Desktop aktif di komputer Anda.
2. Jalankan perintah:
   ```bash
   docker compose up --build
   ```
3. Akses layanan:
   - **Frontend Web:** `http://localhost:3000`
   - **Backend API Rust:** `http://localhost:8080/api/v1`
   - **Health Check API:** `http://localhost:8080/health/ready`
   - **PostgreSQL Database:** `localhost:5432` (`sijaga_db`)

---

### Cara 3: Menjalankan Backend Rust Manual

1. Pastikan Rust toolchain (`cargo`) dan PostgreSQL telah terpasang.
2. Buat database `sijaga_db` di PostgreSQL dan jalankan migrasi:
   ```bash
   psql -U postgres -d sijaga_db -f backend/migrations/0001_initial_schema.sql
   psql -U postgres -d sijaga_db -f backend/migrations/0002_seed_data.sql
   ```
3. Atur variabel lingkungan di `backend/.env`:
   ```env
   HOST=0.0.0.0
   PORT=8080
   DATABASE_URL=postgres://postgres:password123@localhost:5432/sijaga_db
   JWT_SECRET=sijaga_sampah_super_secret_jwt_key_2026_change_me
   JWT_EXPIRATION_HOURS=24
   ```
4. Jalankan backend:
   ```bash
   cd backend
   cargo run
   ```

---

## 👥 Akun & Peran Bawaan (*Default Credentials*)

Semua akun bawaan di dalam database dienkripsi menggunakan Argon2id dengan password standar: `Password123!`

| Peran | Nama Pengguna | Email | Hak Akses |
|---|---|---|---|
| **Warga** | Ahmad Yusuf | `warga@sijaga.id` | Membuat laporan, memantau nomor tiket, melihat status miliknya |
| **Petugas Lapangan** | Budi Santoso | `petugas1@sijaga.id` | Melihat tugas aktif, menerima tugas, mengunggah bukti penyelesaian |
| **Operator Dinas** | Siti Rahmawati, S.AP | `operator@sijaga.id` | Memverifikasi laporan, menolak dengan alasan, menugaskan petugas |
| **Pimpinan / Viewer** | Drs. Hendrawan, M.Si | `pimpinan@sijaga.id` | Melihat dashboard eksekutif, KPI kota, sebaran peta, ekspor CSV |
| **Admin Sistem** | Administrator SIJAGA | `admin@sijaga.id` | Mengelola data master kategori, wilayah, akun, dan audit teknis |

---

## 🔄 Siklus Status Laporan (Sesuai PRD Bagian 6.1)

```text
submitted ──► under_review ──► verified ──► assigned ──► in_progress ──► resolved ──► closed
   │                                                                        ▲
   └──► rejected (wajib alasan)                                              │
                                                                       reopened
```

---

## 📑 Dokumentasi API

Kontrak API lengkap didokumentasikan dalam format standar OpenAPI 3.0 pada berkas [openapi.yaml](file:///c:/Users/yusuf/Downloads/sijaga/openapi.yaml).
Endpoint utama memiliki prefix `/api/v1/`:
- Autentikasi: `POST /api/v1/auth/register`, `POST /api/v1/auth/login`, `GET /api/v1/auth/me`
- Laporan: `POST /api/v1/reports`, `GET /api/v1/reports`, `GET /api/v1/reports/{id}`
- Aksi Operator: `POST /api/v1/reports/{id}/verify`, `POST /api/v1/reports/{id}/reject`, `POST /api/v1/reports/{id}/assignments`
- Aksi Petugas: `GET /api/v1/tasks/my`, `POST /api/v1/tasks/{id}/accept`, `POST /api/v1/tasks/{id}/evidence`
- Dashboard: `GET /api/v1/dashboard/summary`, `GET /api/v1/dashboard/map-points`
- Jadwal: `GET /api/v1/collection-schedules`, `POST /api/v1/collection-schedules`
- Ekspor: `GET /api/v1/reports/export` (CSV)

---

## 📜 Hak Cipta & Lisensi

**Copyright©2026 by Yusuf dan Dr. Erliyan Redy Susanto, M.Kom**  
Universitas Teknokrat Indonesia & Dinas Lingkungan Hidup Pemerintah Kota.
