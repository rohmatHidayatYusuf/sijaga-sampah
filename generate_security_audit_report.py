import docx
from docx.shared import Inches, Pt, RGBColor
from docx.enum.text import WD_ALIGN_PARAGRAPH
from docx.enum.table import WD_TABLE_ALIGNMENT
from docx.oxml import OxmlElement, parse_xml
from docx.oxml.ns import nsdecls, qn

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

def build_audit_report():
    doc = docx.Document()

    for section in doc.sections:
        section.top_margin = Inches(1.0)
        section.bottom_margin = Inches(1.0)
        section.left_margin = Inches(1.0)
        section.right_margin = Inches(1.0)

    # Styles
    normal_style = doc.styles['Normal']
    normal_style.font.name = 'Calibri'
    normal_style.font.size = Pt(11)
    normal_style.font.color.rgb = RGBColor(30, 41, 59)

    # ==========================================
    # COVER PAGE
    # ==========================================
    p_inst = doc.add_paragraph()
    p_inst.alignment = WD_ALIGN_PARAGRAPH.CENTER
    r_inst = p_inst.add_run("DINAS LINGKUNGAN HIDUP KOTA BANDAR LAMPUNG\nUNIVERSITAS TEKNOKRAT INDONESIA — SANG JUARA")
    r_inst.font.size = Pt(11.5)
    r_inst.font.bold = True
    r_inst.font.color.rgb = RGBColor(5, 150, 105)

    doc.add_paragraph("\n")

    p_title = doc.add_paragraph()
    p_title.alignment = WD_ALIGN_PARAGRAPH.CENTER
    r_title = p_title.add_run("LAPORAN AUDIT KEAMANAN SISTEM\n(SECURITY AUDIT REPORT)")
    r_title.font.size = Pt(22)
    r_title.font.bold = True
    r_title.font.color.rgb = RGBColor(185, 28, 28) # Red for security audit

    p_sub = doc.add_paragraph()
    p_sub.alignment = WD_ALIGN_PARAGRAPH.CENTER
    r_sub = p_sub.add_run("Evaluasi Komprehensif Kerentanan Berdasarkan OWASP Top 10 dan PRD v1.0\nAplikasi SIJAGA SAMPAH (Smart Environment Kota)")
    r_sub.font.size = Pt(12)
    r_sub.font.italic = True
    r_sub.font.color.rgb = RGBColor(71, 85, 105)

    doc.add_paragraph("\n\n")

    # Meta table
    table = doc.add_table(rows=6, cols=2)
    table.alignment = WD_TABLE_ALIGNMENT.CENTER
    meta = [
        ("Aplikasi yang Diaudit", "SIJAGA SAMPAH (Rust Backend & Responsive Web PWA)"),
        ("Auditor Keamanan", "Yusuf (Security Analyst / Pengembang Sistem)"),
        ("Dosen Pembimbing", "Dr. Erliyan Redy Susanto, M.Kom"),
        ("Standar Acuan", "OWASP Top 10:2021 & PRD Keamanan Bagian 13"),
        ("Tanggal Audit", "13 November 2026"),
        ("Status Hasil Audit", "TEMUAN IDENTIFIKASI LENGKAP (Pre-Remediation)")
    ]
    for i, (k, v) in enumerate(meta):
        row = table.rows[i]
        c0, c1 = row.cells[0], row.cells[1]
        c0.width = Inches(2.2)
        c1.width = Inches(4.3)
        p0 = c0.paragraphs[0]
        r0 = p0.add_run(k)
        r0.bold = True
        p1 = c1.paragraphs[0]
        p1.add_run(f":  {v}")
        set_cell_background(c0, "FEF2F2")
        set_cell_background(c1, "FEF2F2")
        set_cell_margins(c0, 80, 80, 100, 100)
        set_cell_margins(c1, 80, 80, 100, 100)
    set_table_borders(table, color="FECACA")

    doc.add_page_break()

    def add_h1(text):
        h = doc.add_paragraph()
        h.paragraph_format.space_before = Pt(14)
        h.paragraph_format.space_after = Pt(4)
        r = h.add_run(text)
        r.font.size = Pt(14)
        r.font.bold = True
        r.font.color.rgb = RGBColor(185, 28, 28)
        return h

    def add_h2(text):
        h = doc.add_paragraph()
        h.paragraph_format.space_before = Pt(10)
        h.paragraph_format.space_after = Pt(2)
        r = h.add_run(text)
        r.font.size = Pt(12)
        r.font.bold = True
        r.font.color.rgb = RGBColor(30, 41, 59)
        return h

    # Section 1
    add_h1("1. Ringkasan Eksekutif (Executive Summary)")
    doc.add_paragraph(
        "Audit keamanan sistem dilakukan terhadap seluruh komponen aplikasi SIJAGA SAMPAH yang mencakup "
        "antarmuka frontend, arsitektur REST API Rust (Axum), basis data PostgreSQL, mekanisme otentikasi "
        "Argon2id/JWT, otorisasi RBAC, validasi input, serta kepatuhan terhadap PRD Bagian 13 (Keamanan Sistem "
        "dan Validasi Data). Audit ini mengadopsi standar kerangka kerja OWASP Top 10 (2021).\n\n"
        "Dari hasil audit kode aktual, teridentifikasi 10 temuan kerentanan keamanan yang dikelompokkan ke dalam "
        "4 tingkat risiko: 2 Critical, 3 High, 4 Medium, dan 1 Low. Seluruh temuan memerlukan tindakan mitigasi "
        "sebelum aplikasi dideploy ke lingkungan produksi Pemerintah Daerah."
    )

    # Section 2
    add_h1("2. Metodologi dan Standar Acuan Audit")
    doc.add_paragraph(
        "Metodologi pengujian keamanan memadukan Static Application Security Testing (SAST), "
        "analisis kode sumber mendalam (manual source code review), dan Dynamic Analysis pada endpoint API:\n"
        "1. OWASP Top 10 (2021): Evaluasi kategori Broken Access Control, Cryptographic Failures, Injection, "
        "Insecure Design, Security Misconfiguration, Identification and Authentication Failures, dan Security Logging Failures.\n"
        "2. CWE (Common Weakness Enumeration): Pemetaan terhadap CWE-79 (XSS), CWE-284 (Broken Access Control), "
        "CWE-1236 (CSV Injection), CWE-798 (Hardcoded Credentials), dan CWE-778 (Insufficient Logging).\n"
        "3. PRD Baseline: Verifikasi kepatuhan terhadap kontrol teknis pada PRD Bagian 4.1 (Aturan Otorisasi), "
        "Bagian 9 (DDL PostgreSQL), Bagian 11 (Kontrak REST API), dan Bagian 13 (Keamanan)."
    )

    # Section 3: Summary Table
    add_h1("3. Matriks Rekapitulasi Temuan Kerentanan")
    summary_table = doc.add_table(rows=11, cols=5)
    summary_table.alignment = WD_TABLE_ALIGNMENT.CENTER
    s_headers = ["ID", "Kategori OWASP", "Judul Kerentanan", "Tingkat Risiko", "Komponen Terdampak"]
    for j, h in enumerate(s_headers):
        c = summary_table.rows[0].cells[j]
        p = c.paragraphs[0]
        r = p.add_run(h)
        r.bold = True
        r.font.size = Pt(8.5)
        r.font.color.rgb = RGBColor(255, 255, 255)
        set_cell_background(c, "1E293B")
        set_cell_margins(c, 80, 80, 80, 80)

    findings = [
        ("SEC-01", "A02: Cryptographic Failures", "Hardcoded Fallback JWT Secret Key di Backend", "CRITICAL", "backend/src/config/mod.rs"),
        ("SEC-02", "A03: Injection (Stored XSS)", "Injeksi HTML/JavaScript pada DOM Rendering Kartu & Modal", "CRITICAL", "frontend/js/app.js"),
        ("SEC-03", "A01: Broken Access Control", "Hak Akses Petugas Lapangan Melebihi Batas pada Query Laporan", "HIGH", "backend/src/routes/reports.rs"),
        ("SEC-04", "A03: Injection (CSV Formula)", "Formula Injection (CWE-1236) pada Modul Ekspor CSV Laporan", "HIGH", "backend/src/routes/reports.rs & frontend"),
        ("SEC-05", "A09: Logging Failures", "Ketiadaan Pencatatan Audit Trail ke Tabel audit_logs PostgreSQL", "HIGH", "backend/src/routes/reports.rs & auth.rs"),
        ("SEC-06", "A05: Security Misconfig", "Konfigurasi Permissive CORS (allow_origin: Any)", "MEDIUM", "backend/src/main.rs"),
        ("SEC-07", "A05: Security Misconfig", "Ketiadaan HTTP Security Headers (CSP, X-Frame, HSTS)", "MEDIUM", "backend/src/main.rs & frontend"),
        ("SEC-08", "A04: Insecure Design", "Ketiadaan Verifikasi Magic Numbers File Upload", "MEDIUM", "frontend/js/app.js & api-client.js"),
        ("SEC-09", "A07: Auth Failures", "Kebijakan Kompleksitas Kata Sandi Lemah & Ketiadaan Rate Limit", "MEDIUM", "backend/src/routes/auth.rs"),
        ("SEC-10", "A01: Broken Access / Privacy", "Eksposur Nomor Telepon Pelapor Tanpa Masking (Data Privacy)", "LOW", "backend/src/routes/reports.rs")
    ]

    for i, row in enumerate(findings, start=1):
        r_cells = summary_table.rows[i].cells
        for col_idx, val in enumerate(row):
            c = r_cells[col_idx]
            p = c.paragraphs[0]
            r = p.add_run(val)
            r.font.size = Pt(8)
            if col_idx == 3:
                r.bold = True
                if val == "CRITICAL": r.font.color.rgb = RGBColor(220, 38, 38)
                elif val == "HIGH": r.font.color.rgb = RGBColor(234, 88, 12)
                elif val == "MEDIUM": r.font.color.rgb = RGBColor(202, 138, 4)
                else: r.font.color.rgb = RGBColor(71, 85, 105)
            set_cell_background(c, "F8FAFC" if i % 2 == 1 else "FFFFFF")
            set_cell_margins(c, 50, 50, 60, 60)

    set_table_borders(summary_table)

    doc.add_page_break()

    # Section 4: Detailed Findings
    add_h1("4. Detail Temuan, Bukti Kode, Dampak, dan Rekomendasi")

    detailed_findings = [
        {
            "id": "SEC-01",
            "title": "Hardcoded Fallback JWT Secret Key di Backend",
            "owasp": "A02:2021 - Cryptographic Failures (CWE-798: Use of Hard-coded Credentials)",
            "severity": "CRITICAL",
            "location": "backend/src/config/mod.rs (Baris 25-26)",
            "evidence": 'let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "sijaga_sampah_super_secret_jwt_key_2026_change_me".to_string());',
            "impact": "Jika variabel lingkungan JWT_SECRET tidak didefinisikan di server, sistem akan menggunakan secret default statis publik. Penyerang dapat memalsukan token JWT untuk user_id siapa pun dengan role 'admin', menyebabkan pengambilalihan akun total (Full Account Takeover).",
            "recommendation": "Hapus fallback hardcoded. Wajibkan aplikasi memverifikasi keberadaan JWT_SECRET dengan panjang minimal 32 karakter saat inisialisasi server, dan hentikan proses startup (panic/exit) jika kunci rahasia tidak aman."
        },
        {
            "id": "SEC-02",
            "title": "Stored Cross-Site Scripting (XSS) via Unsanitized Data Rendering in DOM",
            "owasp": "A03:2021 - Injection (CWE-79: Cross-site Scripting)",
            "severity": "CRITICAL",
            "location": "frontend/js/app.js (Baris 395, 398, 404, 586-590)",
            "evidence": 'HTML Template Literal: `📁 ${r.category_name}`, `📍 ${r.region_name}`, `👤 Pelapor: ${r.reporter_name}` disisipkan langsung ke innerHTML tanpa sanitasi escapeHtml().',
            "impact": "Jika pengguna mendaftar dengan nama mengandung payload script berbahaya seperti `<img src=x onerror=alert(document.cookie)>`, script akan dieksekusi di browser pengguna lain atau operator saat membuka daftar laporan, memicu pencurian token sesi dan session hijacking.",
            "recommendation": "Bungkus seluruh variabel dinamis dari pengguna atau API dengan fungsi escapeHtml() sebelum disisipkan ke template string innerHTML."
        },
        {
            "id": "SEC-03",
            "title": "Hak Akses Petugas Lapangan Melebihi Batas pada Query Laporan (Broken Access Control)",
            "owasp": "A01:2021 - Broken Access Control (CWE-284: Improper Access Control)",
            "severity": "HIGH",
            "location": "backend/src/routes/reports.rs (Baris 65)",
            "evidence": 'let is_warga = auth.has_role("warga") && !auth.has_role("operator") && !auth.has_role("admin") && !auth.has_role("pimpinan");\nJika auth adalah "petugas", is_warga bernilai false, sehingga query laporan tidak menyertakan filter WHERE r.reporter_id = auth.id.',
            "impact": "Petugas kebersihan dapat melihat seluruh laporan privat warga di kota secara bebas, melanggar PRD Bagian 4.1 yang menetapkan bahwa petugas hanya boleh mengakses laporan yang secara eksplisit ditugaskan kepadanya.",
            "recommendation": "Perketat otorisasi backend pada endpoint GET /reports. Jika peran pengguna adalah 'petugas' murni, batasi query hanya pada laporan yang memiliki penugasan aktif untuk petugas tersebut (EXISTS in assignments)."
        },
        {
            "id": "SEC-04",
            "title": "Formula Injection (CSV Injection) pada Fitur Ekspor Laporan",
            "owasp": "A03:2021 - Injection (CWE-1236: Improper Neutralization of Formula Elements in a CSV File)",
            "severity": "HIGH",
            "location": "backend/src/routes/reports.rs (Baris 495-508) & frontend/js/api-client.js",
            "evidence": 'csv_data.push_str(&format!("\\"{}...\\"", row.title.replace(\'"\', \'""\')));\nField judul laporan langsung ditulis tanpa memeriksa karakter awal formula (=, +, -, @).',
            "impact": "Penyerang dapat membuat laporan dengan judul `=cmd|'/C calc'!A0` atau formula DDE spreadsheet. Saat Operator atau Pimpinan mengunduh dan membuka berkas CSV di Microsoft Excel, perintah eksternal berbahaya dapat dieksekusi pada komputer korban.",
            "recommendation": "Lakukan sanitasi formula CSV: jika nilai diawali dengan karakter '=', '+', '-', '@', tambahkan karakter petik tunggal (`'`) di depannya untuk menonaktifkan eksekusi formula di aplikasi spreadsheet."
        },
        {
            "id": "SEC-05",
            "title": "Ketiadaan Pencatatan Audit Trail ke Tabel audit_logs PostgreSQL",
            "owasp": "A09:2021 - Security Logging and Monitoring Failures (CWE-778: Insufficient Logging)",
            "severity": "HIGH",
            "location": "backend/src/routes/reports.rs & auth.rs",
            "evidence": 'Tabel audit_logs telah dibuat di migrasi 0001_initial_schema.sql, namun tidak ada query INSERT INTO audit_logs saat operator memverifikasi, menolak, menugaskan petugas, atau saat login terjadi.',
            "impact": "Tindakan administratif sensitif (penolakan laporan, perubahan penugasan, eskalasi prioritas) tidak dapat diaudit atau ditelusuri secara forensik jika terjadi sengketa operasional atau sabotase internal.",
            "recommendation": "Implementasikan fungsi utilitas audit logger di backend untuk mencatat aktor, aksi, entity_type, entity_id, dan metadata JSONB ke tabel audit_logs pada setiap operasi sensitif."
        },
        {
            "id": "SEC-06",
            "title": "Konfigurasi Permissive CORS (allow_origin: Any)",
            "owasp": "A05:2021 - Security Misconfiguration (CWE-942: Permissive Cross-domain Policy)",
            "severity": "MEDIUM",
            "location": "backend/src/main.rs (Baris 55-58)",
            "evidence": 'let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);',
            "impact": "Domain pihak ketiga yang berbahaya dapat memicu request lintas asal dan membaca data respon jika token atau sesi diteruskan, meningkatkan risiko Cross-Site Script Inclusion dan CSRF.",
            "recommendation": "Batasi CORS allow_origin hanya pada origin resmi yang diizinkan (misalnya localhost:3000 atau domain produksi yang terkonfigurasi di environment)."
        },
        {
            "id": "SEC-07",
            "title": "Ketiadaan HTTP Security Headers (CSP, X-Frame-Options, HSTS)",
            "owasp": "A05:2021 - Security Misconfiguration (CWE-16: Configuration)",
            "severity": "MEDIUM",
            "location": "backend/src/main.rs & frontend/index.html",
            "evidence": 'Respon server tidak menyertakan Content-Security-Policy, X-Frame-Options, X-Content-Type-Options: nosniff, atau Referrer-Policy.',
            "impact": "Aplikasi rentan terhadap serangan Clickjacking (dibuka di dalam iframe pihak ketiga) serta eksploitasi MIME type sniffing.",
            "recommendation": "Tambahkan middleware security headers di Axum dan tag meta HTTP-Equiv di frontend untuk memberlakukan X-Frame-Options: DENY, X-Content-Type-Options: nosniff, dan CSP dasar."
        },
        {
            "id": "SEC-08",
            "title": "Ketiadaan Verifikasi Magic Numbers File Upload",
            "owasp": "A04:2021 - Insecure Design & A08: Software and Data Integrity Failures (CWE-434)",
            "severity": "MEDIUM",
            "location": "frontend/js/app.js (Baris 277-290)",
            "evidence": 'Verifikasi file hanya memeriksa ekstensi file di input tag dan file.size > 5MB tanpa memeriksa byte header (magic numbers).',
            "impact": "Penyerang dapat mengubah ekstensi berkas berbahaya (executable, script) menjadi .png atau .jpg dan mengunggahnya ke storage.",
            "recommendation": "Periksa magic numbers file secara biner (misal: FF D8 FF untuk JPEG, 89 50 4E 47 untuk PNG, 52 49 46 46 untuk WebP) sebelum membaca dan menyimpan data foto."
        },
        {
            "id": "SEC-09",
            "title": "Kebijakan Kompleksitas Kata Sandi Lemah & Ketiadaan Rate Limiting",
            "owasp": "A07:2021 - Identification and Authentication Failures (CWE-521 & CWE-307)",
            "severity": "MEDIUM",
            "location": "backend/src/routes/auth.rs (Baris 30-32)",
            "evidence": 'if payload.password.len() < 8 { return Err(AppError::BadRequest(...)); }\nTidak ada pengecekan kombinasi karakter huruf besar, huruf kecil, angka, atau simbol, serta ketiadaan pembatasan laju percobaan login.',
            "impact": "Akun rentan terhadap serangan brute-force kamus kata sandi sederhana seperti '12345678' atau 'password'.",
            "recommendation": "Perketat validasi kata sandi dengan regex minimal 8 karakter mengandung kombinasi huruf dan angka, serta siapkan mekanisme pembatasan percobaan login berturut-turut."
        },
        {
            "id": "SEC-10",
            "title": "Eksposur Nomor Telepon Pelapor Tanpa Masking (Data Privacy)",
            "owasp": "A01:2021 - Broken Access Control & Data Privacy Violation",
            "severity": "LOW",
            "location": "backend/src/routes/reports.rs (Baris 221) & frontend",
            "evidence": 'Field "reporter_phone": report.reporter_phone dikembalikan secara mentah (plaintext) kepada pemanggil detail laporan.',
            "impact": "Nomor kontak pribadi warga dapat dilihat oleh petugas yang tidak berkepentingan langsung, berisiko terhadap penyalahgunaan data pribadi (UU Perlindungan Data Pribadi).",
            "recommendation": "Terapkan data masking pada nomor telepon (misal: 0812****7890) kecuali bagi operator resmi yang memiliki kebutuhan kontak darurat."
        }
    ]

    for f in detailed_findings:
        add_h2(f"{f['id']}: {f['title']}")
        p_desc = doc.add_paragraph()
        r_sev = p_desc.add_run(f"Tingkat Risiko: {f['severity']}\n")
        r_sev.bold = True
        if f['severity'] == "CRITICAL": r_sev.font.color.rgb = RGBColor(220, 38, 38)
        elif f['severity'] == "HIGH": r_sev.font.color.rgb = RGBColor(234, 88, 12)
        elif f['severity'] == "MEDIUM": r_sev.font.color.rgb = RGBColor(202, 138, 4)
        else: r_sev.font.color.rgb = RGBColor(71, 85, 105)

        p_desc.add_run(f"Kategori OWASP: {f['owasp']}\n")
        p_desc.add_run(f"Lokasi Kode: {f['location']}\n\n")

        p_ev = doc.add_paragraph()
        r_ev_label = p_ev.add_run("Bukti Kode Aktual:\n")
        r_ev_label.bold = True
        r_ev_code = p_ev.add_run(f"{f['evidence']}\n\n")
        r_ev_code.font.name = "Consolas"
        r_ev_code.font.size = Pt(9.5)

        p_imp = doc.add_paragraph()
        r_imp_label = p_imp.add_run("Dampak Kerentanan:\n")
        r_imp_label.bold = True
        p_imp.add_run(f"{f['impact']}\n\n")

        p_rec = doc.add_paragraph()
        r_rec_label = p_rec.add_run("Rekomendasi Perbaikan:\n")
        r_rec_label.bold = True
        r_rec_label.font.color.rgb = RGBColor(5, 150, 105)
        p_rec.add_run(f"{f['recommendation']}\n")

        doc.add_paragraph("―" * 40)

    # Save
    out_file = "Laporan_Audit_Security_SIJAGA_SAMPAH.docx"
    doc.save(out_file)
    print(f"Laporan audit berhasil disimpan sebagai: {out_file}")

if __name__ == "__main__":
    build_audit_report()
