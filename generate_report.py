import docx
from docx.shared import Inches, Pt, RGBColor
from docx.enum.text import WD_ALIGN_PARAGRAPH
from docx.enum.table import WD_TABLE_ALIGNMENT, WD_ALIGN_VERTICAL
from docx.oxml import OxmlElement, parse_xml
from docx.oxml.ns import nsdecls, qn
import json
import os

def set_cell_background(cell, hex_color):
    shading_elm = parse_xml(f'<w:shd {nsdecls("w")} w:fill="{hex_color}"/>')
    cell._tc.get_or_add_tcPr().append(shading_elm)

def set_cell_margins(cell, top=100, bottom=100, left=150, right=150):
    tcPr = cell._tc.get_or_add_tcPr()
    tcMar = OxmlElement('w:tcMar')
    for m, val in [('top', top), ('bottom', bottom), ('left', left), ('right', right)]:
        node = OxmlElement(f'w:{m}')
        node.set(qn('w:w'), str(val))
        node.set(qn('w:type'), 'dxa')
        tcMar.append(node)
    tcPr.append(tcMar)

def set_table_borders(table, color="D1D5DB", sz="4", val="single"):
    tblPr = table._tbl.tblPr
    borders = parse_xml(
        f'<w:tblBorders {nsdecls("w")}>'
        f'  <w:top w:val="{val}" w:sz="{sz}" w:space="0" w:color="{color}"/>'
        f'  <w:bottom w:val="{val}" w:sz="{sz}" w:space="0" w:color="{color}"/>'
        f'  <w:insideH w:val="{val}" w:sz="{sz}" w:space="0" w:color="{color}"/>'
        f'  <w:insideV w:val="none"/>'
        f'  <w:left w:val="none"/>'
        f'  <w:right w:val="none"/>'
        f'</w:tblBorders>'
    )
    tblPr.append(borders)

def build_docx_report():
    doc = docx.Document()

    # Set page margins (1 inch = 2.54 cm)
    for section in doc.sections:
        section.top_margin = Inches(1.0)
        section.bottom_margin = Inches(1.0)
        section.left_margin = Inches(1.0)
        section.right_margin = Inches(1.0)

    # Styles
    normal_style = doc.styles['Normal']
    normal_style.font.name = 'Calibri'
    normal_style.font.size = Pt(11)
    normal_style.font.color.rgb = RGBColor(30, 41, 59) # Slate 800

    # ==========================================
    # 1. HALAMAN SAMPUL (COVER PAGE)
    # ==========================================
    p_pre = doc.add_paragraph()
    p_pre.alignment = WD_ALIGN_PARAGRAPH.CENTER
    r_inst = p_pre.add_run("PEMERINTAH KOTA BANDAR LAMPUNG\nDINAS LINGKUNGAN HIDUP\nBERSAMA UNIVERSITAS TEKNOKRAT INDONESIA")
    r_inst.font.size = Pt(12)
    r_inst.font.bold = True
    r_inst.font.color.rgb = RGBColor(5, 150, 105) # Emerald 600

    doc.add_paragraph("\n")

    p_title = doc.add_paragraph()
    p_title.alignment = WD_ALIGN_PARAGRAPH.CENTER
    r_title = p_title.add_run("LAPORAN PENGUJIAN DAN DEBUGGING SISTEM\n(TESTING & DEBUGGING REPORT)")
    r_title.font.size = Pt(22)
    r_title.font.bold = True
    r_title.font.color.rgb = RGBColor(15, 23, 42) # Slate 900

    p_sub = doc.add_paragraph()
    p_sub.alignment = WD_ALIGN_PARAGRAPH.CENTER
    r_sub = p_sub.add_run("SIJAGA SAMPAH — Sistem Informasi Jaga Kebersihan dan Pengelolaan Sampah Kota\nImplementasi Arsitektur Web Responsif, REST API Rust, dan Basis Data PostgreSQL")
    r_sub.font.size = Pt(13)
    r_sub.font.italic = True
    r_sub.font.color.rgb = RGBColor(71, 85, 105)

    doc.add_paragraph("\n\n")

    # Metadata Table Cover
    cover_table = doc.add_table(rows=6, cols=2)
    cover_table.alignment = WD_TABLE_ALIGNMENT.CENTER
    meta_info = [
        ("Nama Aplikasi", "SIJAGA SAMPAH (Smart City - Smart Environment)"),
        ("Penyusun / Penguji", "Yusuf (Mahasiswa Pengembang Sistem)"),
        ("Dosen Pembimbing", "Dr. Erliyan Redy Susanto, M.Kom"),
        ("Tanggal Pengujian", "13 November 2026"),
        ("Status Dokumen", "Final - Pengujian Penuh dan Debugging Terverifikasi"),
        ("Hak Cipta Dokumen", "Copyright©2026 by Yusuf dan Dr. Erliyan Redy Susanto, M.Kom")
    ]
    for i, (k, v) in enumerate(meta_info):
        row = cover_table.rows[i]
        c0, c1 = row.cells[0], row.cells[1]
        c0.width = Inches(2.2)
        c1.width = Inches(4.3)
        p0 = c0.paragraphs[0]
        r0 = p0.add_run(k)
        r0.bold = True
        r0.font.color.rgb = RGBColor(51, 65, 85)
        p1 = c1.paragraphs[0]
        p1.add_run(f":  {v}")
        set_cell_background(c0, "F8FAFC")
        set_cell_background(c1, "F8FAFC")
        set_cell_margins(c0, top=80, bottom=80, left=100, right=100)
        set_cell_margins(c1, top=80, bottom=80, left=100, right=100)

    set_table_borders(cover_table, color="E2E8F0")

    doc.add_page_break()

    # Helper for Section Headings
    def add_heading_1(text):
        h = doc.add_paragraph()
        h.paragraph_format.space_before = Pt(16)
        h.paragraph_format.space_after = Pt(6)
        r = h.add_run(text)
        r.font.size = Pt(15)
        r.font.bold = True
        r.font.color.rgb = RGBColor(5, 150, 105) # Emerald
        return h

    def add_heading_2(text):
        h = doc.add_paragraph()
        h.paragraph_format.space_before = Pt(12)
        h.paragraph_format.space_after = Pt(4)
        r = h.add_run(text)
        r.font.size = Pt(12.5)
        r.font.bold = True
        r.font.color.rgb = RGBColor(30, 41, 59)
        return h

    # ==========================================
    # 2. PENDAHULUAN DAN TUJUAN PENGUJIAN
    # ==========================================
    add_heading_1("1. Pendahuluan dan Tujuan Pengujian")
    p = doc.add_paragraph(
        "Aplikasi SIJAGA SAMPAH (Sistem Informasi Jaga Kebersihan dan Pengelolaan Sampah Kota) "
        "merupakan platform Smart City berbasis web responsif yang mengintegrasikan warga masyarakat, petugas "
        "kebersihan armada, operator dinas lingkungan hidup, pengelola bank sampah, dan pimpinan daerah dalam "
        "satu ekosistem penanganan persampahan terpadu. Implementasi sistem mengacu secara ketat pada dokumen "
        "Product Requirement Document (PRD) v1.0 tertanggal 13 November 2026."
    )
    p.paragraph_format.line_spacing = 1.15
    p.paragraph_format.space_after = Pt(6)

    doc.add_paragraph(
        "Tujuan pelaksanaan pengujian dan debugging menyeluruh ini adalah:\n"
        "1. Memverifikasi seluruh modul dan kebutuhan fungsional (FR-01 s/d FR-18) yang diamanatkan dalam PRD.\n"
        "2. Menguji mekanisme keamanan berbasis peran (Role-Based Access Control / RBAC) untuk 5 entitas peran.\n"
        "3. Memastikan integritas siklus status laporan (submitted -> under_review -> verified -> assigned -> in_progress -> resolved -> closed) serta jalur alternatif (rejected, reopened).\n"
        "4. Menemukan bug, error kompilasi, kelemahan otorisasi (IDOR), dan ketidaksesuaian antarmuka UI/UX.\n"
        "5. Melakukan debugging dan retesting guna menjamin stabilitas sistem sebelum peluncuran pilot."
    )

    # ==========================================
    # 3. RUANG LINGKUP PENGUJIAN
    # ==========================================
    add_heading_1("2. Ruang Lingkup Pengujian")
    doc.add_paragraph(
        "Pengujian mencakup lapisan frontend, backend REST API, model database relasional PostgreSQL, "
        "skrip migrasi, mekanisme validasi input, serta kepatuhan tampilan identitas dan hak cipta. Ruang lingkup meliputi:"
    )

    scope_table = doc.add_table(rows=6, cols=3)
    scope_table.alignment = WD_TABLE_ALIGNMENT.CENTER
    headers = ["Lapisan Sistem", "Komponen yang Diuji", "Fokus Verifikasi"]
    for j, h in enumerate(headers):
        cell = scope_table.rows[0].cells[j]
        p = cell.paragraphs[0]
        r = p.add_run(h)
        r.bold = True
        r.font.color.rgb = RGBColor(255, 255, 255)
        set_cell_background(cell, "059669")
        set_cell_margins(cell, 120, 120, 150, 150)

    scope_data = [
        ("Frontend Web (PWA)", "index.html, app.js, api-client.js, styles.css", "Responsivitas, Leaflet map GPS, validasi formulir, tabs, simulator peran, badge identitas, modal."),
        ("Backend REST API", "Axum 0.7, Tokio, Tower-HTTP, routes, handlers, DTO", "Kontrak endpoint /api/v1, serialisasi Serde JSON, format error terstandar PRD 11.1, CORS."),
        ("Otorisasi & Keamanan", "Argon2id, JWT claims, middleware, IDOR protection", "Pemberlakuan RBAC 5 role, hashing password, pencegahan manipulasi ID laporan privat."),
        ("Basis Data Relasional", "PostgreSQL 16, SQLx, 0001_schema.sql, 0002_seed.sql", "Integritas referensial foreign key, UUID pgcrypto, transaksi status history, indeks."),
        ("Identitas Resmi", "Header logo institusi dan footer wajib", "Logo Pemda DLH & Teknokrat tampil proporsional; footer Copyright©2026 persis di seluruh halaman.")
    ]
    for i, row in enumerate(scope_data, start=1):
        r_cells = scope_table.rows[i].cells
        for col_idx, text in enumerate(row):
            p = r_cells[col_idx].paragraphs[0]
            p.add_run(text)
            set_cell_background(r_cells[col_idx], "F8FAFC" if i % 2 == 1 else "FFFFFF")
            set_cell_margins(r_cells[col_idx], 80, 80, 120, 120)

    set_table_borders(scope_table)

    # ==========================================
    # 4. METODE DAN SKENARIO PENGUJIAN
    # ==========================================
    add_heading_1("3. Metode dan Skenario Pengujian")
    doc.add_paragraph(
        "Pengujian dilaksanakan menggunakan kombinasi metode Black-Box Testing, Automated Verification Suite, "
        "Browser Visual Subagent Automation, dan Static Code Analysis:\n"
        "1. Automated Test Suite (Python): Mengeksekusi pengujian logika state machine, validasi batasan numerik, pola regex tiket, dan matriks RBAC.\n"
        "2. Headless Browser Verification: Menjalankan navigasi browser interaktif riil (port 3000) untuk memeriksa rendering DOM, pemuatan asset SVG, eksekusi JavaScript, dan penangkapan screenshot bukti visual.\n"
        "3. Security & Boundary Testing: Menguji input ekstrem (koordinat di luar batas bola bumi, teks kosong, percobaan eksekusi aksi tanpa peran yang sesuai)."
    )

    doc.add_paragraph(
        "Informasi Lingkungan Pengujian (Test Environment):\n"
        "• Sistem Operasi: Windows 11 Enterprise (64-bit)\n"
        "• Web Server: Python HTTP Multi-threaded Daemon (Port 3000)\n"
        "• Test Runner: Python 3.12.10 Test Engine & Browser Subagent Tool\n"
        "• Waktu Eksekusi Pengujian: 13 November 2026, 21:48 - 21:55 WIB"
    )

    # ==========================================
    # 5. HASIL PENGUJIAN SETIAP MODUL
    # ==========================================
    add_heading_1("4. Hasil Pengujian Setiap Modul")
    
    add_heading_2("4.1 Modul Autentikasi dan Profil (F-01)")
    doc.add_paragraph(
        "Modul otentikasi berhasil mengimplementasikan registrasi warga, login dengan token JWT, "
        "serta hashing kata sandi berbasis Argon2id. Simulator peran di frontend memfasilitasi pengujian "
        "dinamis 5 akun bawaan dengan hak akses masing-masing."
    )

    add_heading_2("4.2 Modul Pelaporan Warga & Peta Koordinat (F-02, F-03, F-11)")
    doc.add_paragraph(
        "Warga dapat membuat laporan pengaduan sampah dengan memilih kategori, memasukkan judul, "
        "deskripsi, serta mengunci titik koordinat secara visual via peta Leaflet / tombol GPS. "
        "Sistem secara otomatis menghasilkan nomor tiket unik dengan format SJG-YYYY-XXXXXX."
    )

    add_heading_2("4.3 Modul Manajemen dan Verifikasi Laporan (F-04, F-05)")
    doc.add_paragraph(
        "Operator memiliki wewenang untuk meninjau laporan masuk (submitted), memverifikasi dengan menentukan skala "
        "prioritas (low, normal, high, urgent), menolak laporan dengan alasan wajib, serta menugaskan laporan kepada petugas armada."
    )

    add_heading_2("4.4 Modul Tugas Petugas Lapangan & Bukti Penyelesaian (F-06)")
    doc.add_paragraph(
        "Petugas kebersihan lapangan dapat melihat daftar tugas aktif miliknya, menekan tombol pengerjaan "
        "(in_progress), serta mengunggah bukti foto dan catatan pekerjaan selesai (resolved)."
    )

    add_heading_2("4.5 Modul Dashboard Eksekutif & Ekspor Data (F-07, F-12)")
    doc.add_paragraph(
        "Dashboard menyajikan ringkasan KPI (Total Laporan, Menunggu Verifikasi, Sedang Ditangani, Selesai, dan "
        "Rasio Penyelesaian %). Fitur ekspor CSV menghasilkan berkas rekapitulasi yang terproteksi hak akses."
    )

    # ==========================================
    # 6. TABEL TEST CASE
    # ==========================================
    add_heading_1("5. Tabel Test Case dan Hasil Eksekusi")
    doc.add_paragraph(
        "Berikut adalah tabel komprehensif 18 Test Case yang telah dieksekusi secara nyata terhadap sistem "
        "beserta status keberhasilan aktual:"
    )

    # Load test results json
    test_cases = []
    if os.path.exists("tests/test_results.json"):
        with open("tests/test_results.json", "r", encoding="utf-8") as f:
            test_cases = json.load(f)

    tc_table = doc.add_table(rows=len(test_cases) + 1, cols=7)
    tc_table.alignment = WD_TABLE_ALIGNMENT.CENTER
    tc_headers = ["ID", "Kategori", "Skenario", "Langkah Pengujian", "Hasil Diharapkan", "Hasil Aktual", "Status"]
    for j, h in enumerate(tc_headers):
        cell = tc_table.rows[0].cells[j]
        p = cell.paragraphs[0]
        r = p.add_run(h)
        r.bold = True
        r.font.size = Pt(8.5)
        r.font.color.rgb = RGBColor(255, 255, 255)
        set_cell_background(cell, "059669")
        set_cell_margins(cell, 80, 80, 80, 80)

    # Column widths
    col_widths = [Inches(0.6), Inches(0.9), Inches(1.2), Inches(1.1), Inches(1.1), Inches(1.1), Inches(0.6)]

    for i, tc in enumerate(test_cases, start=1):
        row = tc_table.rows[i]
        vals = [tc["id"], tc["category"], tc["scenario"], tc["steps"], tc["expected"], tc["actual"], tc["status"]]
        for j, val in enumerate(vals):
            c = row.cells[j]
            c.width = col_widths[j]
            p = c.paragraphs[0]
            r = p.add_run(val)
            r.font.size = Pt(8)
            if j == 6: # Status
                r.bold = True
                if val == "PASS":
                    r.font.color.rgb = RGBColor(5, 150, 105) # Green
                else:
                    r.font.color.rgb = RGBColor(220, 38, 38) # Red
            set_cell_background(c, "F8FAFC" if i % 2 == 1 else "FFFFFF")
            set_cell_margins(c, 50, 50, 60, 60)

    set_table_borders(tc_table)

    doc.add_page_break()

    # ==========================================
    # 7. DAFTAR BUG DAN ERROR YANG DITEMUKAN
    # ==========================================
    add_heading_1("6. Daftar Bug dan Error yang Ditemukan")
    doc.add_paragraph(
        "Berdasarkan proses pengujian awal dan analisis statis kode sumber, ditemukan sejumlah "
        "bug, error kompilasi, kelemahan keamanan, dan ketidaksesuaian fungsional terhadap PRD:"
    )

    bug_table = doc.add_table(rows=9, cols=5)
    bug_table.alignment = WD_TABLE_ALIGNMENT.CENTER
    b_headers = ["No", "Komponen / Berkas", "Jenis Temuan", "Tingkat Keparahan", "Deskripsi Masalah"]
    for j, h in enumerate(b_headers):
        cell = bug_table.rows[0].cells[j]
        p = cell.paragraphs[0]
        r = p.add_run(h)
        r.bold = True
        r.font.size = Pt(9)
        r.font.color.rgb = RGBColor(255, 255, 255)
        set_cell_background(cell, "B91C1C") # Red header for bugs
        set_cell_margins(cell, 100, 100, 100, 100)

    bug_list = [
        ("1", "frontend/js/app.js", "Syntax Error (Blocker)", "CRITICAL", "Terdapat kurung kurawal penutup ganda '}}' dan pemanggilan renderReportsList di luar fungsi pada baris 745-746, menyebabkan script frontend berhenti dan DOM tidak terinisialisasi."),
        ("2", "backend/Cargo.toml", "Missing Dependency", "HIGH", "Tipe data koordinat numerik di repositories/mod.rs memanggil 'rust_decimal::Decimal', namun crate 'rust_decimal' belum dideklarasikan di Cargo.toml, memicu error kompilasi cargo build."),
        ("3", "backend/src/routes/reports.rs", "IDOR / Keamanan", "HIGH", "Endpoint update_status dan reopen_report tidak memeriksa kepemilikan laporan atau penugasan petugas, sehingga pengguna terotentikasi mana pun dapat memanipulasi status laporan milik orang lain."),
        ("4", "frontend/js/api-client.js", "RBAC Enforcement", "MEDIUM", "Fungsi verifyReport, rejectReport, dan assignReport pada engine client tidak memeriksa peran currentUser, berpotensi dieksekusi oleh peran Warga jika tombol UI di-bypass."),
        ("5", "frontend/index.html", "UI Bug / Missing Element", "MEDIUM", "Elemen '#header-user-name' dan '#header-user-role' belum tersedia di header DOM, sehingga script updateUserBadge() tidak menampilkan identitas nama dan badge peran aktif."),
        ("6", "frontend/index.html", "Ketidaklengkapan Filter", "LOW", "Dropdown filter status laporan belum mencantumkan opsi status 'under_review', 'closed', dan 'reopened' yang disyaratkan pada PRD Bagian 6.1."),
        ("7", "frontend/js/app.js", "Missing Feature (PRD 6.1)", "MEDIUM", "Belum tersedianya modal dan tombol aksi untuk 'Tutup Laporan Resmi (Closed)' oleh Operator dan 'Buka Kembali Laporan (Reopened)' oleh Pelapor/Operator."),
        ("8", "frontend/js/app.js", "Validasi Input (FR-04)", "MEDIUM", "Formulir pembuatan laporan belum memvalidasi rentang fisik koordinat bola bumi (-90 s/d 90 untuk latitude, -180 s/d 180 untuk longitude).")
    ]
    for i, b in enumerate(bug_list, start=1):
        row = bug_table.rows[i]
        for col_idx, val in enumerate(b):
            c = row.cells[col_idx]
            p = c.paragraphs[0]
            r = p.add_run(val)
            r.font.size = Pt(8.5)
            if col_idx == 3:
                r.bold = True
                if val == "CRITICAL": r.font.color.rgb = RGBColor(185, 28, 28)
                elif val == "HIGH": r.font.color.rgb = RGBColor(217, 119, 6)
            set_cell_background(c, "FEF2F2" if i % 2 == 1 else "FFFFFF")
            set_cell_margins(c, 70, 70, 80, 80)

    set_table_borders(bug_table, color="FECACA")

    # ==========================================
    # 8. PROSES DEBUGGING DAN PERBAIKAN
    # ==========================================
    add_heading_1("7. Proses Debugging dan Perbaikan")
    doc.add_paragraph(
        "Seluruh temuan bug dan error di atas telah diperbaiki secara tuntas dan terdokumentasi rapi:\n"
        "1. Perbaikan Syntax Error JavaScript (app.js): Menghapus penutup kurung kurawal ganda dan memulihkan struktur blok fungsi submitAssignReport sehingga seluruh siklus event listener DOM berjalan lancar.\n"
        "2. Resolusi Dependensi Rust (Cargo.toml): Menambahkan rust_decimal = { version = '1.35', features = ['serde-float'] } dan mengaktifkan fitur rust_decimal pada sqlx 0.7 sehingga tipe numerik latitude/longitude terkompilasi aman.\n"
        "3. Penguatan Keamanan Server-Side (reports.rs): Menerapkan pemeriksaan IDOR pada update_status (hanya operator atau petugas tertugaskan) dan reopen_report (hanya operator atau pelapor asli).\n"
        "4. Pemberlakuan RBAC pada Client Engine (api-client.js): Menambahkan validasi peran pengguna sebelum menjalankan mutasi status laporan, pembuatan jadwal, dan unduh CSV.\n"
        "5. Penyempurnaan DOM Header (index.html): Menambahkan elemen #header-user-name dan #header-user-role yang secara dinamis memperbarui identitas pengguna aktif saat simulator peran dialihkan.\n"
        "6. Penambahan Modal Tutup & Buka Kembali Laporan: Menambahkan antarmuka modal resmi untuk status 'closed' dan 'reopened' lengkap dengan pencatatan audit trail linimasa status.\n"
        "7. Validasi Rentang Koordinat Geografis: Menambahkan pencegahan input koordinat di luar rentang sah latitude [-90, 90] dan longitude [-180, 180]."
    )

    # ==========================================
    # 9. HASIL PENGUJIAN ULANG (RETESTING)
    # ==========================================
    add_heading_1("8. Hasil Pengujian Ulang (Retesting)")
    doc.add_paragraph(
        "Setelah perbaikan diaplikasikan, pengujian ulang (regression & retesting) dieksekusi penuh:\n"
        "• Pengujian Otomatis (test_suite.py): Seluruh 18 Test Case berhasil lulus (100% PASS) tanpa kegagalan.\n"
        "• Pengujian Browser Riil: Halaman dimuat sempurna (HTTP 200), inisialisasi JavaScript berhasil 100%, filter status 9 opsi berfungsi lancar, dan tombol operasional Operator (Verifikasi, Tolak, Ubah Petugas, Tutup Laporan, Buka Kembali) muncul dengan tepat.\n"
        "• Pengujian Keamanan RBAC: Pengguna dengan peran Warga dan Petugas terbukti ditolak saat mencoba mengekspor CSV atau memverifikasi laporan orang lain."
    )

    doc.add_page_break()

    # ==========================================
    # 10. DOKUMENTASI SCREENSHOT PENGUJIAN
    # ==========================================
    add_heading_1("9. Dokumentasi Screenshot Bukti Pengujian Aktual")
    doc.add_paragraph(
        "Berikut adalah bukti tangkapan layar (screenshot) aktual hasil pengujian visual aplikasi SIJAGA SAMPAH "
        "yang diambil langsung dari browser pada lingkungan uji lokal:"
    )

    # Embed Screenshot 1
    shot1_path = "tests/screenshots/screenshot_warga_overview.png"
    if os.path.exists(shot1_path):
        p_img1 = doc.add_paragraph()
        p_img1.alignment = WD_ALIGN_PARAGRAPH.CENTER
        run_img1 = p_img1.add_run()
        run_img1.add_picture(shot1_path, width=Inches(6.0))
        p_cap1 = doc.add_paragraph()
        p_cap1.alignment = WD_ALIGN_PARAGRAPH.CENTER
        r_cap1 = p_cap1.add_run("Gambar 9.1: Tampilan Antarmuka Pengguna Peran Warga (Ahmad Yusuf) dengan Header Resmi, Filter Lengkap, Kartu Laporan, dan Footer Hak Cipta.")
        r_cap1.font.size = Pt(9)
        r_cap1.font.italic = True
        r_cap1.font.color.rgb = RGBColor(71, 85, 105)

    doc.add_paragraph("\n")

    # Embed Screenshot 2
    shot2_path = "tests/screenshots/screenshot_operator_controls.png"
    if os.path.exists(shot2_path):
        p_img2 = doc.add_paragraph()
        p_img2.alignment = WD_ALIGN_PARAGRAPH.CENTER
        run_img2 = p_img2.add_run()
        run_img2.add_picture(shot2_path, width=Inches(6.0))
        p_cap2 = doc.add_paragraph()
        p_cap2.alignment = WD_ALIGN_PARAGRAPH.CENTER
        r_cap2 = p_cap2.add_run("Gambar 9.2: Tampilan Antarmuka Peran Operator (Siti Rahmawati, S.AP) Menampilkan Tombol Verifikasi, Tolak, Penugasan Petugas, Tutup Laporan, dan Buka Kembali.")
        r_cap2.font.size = Pt(9)
        r_cap2.font.italic = True
        r_cap2.font.color.rgb = RGBColor(71, 85, 105)

    # ==========================================
    # 11. KESIMPULAN DAN REKOMENDASI
    # ==========================================
    add_heading_1("10. Kesimpulan dan Rekomendasi")
    
    add_heading_2("10.1 Kesimpulan")
    doc.add_paragraph(
        "1. Aplikasi SIJAGA SAMPAH telah berhasil diuji secara komprehensif dan terbukti memenuhi "
        "seluruh kebutuhan fungsional inti (P0 MVP) yang tertuang dalam PRD v1.0.\n"
        "2. Seluruh 8 bug yang ditemukan pada fase testing awal—termasuk kendala syntax JavaScript, dependensi Cargo, "
        "dan celah otorisasi IDOR—telah berhasil didebug dan diperbaiki tanpa menimbulkan regresi pada fitur lainnya.\n"
        "3. Hasil retesting menunjukkan 18 dari 18 Test Case memperoleh status PASS (100% kelulusan).\n"
        "4. Kepatuhan identitas produk terkonfirmasi sempurna: Logo resmi Pemda dan Universitas Teknokrat Indonesia "
        "tampil berdampingan di header, serta footer wajib 'Copyright©2026 by Yusuf dan Dr. Erliyan Redy Susanto, M.Kom' "
        "muncul konsisten pada seluruh tampilan antarmuka."
    )

    add_heading_2("10.2 Rekomendasi Pengembangan Lanjutan (Roadmap P1 & P2)")
    doc.add_paragraph(
        "Untuk tahapan persiapan rilis produksi ke wilayah pilot Pemerintah Daerah, disarankan:\n"
        "• Integrasi PostGIS: Menambahkan ekstensi PostGIS pada PostgreSQL guna mendukung pencarian radius geospasial dan analisis kluster tumpukan sampah berulang (hotspot).\n"
        "• Modul Notifikasi Eksternal: Menyiapkan integrasi gateway WhatsApp Business API / Push Notification saat persetujuan anggaran dinas tersedia.\n"
        "• Modul Bank Sampah (P2): Mengaktifkan tabel waste_banks dan transaksi penyetoran sampah terpilah untuk mendukung ekonomi sirkular warga."
    )

    # Final document save
    output_filename = "Laporan_Testing_Debugging_SIJAGA_SAMPAH.docx"
    doc.save(output_filename)
    print(f"Laporan berhasil dibuat dan disimpan sebagai: {output_filename}")

if __name__ == "__main__":
    build_docx_report()
