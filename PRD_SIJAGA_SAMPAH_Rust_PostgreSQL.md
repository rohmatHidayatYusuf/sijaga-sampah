# PRODUCT REQUIREMENT DOCUMENT (PRD)

## SIJAGA SAMPAH

### Sistem Informasi Jaga Kebersihan dan Pengelolaan Sampah Kota

  Informasi       Detail
  --------------- -------------------------------------------
  Versi dokumen   1.0
  Tanggal         13 November 2026
  Status          Draft pedoman implementasi
  Domain          Smart City -- Smart Environment
  Backend         Rust
  Database        PostgreSQL
  Platform MVP    Web responsif / Progressive Web App (PWA)
  Wilayah pilot   Ditentukan bersama Pemerintah Daerah

> **Catatan identitas:** Gunakan berkas logo resmi Pemerintah Daerah
> yang menjadi mitra/pemilik proses dan logo resmi Universitas Teknokrat
> Indonesia. Jangan menggunakan logo hasil rekonstruksi. Ganti teks
> `Nama Mahasiswa` pada footer dengan nama mahasiswa sebelum rilis
> final.

------------------------------------------------------------------------

# 1. Ringkasan Produk

**SIJAGA SAMPAH** adalah aplikasi pengelolaan sampah berbasis web
responsif yang menghubungkan masyarakat, petugas kebersihan, operator,
pengelola bank sampah, dan pemerintah daerah dalam satu sistem.

MVP berfokus pada proses yang dapat dijalankan tanpa perangkat khusus:
pelaporan masalah persampahan, verifikasi laporan, penugasan petugas,
pembaruan status pekerjaan, pengelolaan jadwal dasar, dan dashboard
ringkasan. Sistem menyimpan riwayat aktivitas agar proses dapat
ditelusuri dan dievaluasi.

## 1.1 Visi

Mendukung pengelolaan sampah yang lebih cepat, transparan, terukur, dan
berbasis data.

## 1.2 Sasaran MVP

1.  Warga dapat membuat laporan dengan kategori, deskripsi, lokasi, dan
    foto opsional.
2.  Operator dapat memverifikasi laporan, menentukan prioritas, dan
    menugaskan petugas.
3.  Petugas dapat melihat tugas yang diberikan dan memperbarui progres.
4.  Warga dapat memantau status laporannya.
5.  Pemerintah daerah dapat melihat statistik dasar berdasarkan waktu,
    kategori, wilayah, dan status.
6.  Perubahan penting tersimpan sebagai riwayat dan audit.

## 1.3 Batasan ruang lingkup

Tidak termasuk MVP: - Sensor IoT tempat sampah. - AI untuk klasifikasi
foto. - Optimasi rute kendaraan tingkat lanjut. - Pembayaran digital
atau dompet uang. - Integrasi otomatis ke seluruh sistem pemerintah
daerah. - Aplikasi native Android/iOS terpisah.

Fitur tersebut dapat dipertimbangkan setelah MVP digunakan dan proses
operasional dievaluasi.

# 2. Latar Belakang dan Permasalahan

Pengelolaan sampah melibatkan banyak pihak dan aktivitas: laporan warga,
verifikasi lokasi, penugasan petugas, pengangkutan, dan pelaporan
kinerja. Jika informasi tersebar melalui telepon, pesan pribadi,
formulir, atau spreadsheet, data sulit ditelusuri dan dianalisis.

Masalah utama yang ditangani:

1.  **Pelaporan belum terpusat:** warga belum tentu mengetahui kanal
    resmi untuk melaporkan tumpukan sampah atau pembuangan liar.
2.  **Status tidak transparan:** pelapor kesulitan mengetahui apakah
    laporan diterima, ditinjau, ditugaskan, atau diselesaikan.
3.  **Koordinasi operasional tersebar:** operator dan petugas mungkin
    menggunakan media pencatatan yang berbeda.
4.  **Data historis tidak konsisten:** format lokasi, kategori, waktu,
    dan status dapat berbeda.
5.  **Pengambilan keputusan terbatas:** pemerintah membutuhkan ringkasan
    jumlah laporan, lokasi berulang, waktu penyelesaian, dan beban
    kerja.
6.  **Jadwal sulit dipantau:** jadwal pengangkutan dan pelaksanaannya
    perlu dikelola secara terpusat.

## 2.1 Asumsi dan prasyarat

-   Pemerintah daerah menunjuk unit kerja pemilik proses.
-   Tersedia operator dan petugas yang bertanggung jawab menindaklanjuti
    laporan.
-   Data wilayah, kategori, prioritas, dan daftar petugas disediakan
    atau disepakati.
-   Pengguna mengakses aplikasi melalui browser dan jaringan internet.
-   Penggunaan foto dan data lokasi mengikuti kebijakan privasi serta
    persetujuan pengguna.
-   Implementasi awal dilakukan di wilayah pilot terbatas.

# 3. Tujuan, Solusi, dan Indikator Keberhasilan

## 3.1 Tujuan

-   Mempermudah masyarakat menyampaikan laporan.
-   Mempercepat verifikasi dan penugasan laporan.
-   Memberikan informasi status kepada pelapor.
-   Menyediakan data operasional yang konsisten.
-   Mengurangi pencatatan manual yang tersebar.

## 3.2 Solusi

Satu aplikasi web responsif dengan portal warga dan dashboard internal.
Warga membuat laporan; operator memeriksa dan menugaskan; petugas
memperbarui progres serta bukti pekerjaan; sistem mencatat riwayat dan
menyajikan dashboard agregat.

## 3.3 KPI awal untuk pilot

Angka berikut adalah target awal untuk dievaluasi, bukan klaim hasil
yang telah dicapai.

  ------------------------------------------------------------------------
  KPI                   Definisi                               Target awal
  --------------------- --------------------- ----------------------------
  Keberhasilan          Laporan valid                                ≥ 95%
  pencatatan            tersimpan dan         
                        mendapat nomor tiket  

  Kecepatan verifikasi  Laporan valid                                ≥ 90%
                        ditinjau dalam satu   
                        hari kerja            

  Keterlacakan          Tiket valid dapat                             100%
                        dilihat statusnya     
                        oleh pelapor          

  Pembaruan tugas       Tugas yang mendapat                          ≥ 90%
                        pembaruan status      
                        petugas               

  Kelengkapan lokasi    Laporan memiliki                             ≥ 90%
                        koordinat atau alamat 
                        yang dapat            
                        diverifikasi          

  Ketersediaan layanan  Ketersediaan aplikasi                   Target 99%
                        per bulan pada jam    
                        layanan               
  ------------------------------------------------------------------------

Target perlu dikalibrasi setelah baseline operasional dan kapasitas
infrastruktur diketahui.

# 4. Target Pengguna, Role, dan Hak Akses

  -----------------------------------------------------------------------
  Role                    Pengguna                Hak akses
  ----------------------- ----------------------- -----------------------
  Warga                   Masyarakat              Registrasi/login,
                                                  membuat laporan,
                                                  melihat laporan milik
                                                  sendiri, memberi umpan
                                                  balik

  Petugas                 Petugas kebersihan      Melihat tugas yang
                                                  ditugaskan kepadanya,
                                                  memperbarui status,
                                                  mengunggah bukti

  Operator                Admin operasional       Memverifikasi, menolak
                          persampahan             dengan alasan, mengatur
                                                  prioritas, menugaskan
                                                  petugas, mengelola
                                                  jadwal

  Pengelola Bank Sampah   Pengelola unit bank     Mengelola profil dan
                          sampah                  transaksi bank sampah
                                                  jika modul diaktifkan

  Admin Sistem            Pengelola teknis        Mengelola akun, role,
                                                  master data,
                                                  konfigurasi, dan audit
                                                  teknis

  Pimpinan/Viewer         Pejabat/analis          Melihat dashboard dan
                                                  laporan agregat tanpa
                                                  mengubah data
                                                  operasional
  -----------------------------------------------------------------------

## 4.1 Aturan otorisasi

-   Warga hanya melihat laporan miliknya, kecuali informasi yang secara
    eksplisit dipublikasikan.
-   Petugas hanya melihat tugas yang ditugaskan kepadanya.
-   Operator hanya mengelola data dalam wilayah kewenangannya.
-   Pimpinan/Viewer memiliki izin baca.
-   Admin Sistem mengelola akun dan konfigurasi, tetapi setiap tindakan
    administratif tetap dicatat.
-   Role dan izin diperiksa di backend pada setiap endpoint privat;
    menyembunyikan tombol di UI bukan mekanisme keamanan.
-   Terapkan prinsip least privilege dan pembatasan wilayah bila
    diperlukan.

# 5. Ruang Lingkup dan Daftar Fitur

Prioritas: **P0** wajib untuk MVP, **P1** setelah alur inti stabil,
**P2** pengembangan lanjutan.

## 5.1 Fitur utama

  -----------------------------------------------------------------------
  ID                Fitur             Deskripsi         Prioritas
  ----------------- ----------------- ----------------- -----------------
  F-01              Autentikasi dan   Registrasi,       P0
                    profil            login, logout,    
                                      profil,           
                                      pengelolaan sesi  

  F-02              Pelaporan warga   Kategori, judul,  P0
                                      deskripsi,        
                                      lokasi, foto      
                                      opsional          

  F-03              Nomor tiket dan   Nomor tiket unik  P0
                    pelacakan         dan riwayat       
                                      status            

  F-04              Manajemen laporan Verifikasi,       P0
                                      penolakan dengan  
                                      alasan, prioritas 

  F-05              Penugasan petugas Operator          P0
                                      menugaskan        
                                      laporan kepada    
                                      petugas           

  F-06              Pembaruan         Petugas           P0
                    pekerjaan         memperbarui       
                                      status dan        
                                      mengunggah bukti  

  F-07              Dashboard         Ringkasan laporan P0
                                      dan tren dasar    

  F-08              Master data       Kategori,         P0
                                      wilayah,          
                                      prioritas, akun   
                                      petugas           

  F-09              Notifikasi        Pemberitahuan     P1
                    aplikasi          perubahan status  
                                      dan penugasan     

  F-10              Jadwal            Jadwal per        P1
                    pengangkutan      wilayah dan       
                                      status            
                                      pelaksanaan       

  F-11              Peta laporan      Pemetaan laporan  P1
                                      sesuai hak akses  

  F-12              Ekspor laporan    Ekspor CSV sesuai P1
                                      izin              

  F-13              Bank sampah       Profil unit dan   P2
                                      pencatatan        
                                      setoran dasar     

  F-14              Edukasi dan       Materi dan        P2
                    eco-point         aktivitas         
                                      partisipasi warga 
  -----------------------------------------------------------------------

## 5.2 Fitur pendukung

-   Pencarian dan filter tanggal, status, kategori, prioritas, dan
    wilayah.
-   Pagination untuk daftar data.
-   Riwayat perubahan status dan penugasan.
-   Tampilan loading, empty state, error, dan koneksi terputus.
-   Validasi ukuran dan tipe file.
-   Halaman bantuan, panduan pelaporan, dan kebijakan privasi.
-   Pengaturan profil dan preferensi notifikasi.
-   Audit log untuk tindakan administratif penting.

# 6. Kebutuhan Fungsional

  -----------------------------------------------------------------------
  ID                      Kebutuhan               Kriteria penerimaan
  ----------------------- ----------------------- -----------------------
  FR-01                   Registrasi dan login    Akun valid dapat masuk;
                                                  kredensial salah
                                                  ditolak dengan pesan
                                                  aman

  FR-02                   Role-based access       Pengguna ditolak saat
                                                  mengakses fungsi di
                                                  luar izinnya

  FR-03                   Membuat laporan         Laporan valid tersimpan
                                                  dan menerima nomor
                                                  tiket

  FR-04                   Validasi lokasi         Koordinat berada pada
                                                  rentang valid atau
                                                  alamat tersedia

  FR-05                   Lampiran foto           Tipe, ukuran, dan
                                                  jumlah file diperiksa

  FR-06                   Melihat laporan         Warga hanya melihat
                                                  laporan miliknya

  FR-07                   Verifikasi              Operator dapat
                                                  memverifikasi atau
                                                  menolak dengan alasan

  FR-08                   Penugasan               Penugasan menyimpan
                                                  petugas, operator, dan
                                                  waktu

  FR-09                   Pembaruan status        Hanya petugas
                                                  terkait/operator
                                                  berwenang yang dapat
                                                  mengubah status

  FR-10                   Bukti pekerjaan         Bukti terhubung ke
                                                  laporan dan memiliki
                                                  metadata

  FR-11                   Riwayat status          Aktor, status
                                                  asal/tujuan, waktu, dan
                                                  catatan tercatat

  FR-12                   Dashboard               Data agregat mengikuti
                                                  filter dan kewenangan

  FR-13                   Master data             Hanya role berwenang
                                                  dapat mengubah
                                                  kategori/wilayah

  FR-14                   Pencarian               Filter dapat
                                                  dikombinasikan dan
                                                  pagination konsisten

  FR-15                   Notifikasi              Peristiwa penting
                                                  menghasilkan notifikasi
                                                  dalam aplikasi jika
                                                  fitur aktif

  FR-16                   Audit                   Aksi sensitif dicatat
                                                  tanpa menyimpan
                                                  password atau token

  FR-17                   Ekspor                  Hanya data yang boleh
                                                  diakses peminta yang
                                                  diekspor

  FR-18                   Jadwal                  Jadwal memiliki
                                                  wilayah, waktu, status,
                                                  dan pembuat
  -----------------------------------------------------------------------

## 6.1 Siklus status laporan

Alur normal:

`submitted` → `under_review` → `verified` → `assigned` → `in_progress` →
`resolved` → `closed`

Status alternatif: - `rejected`: laporan tidak valid/di luar kewenangan;
alasan wajib. - `reopened`: laporan dibuka kembali sesuai kebijakan
operator.

Backend harus memvalidasi transisi status. Klien tidak boleh mengirim
status bebas. Setiap perubahan status dan penugasan dicatat dalam
transaksi database.

# 7. Kebutuhan Nonfungsional

  -----------------------------------------------------------------------
  Area                                Kebutuhan
  ----------------------------------- -----------------------------------
  Kinerja                             Target awal API umum p95 ≤ 800 ms
                                      pada beban pilot yang disepakati,
                                      tidak termasuk unggah file/layanan
                                      eksternal

  Skalabilitas                        Backend stateless agar instance
                                      dapat ditambah; PostgreSQL sebagai
                                      sumber data utama

  Ketersediaan                        Target awal 99% per bulan pada jam
                                      layanan, di luar pemeliharaan
                                      terjadwal

  Keamanan                            HTTPS, hashing password, RBAC
                                      server-side, validasi input, audit
                                      log

  Privasi                             Mengumpulkan data seperlunya dan
                                      menetapkan retensi bersama pemilik
                                      proses

  Responsif                           Mendukung ponsel, tablet, dan
                                      desktop

  Aksesibilitas                       Kontras cukup, label input,
                                      navigasi keyboard dasar, pesan
                                      error jelas

  Kompatibilitas                      Browser modern yang masih didukung

  Keandalan                           Transaksi untuk operasi multi-tabel
                                      dan constraint integritas database

  Pemulihan                           Backup terjadwal dan uji restore
                                      berkala

  Observabilitas                      Log terstruktur, metrik, health
                                      check, dan pelacakan error

  Pemeliharaan                        Kode modular, migrasi database,
                                      dokumentasi API

  Bahasa                              Bahasa Indonesia pada MVP

  Waktu                               Simpan timestamp UTC
                                      (`timestamptz`); tampilkan sesuai
                                      zona waktu operasional
  -----------------------------------------------------------------------

# 8. Alur Kerja Aplikasi (User Flow)

## 8.1 Warga membuat laporan

1.  Warga membuka aplikasi dan login/registrasi.
2.  Memilih **Buat Laporan**.
3.  Memilih kategori dan mengisi judul serta deskripsi.
4.  Menentukan lokasi melalui GPS/peta atau alamat manual.
5.  Menambahkan foto jika tersedia.
6.  Memeriksa ringkasan dan mengirim laporan.
7.  Backend memvalidasi, menyimpan laporan, dan menghasilkan nomor
    tiket.
8.  Warga melihat halaman sukses dan dapat melacak status.

## 8.2 Operator memproses laporan

1.  Operator login dan membuka antrean laporan.
2.  Memeriksa kategori, deskripsi, lokasi, dan lampiran.
3.  Memverifikasi atau menolak dengan alasan.
4.  Jika valid, menetapkan prioritas dan petugas.
5.  Sistem menyimpan penugasan, riwayat status, dan notifikasi.
6.  Petugas melihat tugas pada dashboard.

## 8.3 Petugas menyelesaikan tugas

1.  Petugas login dan membuka daftar tugasnya.
2.  Melihat detail lokasi dan instruksi.
3.  Memperbarui status menjadi `in_progress`.
4.  Setelah pekerjaan selesai, mengunggah bukti dan catatan.
5.  Backend memvalidasi hak akses dan transisi status.
6.  Status menjadi `resolved`; operator dapat meninjau dan menutup
    laporan sesuai SOP.
7.  Warga dapat melihat pembaruan.

## 8.4 Pimpinan melihat dashboard

1.  Pimpinan login.
2.  Memilih rentang tanggal dan wilayah.
3.  Melihat jumlah laporan, status, kategori, tren, dan waktu
    penyelesaian.
4.  Mengekspor ringkasan jika memiliki izin.

# 9. Rancangan Database PostgreSQL

## 9.1 Prinsip desain

-   PostgreSQL menjadi database relasional utama.
-   Primary key menggunakan UUID.
-   Timestamp menggunakan `timestamptz`.
-   Foreign key, `NOT NULL`, `CHECK`, dan unique constraint menjaga
    integritas.
-   Foto disimpan pada object storage privat; database hanya menyimpan
    metadata dan object key.
-   Koordinat MVP dapat disimpan sebagai `numeric`; PostGIS dapat
    ditambahkan bila kebutuhan geospasial meningkat.
-   Password hanya disimpan sebagai hash.

## 9.2 Tabel inti dan relasi

  -------------------------------------------------------------------------------
  Tabel                       Fungsi                      Relasi
  --------------------------- --------------------------- -----------------------
  `users`                     Akun                        Terhubung ke role,
                                                          laporan, notifikasi

  `roles`                     Daftar role                 Terhubung melalui
                                                          `user_roles`

  `user_roles`                Relasi akun-role            Many-to-many

  `regions`                   Wilayah                     Memiliki laporan dan
                              administratif/operasional   jadwal

  `waste_categories`          Kategori laporan            Direferensikan laporan

  `reports`                   Laporan warga               Milik user, kategori,
                                                          wilayah

  `report_attachments`        Metadata lampiran           Milik laporan

  `report_status_history`     Riwayat status              Milik laporan dan aktor

  `assignments`               Penugasan                   Menghubungkan laporan
                                                          dan petugas

  `collection_schedules`      Jadwal pengangkutan         Terhubung wilayah dan
                                                          penanggung jawab

  `notifications`             Notifikasi                  Milik user

  `audit_logs`                Jejak tindakan penting      Memiliki aktor opsional

  `waste_banks`               Unit bank sampah (P2)       Memiliki transaksi

  `waste_bank_transactions`   Catatan setoran (P2)        Menghubungkan unit dan
                                                          warga
  -------------------------------------------------------------------------------

## 9.3 ERD konseptual

``` mermaid
erDiagram
    USERS ||--o{ USER_ROLES : has
    ROLES ||--o{ USER_ROLES : assigned
    USERS ||--o{ REPORTS : submits
    REGIONS ||--o{ REPORTS : contains
    WASTE_CATEGORIES ||--o{ REPORTS : classifies
    REPORTS ||--o{ REPORT_ATTACHMENTS : has
    REPORTS ||--o{ REPORT_STATUS_HISTORY : tracks
    USERS ||--o{ REPORT_STATUS_HISTORY : acts
    REPORTS ||--o{ ASSIGNMENTS : receives
    USERS ||--o{ ASSIGNMENTS : assigned_to
    REGIONS ||--o{ COLLECTION_SCHEDULES : schedules
    USERS ||--o{ NOTIFICATIONS : receives
    USERS ||--o{ AUDIT_LOGS : performs
    WASTE_BANKS ||--o{ WASTE_BANK_TRANSACTIONS : records
    USERS ||--o{ WASTE_BANK_TRANSACTIONS : deposits
```

## 9.4 DDL inti

DDL berikut merupakan baseline. Terapkan melalui migration tool, review,
dan seed data; jangan mengubah skema produksi secara manual.

``` sql
CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE roles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(40) NOT NULL UNIQUE,
    name VARCHAR(100) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    full_name VARCHAR(150) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    phone VARCHAR(30),
    password_hash TEXT NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE user_roles (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role_id UUID NOT NULL REFERENCES roles(id) ON DELETE RESTRICT,
    assigned_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (user_id, role_id)
);

CREATE TABLE regions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    parent_id UUID REFERENCES regions(id) ON DELETE RESTRICT,
    code VARCHAR(50) UNIQUE,
    name VARCHAR(150) NOT NULL,
    region_type VARCHAR(40) NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE TABLE waste_categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(50) NOT NULL UNIQUE,
    name VARCHAR(120) NOT NULL,
    description TEXT,
    is_active BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE TABLE reports (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    ticket_number VARCHAR(32) NOT NULL UNIQUE,
    reporter_id UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    category_id UUID NOT NULL REFERENCES waste_categories(id) ON DELETE RESTRICT,
    region_id UUID REFERENCES regions(id) ON DELETE SET NULL,
    title VARCHAR(160) NOT NULL,
    description TEXT NOT NULL,
    address_text TEXT,
    latitude NUMERIC(9,6),
    longitude NUMERIC(9,6),
    status VARCHAR(30) NOT NULL DEFAULT 'submitted'
      CHECK (status IN ('submitted','under_review','verified','assigned',
        'in_progress','resolved','closed','rejected','reopened')),
    priority VARCHAR(20) NOT NULL DEFAULT 'normal'
      CHECK (priority IN ('low','normal','high','urgent')),
    rejection_reason TEXT,
    submitted_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (latitude IS NULL OR latitude BETWEEN -90 AND 90),
    CHECK (longitude IS NULL OR longitude BETWEEN -180 AND 180),
    CHECK (latitude IS NOT NULL OR longitude IS NOT NULL OR address_text IS NOT NULL)
);

CREATE TABLE report_attachments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    report_id UUID NOT NULL REFERENCES reports(id) ON DELETE CASCADE,
    uploaded_by UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    object_key TEXT NOT NULL UNIQUE,
    original_filename VARCHAR(255),
    content_type VARCHAR(100) NOT NULL,
    file_size_bytes BIGINT NOT NULL CHECK (file_size_bytes > 0),
    attachment_type VARCHAR(30) NOT NULL
      CHECK (attachment_type IN ('report_photo','completion_evidence')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE report_status_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    report_id UUID NOT NULL REFERENCES reports(id) ON DELETE CASCADE,
    changed_by UUID REFERENCES users(id) ON DELETE SET NULL,
    from_status VARCHAR(30),
    to_status VARCHAR(30) NOT NULL,
    note TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE assignments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    report_id UUID NOT NULL REFERENCES reports(id) ON DELETE CASCADE,
    assignee_id UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    assigned_by UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    instructions TEXT,
    assigned_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    accepted_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    is_active BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE TABLE collection_schedules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    region_id UUID NOT NULL REFERENCES regions(id) ON DELETE RESTRICT,
    responsible_user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    title VARCHAR(150) NOT NULL,
    scheduled_at TIMESTAMPTZ NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'planned'
      CHECK (status IN ('planned','in_progress','completed','cancelled')),
    notes TEXT,
    created_by UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE notifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(160) NOT NULL,
    body TEXT NOT NULL,
    target_type VARCHAR(40),
    target_id UUID,
    read_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE audit_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    actor_id UUID REFERENCES users(id) ON DELETE SET NULL,
    action VARCHAR(100) NOT NULL,
    entity_type VARCHAR(80) NOT NULL,
    entity_id UUID,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    ip_address INET,
    user_agent TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_reports_status_submitted
    ON reports(status, submitted_at DESC);
CREATE INDEX idx_reports_reporter_submitted
    ON reports(reporter_id, submitted_at DESC);
CREATE INDEX idx_reports_region_submitted
    ON reports(region_id, submitted_at DESC);
CREATE INDEX idx_reports_category ON reports(category_id);
CREATE INDEX idx_history_report_created
    ON report_status_history(report_id, created_at DESC);
CREATE INDEX idx_assignments_assignee_active
    ON assignments(assignee_id, is_active);
CREATE INDEX idx_notifications_user_created
    ON notifications(user_id, created_at DESC);
CREATE INDEX idx_audit_entity
    ON audit_logs(entity_type, entity_id, created_at DESC);
```

## 9.5 Tabel opsional bank sampah (P2)

``` sql
CREATE TABLE waste_banks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    region_id UUID REFERENCES regions(id) ON DELETE SET NULL,
    name VARCHAR(160) NOT NULL,
    address TEXT,
    contact_phone VARCHAR(30),
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE waste_bank_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    waste_bank_id UUID NOT NULL REFERENCES waste_banks(id) ON DELETE RESTRICT,
    customer_id UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    waste_type VARCHAR(100) NOT NULL,
    weight_kg NUMERIC(10,3) NOT NULL CHECK (weight_kg > 0),
    unit_price NUMERIC(14,2) CHECK (unit_price >= 0),
    recorded_by UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    notes TEXT
);
```

## 9.6 Catatan implementasi database

-   Seed role dan kategori awal melalui migration/seed terkontrol.
-   Buat laporan dan riwayat status awal dalam satu transaksi.
-   Perubahan status dan penambahan riwayat wajib atomik.
-   Gunakan pagination cursor untuk daftar besar.
-   Tambahkan indeks berdasarkan pola query yang terukur.
-   Hindari N+1 query; gunakan join atau query terencana.
-   Jika pemetaan dan pencarian radius menjadi kebutuhan utama,
    pertimbangkan PostGIS dengan `geography(Point, 4326)` dan GiST
    index.
-   Tetapkan kebijakan retensi dan penghapusan data bersama pemilik
    proses.

# 10. Arsitektur Sistem dan Teknologi

## 10.1 Arsitektur logis

``` mermaid
flowchart TD
    A[Browser Warga / Petugas / Operator] --> B[Frontend Web Responsif]
    B --> C[HTTPS REST API]
    C --> D[Rust Backend]
    D --> E[Authentication dan RBAC]
    D --> F[Application Services]
    F --> G[(PostgreSQL)]
    F --> H[Object Storage Foto]
    F --> I[Notification Service]
    D --> J[Logs dan Metrics]
    K[Admin / Pimpinan] --> B
```

## 10.2 Stack teknologi

  -----------------------------------------------------------------------
  Lapisan                 Teknologi               Kegunaan
  ----------------------- ----------------------- -----------------------
  Frontend                React + TypeScript atau UI web responsif
                          SvelteKit               

  Styling                 Tailwind CSS            Sistem desain dan
                                                  layout

  Backend                 Rust                    API dan business logic

  HTTP framework          Axum                    Routing dan handler
                                                  HTTP

  Async runtime           Tokio                   I/O asynchronous

  Database                PostgreSQL              Data relasional dan
                                                  transaksi

  DB access               SQLx                    Query async dan
                                                  integrasi PostgreSQL

  Serialization           Serde                   JSON request/response

  Validation              Validator atau validasi Validasi payload dan
                          domain                  aturan bisnis

  Password hashing        Argon2id                Hash password

  API docs                OpenAPI                 Kontrak dan dokumentasi
                                                  endpoint

  Map                     Leaflet + OpenStreetMap Peta laporan

  File storage            S3-compatible storage   Penyimpanan lampiran

  Reverse proxy           Nginx atau Caddy        TLS dan routing

  Deployment              Docker                  Konsistensi lingkungan

  CI/CD                   GitHub Actions/GitLab   Build dan test otomatis
                          CI                      

  Observability           tracing + metrics +     Diagnostik layanan
                          central logs            
  -----------------------------------------------------------------------

## 10.3 Struktur backend Rust

``` text
backend/
  Cargo.toml
  migrations/
  src/
    main.rs
    config/
    app_state.rs
    routes/
      auth.rs
      users.rs
      reports.rs
      assignments.rs
      schedules.rs
      dashboard.rs
      notifications.rs
      admin.rs
    handlers/
    services/
    repositories/
    models/
    dto/
    middleware/
      authentication.rs
      authorization.rs
      request_id.rs
      rate_limit.rs
    errors/
    validation/
    telemetry/
```

Pemisahan tanggung jawab: - **Routes/handlers:** menerima request,
parsing DTO, dan menyusun response. - **Services:** aturan bisnis dan
orkestrasi transaksi. - **Repositories:** query SQLx ke PostgreSQL. -
**Models:** representasi data internal. - **DTO:** kontrak API terpisah
dari model database. - **Middleware:** autentikasi, otorisasi, rate
limit, dan request ID. - **Migrations:** perubahan skema database yang
dapat dilacak.

## 10.4 Prinsip implementasi Rust

-   Gunakan `Result` dan error terstruktur; jangan mengabaikan error.
-   Hindari `unwrap()` pada jalur request produksi.
-   Validasi payload di batas API dan aturan bisnis di service.
-   Gunakan parameterized query SQLx, bukan menggabungkan input ke
    string SQL.
-   Gunakan connection pool dan transaksi SQLx untuk operasi
    multi-tabel.
-   Simpan konfigurasi/secret di environment atau secret manager.
-   Terapkan graceful shutdown dan health/readiness check.
-   Gunakan `tracing` untuk log terstruktur dan korelasi request.

# 11. Rancangan API

API menggunakan REST dengan prefix `/api/v1`. Format umum JSON; unggah
file menggunakan `multipart/form-data`.

## 11.1 Konvensi

-   Endpoint privat memerlukan autentikasi.
-   Otorisasi diperiksa di backend berdasarkan role, wilayah, dan
    kepemilikan resource.
-   ID menggunakan UUID.
-   Timestamp menggunakan ISO 8601/RFC 3339.
-   Daftar data memakai pagination dan filter yang konsisten.
-   Error menggunakan format standar.
-   Endpoint pembuatan dapat mendukung `Idempotency-Key` untuk retry
    aman.
-   Jangan mengembalikan `password_hash`, token, atau metadata sensitif.

Contoh error:

``` json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Data yang dikirim belum valid.",
    "details": [
      { "field": "title", "message": "Judul wajib diisi." }
    ],
    "request_id": "request-correlation-id"
  }
}
```

## 11.2 Autentikasi dan profil

  -------------------------------------------------------------------------------
  Method            Endpoint                  Akses             Keterangan
  ----------------- ------------------------- ----------------- -----------------
  POST              `/auth/register`          Publik            Registrasi warga

  POST              `/auth/login`             Publik            Login

  POST              `/auth/logout`            Login             Mengakhiri sesi

  GET               `/auth/me`                Login             Profil dan role

  PATCH             `/users/me`               Login             Memperbarui
                                                                profil sendiri

  POST              `/auth/forgot-password`   Publik            Memulai pemulihan
                                                                akun jika kanal
                                                                tersedia

  POST              `/auth/reset-password`    Token reset       Mengatur password
                                                                baru
  -------------------------------------------------------------------------------

## 11.3 Laporan

  --------------------------------------------------------------------------------------------
  Method            Endpoint                      Akses                      Keterangan
  ----------------- ----------------------------- -------------------------- -----------------
  POST              `/reports`                    Warga                      Membuat laporan

  GET               `/reports`                    Sesuai role                Daftar laporan;
                                                                             warga hanya
                                                                             miliknya

  GET               `/reports/{id}`               Pemilik/operator/petugas   Detail laporan
                                                  terkait                    

  PATCH             `/reports/{id}`               Operator                   Mengubah data
                                                                             operasional yang
                                                                             diizinkan

  POST              `/reports/{id}/attachments`   Pemilik/petugas terkait    Unggah lampiran

  GET               `/reports/{id}/history`       Sesuai akses laporan       Riwayat status

  POST              `/reports/{id}/verify`        Operator                   Verifikasi

  POST              `/reports/{id}/reject`        Operator                   Menolak dengan
                                                                             alasan

  POST              `/reports/{id}/assignments`   Operator                   Menugaskan
                                                                             petugas

  POST              `/reports/{id}/status`        Petugas terkait/operator   Transisi status
                                                                             tervalidasi

  POST              `/reports/{id}/reopen`        Operator/pelapor sesuai    Membuka kembali
                                                  kebijakan                  laporan
  --------------------------------------------------------------------------------------------

## 11.4 Tugas dan jadwal

  -----------------------------------------------------------------------------------------------
  Method            Endpoint                       Akses              Keterangan
  ----------------- ------------------------------ ------------------ ---------------------------
  GET               `/tasks/my`                    Petugas            Daftar tugas sendiri

  GET               `/tasks/{id}`                  Petugas            Detail tugas
                                                   terkait/operator   

  POST              `/tasks/{id}/accept`           Petugas terkait    Menerima tugas

  POST              `/tasks/{id}/evidence`         Petugas terkait    Mengunggah bukti

  GET               `/collection-schedules`        Login              Melihat jadwal sesuai
                                                                      kewenangan

  POST              `/collection-schedules`        Operator           Membuat jadwal

  PATCH             `/collection-schedules/{id}`   Operator           Memperbarui jadwal

  DELETE            `/collection-schedules/{id}`   Operator           Membatalkan/menonaktifkan
                                                                      sesuai aturan
  -----------------------------------------------------------------------------------------------

## 11.5 Dashboard, master data, dan notifikasi

  ----------------------------------------------------------------------------------------
  Method            Endpoint                     Akses                   Keterangan
  ----------------- ---------------------------- ----------------------- -----------------
  GET               `/dashboard/summary`         Operator/viewer/admin   KPI ringkas

  GET               `/dashboard/trends`          Operator/viewer/admin   Tren laporan

  GET               `/dashboard/map-points`      Operator/viewer/admin   Titik laporan
                                                                         yang diizinkan

  GET               `/reports/export`            Operator/viewer sesuai  Ekspor CSV
                                                 izin                    

  GET               `/master/categories`         Login                   Kategori aktif

  POST              `/admin/categories`          Admin sistem            Membuat kategori

  PATCH             `/admin/categories/{id}`     Admin sistem            Mengubah kategori

  GET               `/master/regions`            Login                   Wilayah

  GET               `/notifications`             Login                   Notifikasi
                                                                         pengguna

  PATCH             `/notifications/{id}/read`   Pemilik notifikasi      Tandai dibaca

  GET               `/health/live`               Infrastruktur           Liveness

  GET               `/health/ready`              Infrastruktur           Readiness
  ----------------------------------------------------------------------------------------

## 11.6 Contoh payload membuat laporan

``` json
{
  "category_id": "00000000-0000-0000-0000-000000000001",
  "title": "Tumpukan sampah di dekat pasar",
  "description": "Sampah belum terangkut dan mengganggu akses jalan.",
  "region_id": "00000000-0000-0000-0000-000000000002",
  "address_text": "Area sekitar pasar",
  "latitude": -5.429,
  "longitude": 105.262
}
```

Contoh response:

``` json
{
  "data": {
    "id": "00000000-0000-0000-0000-000000000003",
    "ticket_number": "SJG-2026-000001",
    "status": "submitted",
    "submitted_at": "2026-11-13T06:00:00Z"
  }
}
```

# 12. Desain UI/UX

## 12.1 Prinsip desain

-   Modern, profesional, bersih, dan sesuai konsep Smart City.
-   Mobile-first untuk warga dan petugas lapangan.
-   Navigasi sederhana dengan Bahasa Indonesia.
-   Status menggunakan teks dan ikon, bukan warna saja.
-   Formulir bertahap dengan validasi dekat kolom.
-   Dashboard menampilkan informasi yang dapat ditindaklanjuti.
-   Menyediakan loading, empty state, error, dan pesan koneksi lambat.

## 12.2 Header dan logo institusi

Pada halaman login dan header aplikasi tampilkan: 1. **Logo Pemerintah
Daerah** yang menjadi pemilik/pilot proyek. 2. **Logo Universitas
Teknokrat Indonesia**.

Ketentuan: - Gunakan berkas logo resmi yang disediakan atau disetujui
institusi. - Tampilkan berdampingan pada desktop dan tetap proporsional
pada ponsel. - Jangan mengubah rasio, bentuk, atau warna logo resmi. -
Sediakan alt text. - Pastikan penggunaan logo telah mendapat persetujuan
pihak terkait.

Contoh susunan:

``` text
[LOGO PEMDA] [LOGO UNIVERSITAS TEKNOKRAT INDONESIA]
SIJAGA SAMPAH
Sistem Informasi Jaga Kebersihan dan Pengelolaan Sampah Kota
```

## 12.3 Halaman warga

-   Header logo dan nama aplikasi.
-   Tombol utama **Buat Laporan**.
-   Ringkasan laporan: Menunggu, Diproses, Selesai.
-   Daftar laporan terbaru dengan tiket, kategori, tanggal, dan status.
-   Akses ke jadwal dan panduan pemilahan.
-   Footer wajib.

## 12.4 Formulir laporan

Langkah: 1. Pilih kategori. 2. Isi judul dan deskripsi. 3. Tentukan
lokasi melalui peta/GPS atau alamat. 4. Tambahkan foto opsional. 5.
Tinjau ringkasan. 6. Kirim laporan.

Jika GPS tidak tersedia, warga tetap dapat memasukkan alamat atau
memilih titik pada peta.

## 12.5 Dashboard operator

-   Kartu ringkasan total laporan dan status.
-   Antrean laporan baru.
-   Filter tanggal, wilayah, kategori, dan prioritas.
-   Peta laporan.
-   Tabel laporan dan aksi sesuai role.
-   Panel detail untuk verifikasi dan penugasan.

## 12.6 Dashboard petugas

-   Tugas hari ini.
-   Daftar tugas berdasarkan status.
-   Detail lokasi dan instruksi.
-   Tombol mulai dan selesaikan tugas.
-   Unggah bukti serta catatan pekerjaan.

## 12.7 Dashboard pimpinan

-   Total laporan periode terpilih.
-   Persentase penyelesaian.
-   Rata-rata waktu penyelesaian.
-   Tren laporan.
-   Sebaran wilayah dan kategori.
-   Ekspor ringkasan sesuai izin.

## 12.8 Footer wajib di seluruh halaman

Teks yang harus ditampilkan persis:

**Copyright©2026 by Nama Mahasiswa dan Dr. Erliyan Redy Susanto, M.Kom**

Footer harus muncul pada seluruh halaman, termasuk login, halaman
publik, dan dashboard. Ganti `Nama Mahasiswa` dengan nama sebenarnya
sebelum rilis. Implementasikan sebagai komponen bersama agar konsisten
dan tidak menutupi konten pada layar kecil.

# 13. Keamanan Sistem dan Validasi Data

## 13.1 Autentikasi dan sesi

-   Hash password dengan Argon2id menggunakan parameter yang sesuai
    kapasitas server.
-   Untuk web first-party, utamakan cookie sesi `HttpOnly`, `Secure`,
    dan `SameSite` yang sesuai.
-   Terapkan masa berlaku sesi, logout, dan revokasi.
-   Batasi percobaan login dan hindari pesan yang membocorkan keberadaan
    akun.
-   Token reset password harus acak, sekali pakai, dan memiliki masa
    kedaluwarsa.
-   Jangan menyimpan password plaintext atau token rahasia dalam log.

## 13.2 Otorisasi

-   Terapkan RBAC pada middleware dan service backend.
-   Periksa kepemilikan laporan pada setiap operasi berbasis ID.
-   Uji IDOR dan akses lintas wilayah.
-   Jangan mengandalkan pembatasan tampilan UI sebagai kontrol keamanan.

## 13.3 Validasi input

-   Batasi panjang judul, deskripsi, alamat, dan nomor telepon.
-   Validasi UUID dan foreign key.
-   Validasi latitude -90 sampai 90 dan longitude -180 sampai 180.
-   Validasi kategori, prioritas, dan status terhadap nilai yang
    diizinkan.
-   Validasi transisi status di service domain.
-   Gunakan parameterized query SQLx.
-   Batasi ukuran request dan jumlah lampiran.
-   Normalisasi nama file; jangan jadikan nama file pengguna sebagai
    path penyimpanan.

## 13.4 Keamanan file

-   Izinkan hanya format yang diperlukan, misalnya JPEG, PNG, WebP.
-   Periksa MIME type dan signature file, bukan ekstensi saja.
-   Terapkan batas ukuran/jumlah.
-   Simpan file di object storage privat di luar web root.
-   Gunakan object key acak dan URL sementara bila perlu.
-   Pertimbangkan pemindaian malware dan penghapusan metadata EXIF yang
    dapat memuat lokasi pribadi.
-   Jangan mengeksekusi file unggahan atau merendernya sebagai konten
    aktif.

## 13.5 Infrastruktur dan privasi

-   Wajib HTTPS/TLS.
-   Secret tidak boleh masuk repository.
-   Database tidak diekspos langsung ke internet.
-   Batasi hak akses akun database.
-   Backup terenkripsi dan uji restore.
-   Rate limit pada login, registrasi, dan pembuatan laporan.
-   Proteksi CSRF jika memakai cookie sesi.
-   Terapkan security headers dan Content Security Policy.
-   Redaksi data sensitif dari log.
-   Tetapkan kebijakan retensi, koreksi, ekspor, dan penghapusan data
    bersama Pemda.
-   Lakukan security review sebelum produksi.

# 14. Notifikasi

MVP dapat menggunakan notifikasi dalam aplikasi untuk: - Laporan
berhasil dikirim. - Laporan diverifikasi atau ditolak. - Petugas
ditugaskan. - Status pekerjaan berubah. - Laporan selesai atau dibuka
kembali.

Email, WhatsApp Business API, dan push notification dapat ditambahkan
setelah tersedia persetujuan, kredensial, anggaran operasional, dan
kebijakan pengiriman. Kanal eksternal bukan ketergantungan MVP.

# 15. Tahapan Pengembangan

Estimasi awal untuk tim kecil; perlu disesuaikan dengan jumlah
pengembang, kesiapan data, dan proses persetujuan Pemda.

  ------------------------------------------------------------------------
  Tahap                                     Estimasi Hasil
  --------------------- ---------------------------- ---------------------
  0\. Validasi                              1 minggu Wawancara
  kebutuhan                                          stakeholder, SOP,
                                                     wilayah pilot,
                                                     acceptance criteria

  1\. Fondasi teknis                     1--2 minggu Repository, CI,
                                                     Docker, PostgreSQL,
                                                     migrasi, autentikasi,
                                                     RBAC, layout

  2\. Modul laporan                         2 minggu Form laporan, tiket,
                                                     lampiran,
                                                     daftar/detail laporan

  3\. Operasional                           2 minggu Verifikasi,
                                                     penugasan, dashboard
                                                     petugas, riwayat
                                                     status

  4\. Dashboard dan                      1--2 minggu Ringkasan, filter,
  jadwal                                             jadwal, ekspor dasar

  5\. Uji coba dan                       1--2 minggu UAT, perbaikan bug,
  perbaikan                                          hardening,
                                                     dokumentasi, pilot
                                                     release
  ------------------------------------------------------------------------

Perkiraan total MVP: **8--11 minggu**, dengan asumsi ruang lingkup
terkendali dan stakeholder tersedia.

## 15.1 Deliverable

-   Dokumen kebutuhan dan alur proses yang disetujui.
-   Desain UI/UX dan prototype.
-   Source code frontend dan backend.
-   Migration SQL dan seed data.
-   Dokumentasi OpenAPI.
-   Test report.
-   Panduan deployment dan backup/restore.
-   Panduan pengguna warga, operator, dan petugas.

# 16. Strategi Pengujian

  -----------------------------------------------------------------------
  Jenis pengujian         Fokus                   Contoh
  ----------------------- ----------------------- -----------------------
  Unit test               Fungsi domain           Validasi transisi
                                                  status

  Integration test        Rust dengan PostgreSQL  Laporan dan riwayat
                                                  tersimpan atomik

  API test                Kontrak endpoint        Login, create report,
                                                  assign task

  Authorization test      Pembatasan akses        Warga tidak dapat
                                                  melihat laporan orang
                                                  lain

  UI test                 Interaksi               Form laporan di ponsel

  Responsive test         Layout                  Ponsel, tablet, desktop

  Performance test        Beban API/query         Daftar laporan dan
                                                  dashboard

  Security test           Kerentanan umum         SQL injection, XSS,
                                                  CSRF, IDOR, rate limit

  UAT                     Proses lapangan         Operator memproses
                                                  laporan wilayah pilot

  Recovery test           Pemulihan               Restore backup ke
                                                  lingkungan uji
  -----------------------------------------------------------------------

## 16.1 Skenario penerimaan

1.  Warga dapat registrasi dan login.
2.  Warga dapat mengirim laporan valid dan menerima nomor tiket.
3.  Sistem menolak koordinat di luar rentang.
4.  Sistem menolak file dengan tipe/ukuran tidak diizinkan.
5.  Warga A tidak dapat membuka laporan privat Warga B.
6.  Operator dapat memverifikasi dan menugaskan laporan.
7.  Petugas yang tidak ditugaskan tidak dapat mengubah tugas.
8.  Petugas terkait dapat memperbarui status dan mengunggah bukti.
9.  Setiap perubahan status menghasilkan riwayat.
10. Dashboard mengikuti filter tanggal/wilayah.
11. Viewer tidak dapat mengubah data operasional.
12. Logo dan footer tampil konsisten pada seluruh halaman.
13. Aplikasi dapat digunakan pada layar ponsel dan desktop.
14. Backup dapat dipulihkan di lingkungan uji.

## 16.2 Definition of Done

Fitur selesai jika acceptance criteria terpenuhi, test relevan lulus,
otorisasi dan validasi backend diterapkan, tidak ada bug
blocker/critical yang diketahui, dokumentasi diperbarui, UI diuji di
ponsel dan desktop, migrasi tersedia untuk perubahan database, serta
hasil ditinjau reviewer dan pemilik proses.

# 17. Risiko dan Mitigasi

  -----------------------------------------------------------------------
  Risiko                  Dampak                  Mitigasi
  ----------------------- ----------------------- -----------------------
  Master data             Data tidak konsisten    Validasi master data
  wilayah/kategori belum                          sebelum pilot
  siap                                            

  Petugas tidak           Dashboard tidak akurat  SOP ringkas, pelatihan,
  memperbarui status                              dan UI sederhana

  Lokasi/foto tidak       Laporan sulit           Panduan warga dan
  akurat                  ditindaklanjuti         verifikasi operator

  Internet tidak stabil   Pengiriman gagal        Pesan error jelas dan
                                                  retry aman; draft
                                                  offline dapat menjadi
                                                  pengembangan

  Spam laporan            Beban operator          Rate limit, moderasi,
                          meningkat               dan pembatasan akun

  Perubahan kebutuhan     Jadwal mundur           Backlog prioritas dan
                                                  persetujuan perubahan
                                                  scope

  Kebocoran data pribadi  Risiko privasi          Minimasi data, kontrol
                                                  akses, enkripsi, audit,
                                                  prosedur insiden

  Layanan peta/notifikasi Fitur pendukung         Abstraksi provider dan
  terganggu               terganggu               fallback manual
  -----------------------------------------------------------------------

# 18. Infrastruktur dan Deployment

## 18.1 Lingkungan

-   **Development:** lokal pengembang dengan Docker Compose.
-   **Staging:** menyerupai produksi untuk UAT dan regression test.
-   **Production:** server/hosting yang disetujui Pemda atau institusi.

## 18.2 Komponen minimum

-   Frontend web.
-   Rust API service.
-   PostgreSQL.
-   Object storage untuk lampiran.
-   Reverse proxy dengan TLS.
-   Sistem log dan monitoring.
-   Backup database terjadwal.

Spesifikasi CPU, RAM, storage, dan bandwidth ditentukan setelah jumlah
pengguna, volume laporan, ukuran foto, serta hasil uji beban diketahui.

## 18.3 Operasional

-   Jalankan migration sebagai langkah deployment terkontrol.
-   Gunakan health check API dan koneksi database.
-   Terapkan backup harian dengan retensi yang disepakati.
-   Uji restore secara berkala.
-   Monitor error rate, latency, kapasitas disk, koneksi database, dan
    kegagalan unggah.
-   Siapkan prosedur rollback aplikasi dan pemulihan database.

# 19. Roadmap Setelah MVP

**P1 -- Penguatan operasional** - Peta hotspot dan analisis laporan
berulang. - Notifikasi eksternal yang disetujui. - Ekspor laporan lebih
lengkap. - Pengelolaan jadwal pengangkutan lebih rinci.

**P2 -- Ekosistem persampahan** - Modul bank sampah dan pencatatan
setoran. - Edukasi dan tantangan kebersihan. - Analitik tren dan
indikator kinerja.

**P3 -- Integrasi lanjutan setelah studi kelayakan** - Integrasi sensor
IoT pada lokasi terpilih. - Analisis citra berbantuan AI dengan validasi
manusia. - Analisis rute berbasis data armada dan kondisi lapangan. -
Integrasi data dengan platform Smart City Pemda.

# 20. Keputusan Produk yang Perlu Disepakati Sebelum Implementasi

1.  Nama Pemda dan unit kerja pemilik proses.
2.  Wilayah pilot dan struktur wilayah administratif.
3.  SOP verifikasi, penolakan, penugasan, penyelesaian, dan pembukaan
    kembali laporan.
4.  Role final dan batas kewenangan per wilayah.
5.  Kategori sampah, prioritas, serta SLA operasional.
6.  Kebijakan foto, data lokasi, privasi, retensi, dan penghapusan.
7.  Penyedia hosting, object storage, domain, dan layanan peta.
8.  Kanal notifikasi yang disetujui.
9.  Berkas logo resmi dan persetujuan penggunaannya.
10. Nama mahasiswa untuk menggantikan placeholder footer.

# 21. Kesimpulan

SIJAGA SAMPAH dirancang sebagai sistem Smart Environment yang berfokus
pada alur inti dan dapat dibangun secara realistis menggunakan Rust dan
PostgreSQL. MVP memprioritaskan pelaporan warga, verifikasi, penugasan
petugas, pelacakan status, dan dashboard. Struktur API, database, role,
dan arsitektur disusun agar saling terintegrasi serta dapat diperluas
secara bertahap tanpa menjadikan IoT atau AI sebagai prasyarat awal.

Dokumen ini menjadi baseline product dan teknis. Sebelum coding dimulai,
tim perlu memvalidasi SOP dan kewenangan bersama Pemda, menyepakati
wilayah pilot, mengunci acceptance criteria, serta menyiapkan aset logo
resmi.
