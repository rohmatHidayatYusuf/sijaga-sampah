-- ========================================================================
-- SIJAGA SAMPAH - Seed Data Migration
-- Version: 0002_seed_data.sql
-- ========================================================================

-- Roles
INSERT INTO roles (id, code, name) VALUES
('10000000-0000-0000-0000-000000000001', 'warga', 'Masyarakat / Pelapor'),
('10000000-0000-0000-0000-000000000002', 'petugas', 'Petugas Kebersihan'),
('10000000-0000-0000-0000-000000000003', 'operator', 'Admin Operasional Persampahan'),
('10000000-0000-0000-0000-000000000004', 'pimpinan', 'Pimpinan / Viewer'),
('10000000-0000-0000-0000-000000000005', 'admin', 'Admin Sistem')
ON CONFLICT (code) DO NOTHING;

-- Regions (Kecamatan di Wilayah Pilot Kota Bandar Lampung)
INSERT INTO regions (id, code, name, region_type) VALUES
('20000000-0000-0000-0000-000000000001', 'KEC-TKP', 'Tanjung Karang Pusat', 'kecamatan'),
('20000000-0000-0000-0000-000000000002', 'KEC-KDT', 'Kedaton', 'kecamatan'),
('20000000-0000-0000-0000-000000000003', 'KEC-SKR', 'Sukarame', 'kecamatan'),
('20000000-0000-0000-0000-000000000004', 'KEC-TBS', 'Teluk Betung Selatan', 'kecamatan'),
('20000000-0000-0000-0000-000000000005', 'KEC-RJB', 'Rajabasa', 'kecamatan')
ON CONFLICT (code) DO NOTHING;

-- Waste Categories
INSERT INTO waste_categories (id, code, name, description) VALUES
('30000000-0000-0000-0000-000000000001', 'LIAR', 'Sampah Liar / TPS Liar', 'Tumpukan sampah ilegal di pinggir jalan, lahan kosong, atau fasilitas umum.'),
('30000000-0000-0000-0000-000000000002', 'MELUAP', 'TPS Meluap', 'Tempat penampungan sampah sementara yang melebihi kapasitas dan belum diangkut.'),
('30000000-0000-0000-0000-000000000003', 'DRAINASE', 'Saluran Tersumbat Sampah', 'Sampah yang menghambat drainase, selokan, atau kali yang berpotensi banjir.'),
('30000000-0000-0000-0000-000000000004', 'BANGKAI', 'Bangkai Binatang', 'Bangkai hewan yang menimbulkan bau menyengat di ruang publik.'),
('30000000-0000-0000-0000-000000000005', 'B3_MEDIS', 'Sampah B3 / Limbah Berbahaya', 'Limbah medis, zat kimia, atau bahan beracun berbahaya di lingkungan warga.')
ON CONFLICT (code) DO NOTHING;

-- Users (All passwords hashed with Argon2id for 'Password123!')
-- Hash: $argon2id$v=19$m=19456,t=2,p=1$ZGF0YXNlZWRzYWx0MTIz$hQ/qI3BshkS1F4tY76E272vT/vU0Kx2zZ6E25b5pMps
INSERT INTO users (id, full_name, email, phone, password_hash, is_active) VALUES
('40000000-0000-0000-0000-000000000001', 'Ahmad Yusuf', 'warga@sijaga.id', '081234567890', '$argon2id$v=19$m=19456,t=2,p=1$ZGF0YXNlZWRzYWx0MTIz$hQ/qI3BshkS1F4tY76E272vT/vU0Kx2zZ6E25b5pMps', TRUE),
('40000000-0000-0000-0000-000000000002', 'Budi Santoso', 'petugas1@sijaga.id', '081234567891', '$argon2id$v=19$m=19456,t=2,p=1$ZGF0YXNlZWRzYWx0MTIz$hQ/qI3BshkS1F4tY76E272vT/vU0Kx2zZ6E25b5pMps', TRUE),
('40000000-0000-0000-0000-000000000003', 'Siti Rahmawati', 'operator@sijaga.id', '081234567892', '$argon2id$v=19$m=19456,t=2,p=1$ZGF0YXNlZWRzYWx0MTIz$hQ/qI3BshkS1F4tY76E272vT/vU0Kx2zZ6E25b5pMps', TRUE),
('40000000-0000-0000-0000-000000000004', 'Drs. Hendrawan, M.Si', 'pimpinan@sijaga.id', '081234567893', '$argon2id$v=19$m=19456,t=2,p=1$ZGF0YXNlZWRzYWx0MTIz$hQ/qI3BshkS1F4tY76E272vT/vU0Kx2zZ6E25b5pMps', TRUE),
('40000000-0000-0000-0000-000000000005', 'Administrator SIJAGA', 'admin@sijaga.id', '081234567894', '$argon2id$v=19$m=19456,t=2,p=1$ZGF0YXNlZWRzYWx0MTIz$hQ/qI3BshkS1F4tY76E272vT/vU0Kx2zZ6E25b5pMps', TRUE)
ON CONFLICT (email) DO NOTHING;

-- User Roles Mapping
INSERT INTO user_roles (user_id, role_id) VALUES
('40000000-0000-0000-0000-000000000001', '10000000-0000-0000-0000-000000000001'), -- Warga
('40000000-0000-0000-0000-000000000002', '10000000-0000-0000-0000-000000000002'), -- Petugas
('40000000-0000-0000-0000-000000000003', '10000000-0000-0000-0000-000000000003'), -- Operator
('40000000-0000-0000-0000-000000000004', '10000000-0000-0000-0000-000000000004'), -- Pimpinan
('40000000-0000-0000-0000-000000000005', '10000000-0000-0000-0000-000000000005')  -- Admin
ON CONFLICT DO NOTHING;

-- Initial Reports
INSERT INTO reports (
    id, ticket_number, reporter_id, category_id, region_id,
    title, description, address_text, latitude, longitude,
    status, priority, submitted_at, updated_at
) VALUES
(
    '50000000-0000-0000-0000-000000000001',
    'SJG-2026-000001',
    '40000000-0000-0000-0000-000000000001',
    '30000000-0000-0000-0000-000000000001',
    '20000000-0000-0000-0000-000000000001',
    'Tumpukan sampah liar di trotoar Jl. Raden Intan',
    'Sampah menumpuk sejak kemarin malam dan menghambat pejalan kaki serta mengeluarkan bau tidak sedap.',
    'Jl. Raden Intan No. 45, Enggal, Tanjung Karang Pusat',
    -5.418700,
    105.259800,
    'assigned',
    'high',
    now() - INTERVAL '2 days',
    now() - INTERVAL '1 day'
),
(
    '50000000-0000-0000-0000-000000000002',
    'SJG-2026-000002',
    '40000000-0000-0000-0000-000000000001',
    '30000000-0000-0000-0000-000000000002',
    '20000000-0000-0000-0000-000000000002',
    'Bak kontainer TPS Pasar Koga meluap ke badan jalan',
    'Kontainer sampah penuh dan berserakan ke pinggir jalan raya, lalu lintas tersendat.',
    'Depan Pasar Koga, Jl. Teuku Umar, Kedaton',
    -5.389200,
    105.263500,
    'in_progress',
    'urgent',
    now() - INTERVAL '1 day',
    now() - INTERVAL '4 hours'
),
(
    '50000000-0000-0000-0000-000000000003',
    'SJG-2026-000003',
    '40000000-0000-0000-0000-000000000001',
    '30000000-0000-0000-0000-000000000003',
    '20000000-0000-0000-0000-000000000003',
    'Drainase perumahan tertutup tumpukan plastik dan ranting',
    'Air selokan meluap saat hujan kecil karena tersumbat tumpukan sampah kresek dan ranting.',
    'Perumahan Sukarame Indah Blok B3, Sukarame',
    -5.385500,
    105.302100,
    'submitted',
    'normal',
    now() - INTERVAL '2 hours',
    now() - INTERVAL '2 hours'
),
(
    '50000000-0000-0000-0000-000000000004',
    'SJG-2026-000004',
    '40000000-0000-0000-0000-000000000001',
    '30000000-0000-0000-0000-000000000001',
    '20000000-0000-0000-0000-000000000005',
    'Sampah buangan di lahan kosong dekat bundaran',
    'Ada tumpukan sampah plastik dan kasur bekas dibuang sembarangan di tanah kosong.',
    'Jl. ZA. Pagar Alam, Rajabasa',
    -5.372500,
    105.242300,
    'resolved',
    'normal',
    now() - INTERVAL '4 days',
    now() - INTERVAL '1 day'
)
ON CONFLICT (ticket_number) DO NOTHING;

-- Initial Status History
INSERT INTO report_status_history (report_id, changed_by, from_status, to_status, note, created_at) VALUES
('50000000-0000-0000-0000-000000000001', '40000000-0000-0000-0000-000000000001', NULL, 'submitted', 'Laporan pertama kali dibuat oleh warga.', now() - INTERVAL '2 days'),
('50000000-0000-0000-0000-000000000001', '40000000-0000-0000-0000-000000000003', 'submitted', 'verified', 'Laporan valid, lokasi terkonfirmasi di Jalan Raden Intan.', now() - INTERVAL '1 day 12 hours'),
('50000000-0000-0000-0000-000000000001', '40000000-0000-0000-0000-000000000003', 'verified', 'assigned', 'Ditugaskan kepada Petugas Budi Santoso armada roda tiga.', now() - INTERVAL '1 day'),

('50000000-0000-0000-0000-000000000002', '40000000-0000-0000-0000-000000000001', NULL, 'submitted', 'Laporan dibuat oleh warga.', now() - INTERVAL '1 day'),
('50000000-0000-0000-0000-000000000002', '40000000-0000-0000-0000-000000000003', 'submitted', 'verified', 'Prioritas ditingkatkan ke Urgent karena mengganggu jalan utama.', now() - INTERVAL '20 hours'),
('50000000-0000-0000-0000-000000000002', '40000000-0000-0000-0000-000000000003', 'verified', 'assigned', 'Ditugaskan ke Petugas Budi Santoso (Armada Truk Pengangkut TPS).', now() - INTERVAL '18 hours'),
('50000000-0000-0000-0000-000000000002', '40000000-0000-0000-0000-000000000002', 'assigned', 'in_progress', 'Petugas sedang dalam perjalanan dan proses pengangkutan.', now() - INTERVAL '4 hours'),

('50000000-0000-0000-0000-000000000003', '40000000-0000-0000-0000-000000000001', NULL, 'submitted', 'Laporan dikirimkan warga.', now() - INTERVAL '2 hours'),

('50000000-0000-0000-0000-000000000004', '40000000-0000-0000-0000-000000000001', NULL, 'submitted', 'Laporan dibuat.', now() - INTERVAL '4 days'),
('50000000-0000-0000-0000-000000000004', '40000000-0000-0000-0000-000000000003', 'submitted', 'assigned', 'Ditugaskan.', now() - INTERVAL '3 days'),
('50000000-0000-0000-0000-000000000004', '40000000-0000-0000-0000-000000000002', 'assigned', 'resolved', 'Pembersihan tuntas, 1 truk sampah telah diangkut ke TPA.', now() - INTERVAL '1 day')
ON CONFLICT DO NOTHING;

-- Initial Assignments
INSERT INTO assignments (id, report_id, assignee_id, assigned_by, instructions, assigned_at, accepted_at, is_active) VALUES
(
    '60000000-0000-0000-0000-000000000001',
    '50000000-0000-0000-0000-000000000001',
    '40000000-0000-0000-0000-000000000002',
    '40000000-0000-0000-0000-000000000003',
    'Harap bersihkan trotoar dan angkut kantong sampah sebelum jam 12 siang.',
    now() - INTERVAL '1 day',
    now() - INTERVAL '20 hours',
    TRUE
),
(
    '60000000-0000-0000-0000-000000000002',
    '50000000-0000-0000-0000-000000000002',
    '40000000-0000-0000-0000-000000000002',
    '40000000-0000-0000-0000-000000000003',
    'Segera bawa armada truk arm roll untuk ganti kontainer yang meluap di Pasar Koga.',
    now() - INTERVAL '18 hours',
    now() - INTERVAL '17 hours',
    TRUE
)
ON CONFLICT DO NOTHING;

-- Initial Collection Schedules
INSERT INTO collection_schedules (id, region_id, responsible_user_id, title, scheduled_at, status, notes, created_by) VALUES
('70000000-0000-0000-0000-000000000001', '20000000-0000-0000-0000-000000000001', '40000000-0000-0000-0000-000000000002', 'Rute Pagi Protokol Jl. Raden Intan', now() + INTERVAL '1 day', 'planned', 'Pengangkutan harian rutin jalur utama.', '40000000-0000-0000-0000-000000000003'),
('70000000-0000-0000-0000-000000000002', '20000000-0000-0000-0000-000000000002', '40000000-0000-0000-0000-000000000002', 'Pengangkutan TPS Pemukiman Kedaton', now() + INTERVAL '2 days', 'planned', 'Rotasi TPS RW 01 - RW 05.', '40000000-0000-0000-0000-000000000003'),
('70000000-0000-0000-0000-000000000003', '20000000-0000-0000-0000-000000000003', NULL, 'Pembersihan Jalur Bypass Sukarame', now() + INTERVAL '3 days', 'planned', 'Pembersihan bahu jalan raya.', '40000000-0000-0000-0000-000000000003')
ON CONFLICT DO NOTHING;

-- Initial Notifications
INSERT INTO notifications (id, user_id, title, body, target_type, target_id, read_at) VALUES
('80000000-0000-0000-0000-000000000001', '40000000-0000-0000-0000-000000000001', 'Laporan Diverifikasi', 'Laporan Anda #SJG-2026-000001 telah diverifikasi dan petugas telah ditugaskan.', 'report', '50000000-0000-0000-0000-000000000001', now()),
('80000000-0000-0000-0000-000000000002', '40000000-0000-0000-0000-000000000002', 'Tugas Baru Diberikan', 'Anda ditugaskan menangani TPS meluap di Pasar Koga #SJG-2026-000002.', 'assignment', '60000000-0000-0000-0000-000000000002', NULL)
ON CONFLICT DO NOTHING;
