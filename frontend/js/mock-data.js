// ========================================================================
// Mock Data Baseline for SIJAGA SAMPAH
// Synchronized with PostgreSQL Seed Data (PRD Section 9 & 10)
// ========================================================================

const DEFAULT_USERS = [
  {
    id: "40000000-0000-0000-0000-000000000001",
    full_name: "Ahmad Yusuf",
    email: "warga@sijaga.id",
    phone: "081234567890",
    role: "warga",
    role_name: "Masyarakat / Warga Pelapor"
  },
  {
    id: "40000000-0000-0000-0000-000000000002",
    full_name: "Budi Santoso",
    email: "petugas1@sijaga.id",
    phone: "081234567891",
    role: "petugas",
    role_name: "Petugas Kebersihan Lapangan"
  },
  {
    id: "40000000-0000-0000-0000-000000000003",
    full_name: "Siti Rahmawati, S.AP",
    email: "operator@sijaga.id",
    phone: "081234567892",
    role: "operator",
    role_name: "Admin Operasional Dinas LH"
  },
  {
    id: "40000000-0000-0000-0000-000000000004",
    full_name: "Drs. Hendrawan, M.Si",
    email: "pimpinan@sijaga.id",
    phone: "081234567893",
    role: "pimpinan",
    role_name: "Pimpinan / Kepala Dinas (Viewer)"
  },
  {
    id: "40000000-0000-0000-0000-000000000005",
    full_name: "Administrator SIJAGA",
    email: "admin@sijaga.id",
    phone: "081234567894",
    role: "admin",
    role_name: "Administrator Sistem"
  }
];

const DEFAULT_CATEGORIES = [
  { id: "30000000-0000-0000-0000-000000000001", code: "LIAR", name: "Sampah Liar / TPS Liar", desc: "Tumpukan sampah liar di trotoar, lahan kosong, atau tepi jalan." },
  { id: "30000000-0000-0000-0000-000000000002", code: "MELUAP", name: "TPS Meluap", desc: "Bak penampung sampah sementara yang melebihi kapasitas dan belum diangkut." },
  { id: "30000000-0000-0000-0000-000000000003", code: "DRAINASE", name: "Saluran Tersumbat Sampah", desc: "Sampah menyumbat drainase, got atau selokan yang memicu genangan air." },
  { id: "30000000-0000-0000-0000-000000000004", code: "BANGKAI", name: "Bangkai Binatang", desc: "Bangkai hewan yang mengeluarkan bau menyengat di fasilitas publik." },
  { id: "30000000-0000-0000-0000-000000000005", code: "B3_MEDIS", name: "Sampah B3 / Medis", desc: "Limbah bahan berbahaya beracun atau sisa medis liar di lingkungan." }
];

const DEFAULT_REGIONS = [
  { id: "20000000-0000-0000-0000-000000000001", code: "KEC-TKP", name: "Kecamatan Tanjung Karang Pusat" },
  { id: "20000000-0000-0000-0000-000000000002", code: "KEC-KDT", name: "Kecamatan Kedaton" },
  { id: "20000000-0000-0000-0000-000000000003", code: "KEC-SKR", name: "Kecamatan Sukarame" },
  { id: "20000000-0000-0000-0000-000000000004", code: "KEC-TBS", name: "Kecamatan Teluk Betung Selatan" },
  { id: "20000000-0000-0000-0000-000000000005", code: "KEC-RJB", name: "Kecamatan Rajabasa" }
];

const DEFAULT_REPORTS = [
  {
    id: "50000000-0000-0000-0000-000000000001",
    ticket_number: "SJG-2026-000001",
    reporter_id: "40000000-0000-0000-0000-000000000001",
    reporter_name: "Ahmad Yusuf",
    category_id: "30000000-0000-0000-0000-000000000001",
    category_name: "Sampah Liar / TPS Liar",
    region_id: "20000000-0000-0000-0000-000000000001",
    region_name: "Kecamatan Tanjung Karang Pusat",
    title: "Tumpukan sampah liar di trotoar Jl. Raden Intan",
    description: "Sampah kantong plastik menumpuk sejak kemarin malam dan menghambat pejalan kaki serta mengeluarkan bau busuk.",
    address_text: "Jl. Raden Intan No. 45, Enggal, Tanjung Karang Pusat",
    latitude: -5.418700,
    longitude: 105.259800,
    status: "assigned",
    priority: "high",
    photo_url: "https://images.unsplash.com/photo-1605600659908-0ef719419d41?w=600&auto=format&fit=crop&q=80",
    submitted_at: "2026-11-11T08:30:00Z",
    updated_at: "2026-11-12T09:15:00Z",
    history: [
      { from: null, to: "submitted", note: "Laporan pertama kali dikirim warga.", time: "2026-11-11T08:30:00Z", by: "Ahmad Yusuf" },
      { from: "submitted", to: "verified", note: "Laporan diverifikasi, lokasi di jalur protokol kota.", time: "2026-11-11T14:00:00Z", by: "Siti Rahmawati (Operator)" },
      { from: "verified", to: "assigned", note: "Petugas kebersihan Budi Santoso ditugaskan dengan armada roda 3.", time: "2026-11-12T09:15:00Z", by: "Siti Rahmawati (Operator)" }
    ],
    assignment: {
      assignee_id: "40000000-0000-0000-0000-000000000002",
      assignee_name: "Budi Santoso",
      assigned_by_name: "Siti Rahmawati (Operator)",
      instructions: "Segera bersihkan trotoar jalan sebelum jam operasional toko buka.",
      assigned_at: "2026-11-12T09:15:00Z"
    }
  },
  {
    id: "50000000-0000-0000-0000-000000000002",
    ticket_number: "SJG-2026-000002",
    reporter_id: "40000000-0000-0000-0000-000000000001",
    reporter_name: "Ahmad Yusuf",
    category_id: "30000000-0000-0000-0000-000000000002",
    category_name: "TPS Meluap",
    region_id: "20000000-0000-0000-0000-000000000002",
    region_name: "Kecamatan Kedaton",
    title: "Bak kontainer TPS Pasar Koga meluap ke badan jalan",
    description: "Kontainer TPS penuh sesak dan sampah tumpah ke badan aspal jalan raya, mengganggu laju kendaraan.",
    address_text: "Depan Pasar Koga, Jl. Teuku Umar, Kedaton",
    latitude: -5.389200,
    longitude: 105.263500,
    status: "in_progress",
    priority: "urgent",
    photo_url: "https://images.unsplash.com/photo-1530587191325-3db32d826c18?w=600&auto=format&fit=crop&q=80",
    submitted_at: "2026-11-12T07:15:00Z",
    updated_at: "2026-11-13T10:00:00Z",
    history: [
      { from: null, to: "submitted", note: "Laporan dibuat warga.", time: "2026-11-12T07:15:00Z", by: "Ahmad Yusuf" },
      { from: "submitted", to: "verified", note: "Diverifikasi dan skala prioritas dinaikkan ke Mendesak/Urgent.", time: "2026-11-12T08:00:00Z", by: "Siti Rahmawati (Operator)" },
      { from: "verified", to: "assigned", note: "Ditugaskan ke Tim Truk Arm Roll Budi Santoso.", time: "2026-11-12T08:30:00Z", by: "Siti Rahmawati (Operator)" },
      { from: "assigned", to: "in_progress", note: "Armada truk telah tiba di lokasi dan sedang memuat kontainer pengganti.", time: "2026-11-13T10:00:00Z", by: "Budi Santoso (Petugas)" }
    ],
    assignment: {
      assignee_id: "40000000-0000-0000-0000-000000000002",
      assignee_name: "Budi Santoso",
      assigned_by_name: "Siti Rahmawati (Operator)",
      instructions: "Segera kirim truk arm roll angkut kontainer penuh dan ganti kontainer bersih.",
      assigned_at: "2026-11-12T08:30:00Z"
    }
  },
  {
    id: "50000000-0000-0000-0000-000000000003",
    ticket_number: "SJG-2026-000003",
    reporter_id: "40000000-0000-0000-0000-000000000001",
    reporter_name: "Ahmad Yusuf",
    category_id: "30000000-0000-0000-0000-000000000003",
    category_name: "Saluran Tersumbat Sampah",
    region_id: "20000000-0000-0000-0000-000000000003",
    region_name: "Kecamatan Sukarame",
    title: "Drainase pemukiman tersumbat sampah plastik kresek dan ranting",
    description: "Air selokan tidak mengalir lancar dan meluap ke halaman rumah saat gerimis akibat tumpukan sampah plastik.",
    address_text: "Perumahan Sukarame Indah Blok B3 No 12, Sukarame",
    latitude: -5.385500,
    longitude: 105.302100,
    status: "submitted",
    priority: "normal",
    photo_url: "https://images.unsplash.com/photo-1618477461853-cf6ed80faba5?w=600&auto=format&fit=crop&q=80",
    submitted_at: "2026-11-13T13:45:00Z",
    updated_at: "2026-11-13T13:45:00Z",
    history: [
      { from: null, to: "submitted", note: "Laporan baru diterima oleh sistem.", time: "2026-11-13T13:45:00Z", by: "Ahmad Yusuf" }
    ],
    assignment: null
  },
  {
    id: "50000000-0000-0000-0000-000000000004",
    ticket_number: "SJG-2026-000004",
    reporter_id: "40000000-0000-0000-0000-000000000001",
    reporter_name: "Ahmad Yusuf",
    category_id: "30000000-0000-0000-0000-000000000001",
    category_name: "Sampah Liar / TPS Liar",
    region_id: "20000000-0000-0000-0000-000000000005",
    region_name: "Kecamatan Rajabasa",
    title: "Sampah buangan di lahan kosong dekat bundaran Rajabasa",
    description: "Tumpukan sampah kantong dan perabotan rusak dibuang orang tak dikenal.",
    address_text: "Jl. ZA. Pagar Alam, Rajabasa, Bandar Lampung",
    latitude: -5.372500,
    longitude: 105.242300,
    status: "resolved",
    priority: "normal",
    photo_url: "https://images.unsplash.com/photo-1532996122724-e3c354a0b15b?w=600&auto=format&fit=crop&q=80",
    completion_evidence_url: "https://images.unsplash.com/photo-1542601906990-b4d3fb778b09?w=600&auto=format&fit=crop&q=80",
    submitted_at: "2026-11-09T10:00:00Z",
    updated_at: "2026-11-11T16:00:00Z",
    history: [
      { from: null, to: "submitted", note: "Laporan dikirim warga.", time: "2026-11-09T10:00:00Z", by: "Ahmad Yusuf" },
      { from: "submitted", to: "verified", note: "Laporan diverifikasi.", time: "2026-11-09T11:00:00Z", by: "Siti Rahmawati (Operator)" },
      { from: "verified", to: "assigned", note: "Ditugaskan ke Petugas Budi.", time: "2026-11-09T13:00:00Z", by: "Siti Rahmawati (Operator)" },
      { from: "assigned", to: "in_progress", note: "Pembersihan sedang dilakukan.", time: "2026-11-11T09:00:00Z", by: "Budi Santoso (Petugas)" },
      { from: "in_progress", to: "resolved", note: "Pembersihan rampung, 1 armada bak telah mengangkut seluruh sampah ke TPA Bakung.", time: "2026-11-11T16:00:00Z", by: "Budi Santoso (Petugas)" }
    ],
    assignment: {
      assignee_id: "40000000-0000-0000-0000-000000000002",
      assignee_name: "Budi Santoso",
      assigned_by_name: "Siti Rahmawati (Operator)",
      instructions: "Bersihkan lahan kosong dan pasang peringatan larangan membuang sampah.",
      assigned_at: "2026-11-09T13:00:00Z",
      completed_at: "2026-11-11T16:00:00Z"
    }
  }
];

const DEFAULT_SCHEDULES = [
  {
    id: "70000000-0000-0000-0000-000000000001",
    region_id: "20000000-0000-0000-0000-000000000001",
    region_name: "Kecamatan Tanjung Karang Pusat",
    title: "Rute Pengangkutan Pagi Jalur Protokol Jl. Raden Intan",
    scheduled_at: "2026-11-14T06:00:00Z",
    status: "planned",
    notes: "Armada Truk Compactor 03, penyisiran sampah wadah pertokoan."
  },
  {
    id: "70000000-0000-0000-0000-000000000002",
    region_id: "20000000-0000-0000-0000-000000000002",
    region_name: "Kecamatan Kedaton",
    title: "Pengangkutan Rutin TPS Lingkungan Pemukiman Kedaton",
    scheduled_at: "2026-11-14T08:30:00Z",
    status: "planned",
    notes: "Rotasi bak TPS RW 01 sampai RW 05."
  },
  {
    id: "70000000-0000-0000-0000-000000000003",
    region_id: "20000000-0000-0000-0000-000000000003",
    region_name: "Kecamatan Sukarame",
    title: "Pembersihan & Pengangkutan Saluran Terbuka Sukarame",
    scheduled_at: "2026-11-15T07:00:00Z",
    status: "planned",
    notes: "Gotong royong gabungan bersama warga dan petugas armada."
  }
];

const DEFAULT_NOTIFICATIONS = [
  {
    id: "80000000-0000-0000-0000-000000000001",
    user_id: "40000000-0000-0000-0000-000000000001",
    title: "Laporan Sedang Ditangani",
    body: "Laporan Anda #SJG-2026-000002 saat ini sedang dalam proses pembersihan oleh petugas lapangan.",
    read: false,
    created_at: "2026-11-13T10:00:00Z"
  },
  {
    id: "80000000-0000-0000-0000-000000000002",
    user_id: "40000000-0000-0000-0000-000000000001",
    title: "Tugas Selesai",
    body: "Laporan Anda #SJG-2026-000004 telah selesai dibersihkan. Terima kasih telah menjaga kebersihan kota!",
    read: true,
    created_at: "2026-11-11T16:05:00Z"
  }
];
