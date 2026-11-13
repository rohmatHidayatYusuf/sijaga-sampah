// ========================================================================
// API Client & Storage Engine for SIJAGA SAMPAH
// Enforces PRD RBAC Security Rules and Data Validation
// ========================================================================

class ApiClient {
  constructor() {
    this.baseUrl = (typeof API_BASE_URL !== "undefined") ? API_BASE_URL : "http://localhost:8080/api/v1";
    this.token = localStorage.getItem("sijaga_token") || null;
    this.initLocalStorage();
  }

  initLocalStorage() {
    if (!localStorage.getItem("sijaga_reports")) {
      localStorage.setItem("sijaga_reports", JSON.stringify(DEFAULT_REPORTS));
    }
    if (!localStorage.getItem("sijaga_schedules")) {
      localStorage.setItem("sijaga_schedules", JSON.stringify(DEFAULT_SCHEDULES));
    }
    if (!localStorage.getItem("sijaga_notifications")) {
      localStorage.setItem("sijaga_notifications", JSON.stringify(DEFAULT_NOTIFICATIONS));
    }
    if (!localStorage.getItem("sijaga_users")) {
      localStorage.setItem("sijaga_users", JSON.stringify(DEFAULT_USERS));
    }
    if (!localStorage.getItem("sijaga_current_user")) {
      localStorage.setItem("sijaga_current_user", JSON.stringify(DEFAULT_USERS[0])); // Default as Warga
    }
  }

  getCurrentUser() {
    try {
      return JSON.parse(localStorage.getItem("sijaga_current_user")) || DEFAULT_USERS[0];
    } catch {
      return DEFAULT_USERS[0];
    }
  }

  setCurrentUser(user) {
    localStorage.setItem("sijaga_current_user", JSON.stringify(user));
  }

  // --- Reports ---
  getReports(filter = {}) {
    const reports = JSON.parse(localStorage.getItem("sijaga_reports")) || [];
    const currentUser = this.getCurrentUser();

    return reports.filter(r => {
      // Warga only views their own reports
      if (currentUser.role === 'warga' && r.reporter_id !== currentUser.id) {
        return false;
      }
      // Petugas only views reports assigned to them if on their tasks view or filtered
      if (currentUser.role === 'petugas') {
        const isAssigned = r.assignment && r.assignment.assignee_id === currentUser.id;
        if (!isAssigned && filter.assignedOnly) {
          return false;
        }
      }
      if (filter.status && filter.status !== 'all' && r.status !== filter.status) {
        return false;
      }
      if (filter.category_id && filter.category_id !== 'all' && r.category_id !== filter.category_id) {
        return false;
      }
      if (filter.region_id && filter.region_id !== 'all' && r.region_id !== filter.region_id) {
        return false;
      }
      if (filter.search) {
        const query = filter.search.toLowerCase();
        const matchTitle = r.title.toLowerCase().includes(query);
        const matchTicket = r.ticket_number.toLowerCase().includes(query);
        const matchAddress = (r.address_text || '').toLowerCase().includes(query);
        if (!matchTitle && !matchTicket && !matchAddress) return false;
      }
      return true;
    });
  }

  getReportById(id) {
    const reports = JSON.parse(localStorage.getItem("sijaga_reports")) || [];
    return reports.find(r => r.id === id);
  }

  createReport(data) {
    const currentUser = this.getCurrentUser();
    
    // RBAC: Pimpinan cannot create operational reports
    if (currentUser.role === 'pimpinan') {
      throw new Error("Akses ditolak: Pimpinan hanya memiliki izin baca (view-only).");
    }

    // Input Validation (PRD Section 13.3)
    if (!data.title || !data.title.trim()) {
      throw new Error("Judul laporan wajib diisi.");
    }
    if (!data.description || !data.description.trim()) {
      throw new Error("Deskripsi masalah laporan wajib diisi.");
    }
    if (data.latitude !== undefined && data.latitude !== null && !isNaN(data.latitude)) {
      if (data.latitude < -90.0 || data.latitude > 90.0) {
        throw new Error("Koordinat Latitude tidak valid (harus berada di rentang -90 hingga 90).");
      }
    }
    if (data.longitude !== undefined && data.longitude !== null && !isNaN(data.longitude)) {
      if (data.longitude < -180.0 || data.longitude > 180.0) {
        throw new Error("Koordinat Longitude tidak valid (harus berada di rentang -180 hingga 180).");
      }
    }

    const reports = JSON.parse(localStorage.getItem("sijaga_reports")) || [];
    const categories = DEFAULT_CATEGORIES;
    const regions = DEFAULT_REGIONS;

    const cat = categories.find(c => c.id === data.category_id) || categories[0];
    const reg = regions.find(rg => rg.id === data.region_id) || regions[0];

    const year = new Date().getFullYear();
    const count = reports.length + 1;
    const ticketNumber = `SJG-${year}-${String(count).padStart(6, '0')}`;

    const newReport = {
      id: crypto.randomUUID ? crypto.randomUUID() : 'rep-' + Date.now(),
      ticket_number: ticketNumber,
      reporter_id: currentUser.id,
      reporter_name: currentUser.full_name,
      category_id: cat.id,
      category_name: cat.name,
      region_id: reg.id,
      region_name: reg.name,
      title: data.title.trim(),
      description: data.description.trim(),
      address_text: data.address_text ? data.address_text.trim() : null,
      latitude: data.latitude,
      longitude: data.longitude,
      status: "submitted",
      priority: "normal",
      photo_url: data.photo_url || "https://images.unsplash.com/photo-1605600659908-0ef719419d41?w=600&auto=format&fit=crop&q=80",
      submitted_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
      history: [
        {
          from: null,
          to: "submitted",
          note: "Laporan berhasil dibuat dan dikirim ke sistem.",
          time: new Date().toISOString(),
          by: currentUser.full_name
        }
      ],
      assignment: null
    };

    reports.unshift(newReport);
    localStorage.setItem("sijaga_reports", JSON.stringify(reports));

    // Send notification
    this.addNotification({
      user_id: currentUser.id,
      title: "Laporan Diterima",
      body: `Laporan Anda dengan nomor tiket #${ticketNumber} berhasil disimpan dan masuk ke antrean verifikasi.`
    });

    return newReport;
  }

  verifyReport(id, priority = "normal", note = "") {
    const currentUser = this.getCurrentUser();
    // RBAC Security Check
    if (!['operator', 'admin'].includes(currentUser.role)) {
      throw new Error("Akses ditolak: Hanya Operator atau Admin yang berhak memverifikasi laporan.");
    }

    const reports = JSON.parse(localStorage.getItem("sijaga_reports")) || [];
    const report = reports.find(r => r.id === id);

    if (report) {
      const prevStatus = report.status;
      report.status = "verified";
      report.priority = priority;
      report.updated_at = new Date().toISOString();
      report.history.push({
        from: prevStatus,
        to: "verified",
        note: note || "Laporan diverifikasi oleh Admin Operasional.",
        time: new Date().toISOString(),
        by: currentUser.full_name
      });
      localStorage.setItem("sijaga_reports", JSON.stringify(reports));

      this.addNotification({
        user_id: report.reporter_id,
        title: "Laporan Diverifikasi",
        body: `Laporan #${report.ticket_number} telah diverifikasi dan masuk proses penugasan.`
      });
    }
    return report;
  }

  rejectReport(id, reason) {
    const currentUser = this.getCurrentUser();
    // RBAC Security Check
    if (!['operator', 'admin'].includes(currentUser.role)) {
      throw new Error("Akses ditolak: Hanya Operator atau Admin yang berhak menolak laporan.");
    }

    if (!reason || !reason.trim()) {
      throw new Error("Alasan penolakan laporan wajib diisi secara transparan.");
    }

    const reports = JSON.parse(localStorage.getItem("sijaga_reports")) || [];
    const report = reports.find(r => r.id === id);

    if (report) {
      const prevStatus = report.status;
      report.status = "rejected";
      report.rejection_reason = reason.trim();
      report.updated_at = new Date().toISOString();
      report.history.push({
        from: prevStatus,
        to: "rejected",
        note: `Alasan penolakan: ${reason.trim()}`,
        time: new Date().toISOString(),
        by: currentUser.full_name
      });
      localStorage.setItem("sijaga_reports", JSON.stringify(reports));

      this.addNotification({
        user_id: report.reporter_id,
        title: "Laporan Ditolak",
        body: `Laporan #${report.ticket_number} ditolak: ${reason.trim()}`
      });
    }
    return report;
  }

  assignReport(id, assigneeId, instructions, priority) {
    const currentUser = this.getCurrentUser();
    // RBAC Security Check
    if (!['operator', 'admin'].includes(currentUser.role)) {
      throw new Error("Akses ditolak: Hanya Operator atau Admin yang berhak menugaskan petugas.");
    }

    const reports = JSON.parse(localStorage.getItem("sijaga_reports")) || [];
    const report = reports.find(r => r.id === id);
    const users = JSON.parse(localStorage.getItem("sijaga_users")) || DEFAULT_USERS;
    const assignee = users.find(u => u.id === assigneeId) || users[1];

    if (report) {
      const prevStatus = report.status;
      report.status = "assigned";
      if (priority) report.priority = priority;
      report.updated_at = new Date().toISOString();
      report.assignment = {
        assignee_id: assignee.id,
        assignee_name: assignee.full_name,
        assigned_by_name: currentUser.full_name,
        instructions: instructions ? instructions.trim() : "Harap tangani sesuai SOP kebersihan.",
        assigned_at: new Date().toISOString()
      };
      report.history.push({
        from: prevStatus,
        to: "assigned",
        note: `Ditugaskan kepada ${assignee.full_name}. Instruksi: ${report.assignment.instructions}`,
        time: new Date().toISOString(),
        by: currentUser.full_name
      });
      localStorage.setItem("sijaga_reports", JSON.stringify(reports));

      // Notify officer
      this.addNotification({
        user_id: assignee.id,
        title: "Penugasan Baru",
        body: `Anda ditugaskan menangani laporan #${report.ticket_number} di ${report.address_text || report.title}.`
      });
    }
    return report;
  }

  closeReport(id, note = "") {
    const currentUser = this.getCurrentUser();
    // RBAC Security Check: Only Operator/Admin can close a report
    if (!['operator', 'admin'].includes(currentUser.role)) {
      throw new Error("Akses ditolak: Hanya Operator atau Admin yang berhak menutup (close) laporan.");
    }

    const reports = JSON.parse(localStorage.getItem("sijaga_reports")) || [];
    const report = reports.find(r => r.id === id);

    if (report) {
      if (report.status !== 'resolved') {
        throw new Error("Laporan hanya dapat ditutup (closed) jika telah berstatus selesai (resolved).");
      }
      const prevStatus = report.status;
      report.status = "closed";
      report.updated_at = new Date().toISOString();
      report.history.push({
        from: prevStatus,
        to: "closed",
        note: note.trim() || "Laporan telah ditinjau dan dinyatakan resmi ditutup (closed) oleh Operator.",
        time: new Date().toISOString(),
        by: currentUser.full_name
      });
      localStorage.setItem("sijaga_reports", JSON.stringify(reports));

      this.addNotification({
        user_id: report.reporter_id,
        title: "Laporan Resmi Ditutup",
        body: `Laporan #${report.ticket_number} telah diverifikasi tuntas dan ditutup oleh Dinas Lingkungan Hidup.`
      });
    }
    return report;
  }

  reopenReport(id, reason = "") {
    const currentUser = this.getCurrentUser();
    const reports = JSON.parse(localStorage.getItem("sijaga_reports")) || [];
    const report = reports.find(r => r.id === id);

    if (!report) throw new Error("Laporan tidak ditemukan.");

    // RBAC Security Check: Only Operator/Admin or the original reporter can reopen
    if (!['operator', 'admin'].includes(currentUser.role) && report.reporter_id !== currentUser.id) {
      throw new Error("Akses ditolak: Anda tidak berhak membuka kembali laporan ini.");
    }

    if (!['resolved', 'closed', 'rejected'].includes(report.status)) {
      throw new Error("Hanya laporan dengan status resolved, closed, atau rejected yang dapat dibuka kembali.");
    }

    const prevStatus = report.status;
    report.status = "reopened";
    report.updated_at = new Date().toISOString();
    report.history.push({
      from: prevStatus,
      to: "reopened",
      note: reason.trim() || "Laporan dibuka kembali untuk tindak lanjut ulang.",
      time: new Date().toISOString(),
      by: currentUser.full_name
    });
    localStorage.setItem("sijaga_reports", JSON.stringify(reports));

    this.addNotification({
      user_id: report.reporter_id,
      title: "Laporan Dibuka Kembali",
      body: `Laporan #${report.ticket_number} dibuka kembali: ${reason.trim() || 'Tindak lanjut ulang'}.`
    });

    return report;
  }

  // --- Tasks (Petugas) ---
  getMyTasks() {
    const reports = JSON.parse(localStorage.getItem("sijaga_reports")) || [];
    const currentUser = this.getCurrentUser();

    // Petugas only sees tasks assigned to them; operators see all
    if (currentUser.role === 'petugas') {
      return reports.filter(r => r.assignment && r.assignment.assignee_id === currentUser.id);
    } else if (['operator', 'admin'].includes(currentUser.role)) {
      return reports.filter(r => r.assignment !== null);
    }
    return [];
  }

  acceptTask(reportId) {
    const currentUser = this.getCurrentUser();
    if (!['petugas', 'operator', 'admin'].includes(currentUser.role)) {
      throw new Error("Akses ditolak: Hanya Petugas terkait atau Operator yang dapat menerima tugas.");
    }

    const reports = JSON.parse(localStorage.getItem("sijaga_reports")) || [];
    const report = reports.find(r => r.id === reportId);

    if (report) {
      if (currentUser.role === 'petugas' && report.assignment && report.assignment.assignee_id !== currentUser.id) {
        throw new Error("Akses ditolak: Anda bukan petugas yang ditugaskan untuk laporan ini.");
      }
      const prevStatus = report.status;
      report.status = "in_progress";
      report.updated_at = new Date().toISOString();
      if (report.assignment) {
        report.assignment.accepted_at = new Date().toISOString();
      }
      report.history.push({
        from: prevStatus,
        to: "in_progress",
        note: "Tugas diterima. Petugas sedang bekerja di lokasi.",
        time: new Date().toISOString(),
        by: currentUser.full_name
      });
      localStorage.setItem("sijaga_reports", JSON.stringify(reports));
    }
    return report;
  }

  completeTask(reportId, notes, evidenceUrl) {
    const currentUser = this.getCurrentUser();
    if (!['petugas', 'operator', 'admin'].includes(currentUser.role)) {
      throw new Error("Akses ditolak: Hanya Petugas terkait atau Operator yang dapat menyelesaikan tugas.");
    }

    const reports = JSON.parse(localStorage.getItem("sijaga_reports")) || [];
    const report = reports.find(r => r.id === reportId);

    if (report) {
      if (currentUser.role === 'petugas' && report.assignment && report.assignment.assignee_id !== currentUser.id) {
        throw new Error("Akses ditolak: Anda bukan petugas yang ditugaskan untuk laporan ini.");
      }
      const prevStatus = report.status;
      report.status = "resolved";
      report.completion_evidence_url = evidenceUrl || "https://images.unsplash.com/photo-1542601906990-b4d3fb778b09?w=600&auto=format&fit=crop&q=80";
      report.updated_at = new Date().toISOString();
      if (report.assignment) {
        report.assignment.completed_at = new Date().toISOString();
      }
      report.history.push({
        from: prevStatus,
        to: "resolved",
        note: `Pekerjaan selesai. Catatan petugas: ${notes || 'Pembersihan tuntas.'}`,
        time: new Date().toISOString(),
        by: currentUser.full_name
      });
      localStorage.setItem("sijaga_reports", JSON.stringify(reports));

      // Notify citizen
      this.addNotification({
        user_id: report.reporter_id,
        title: "Laporan Telah Selesai Ditangani",
        body: `Laporan Anda #${report.ticket_number} telah dibersihkan oleh petugas kebersihan. Terima kasih!`
      });
    }
    return report;
  }

  // --- Dashboard Data ---
  getDashboardStats() {
    const reports = JSON.parse(localStorage.getItem("sijaga_reports")) || [];
    const total = reports.length;
    const submitted = reports.filter(r => ['submitted', 'under_review', 'verified'].includes(r.status)).length;
    const inProgress = reports.filter(r => ['assigned', 'in_progress', 'reopened'].includes(r.status)).length;
    const resolved = reports.filter(r => ['resolved', 'closed'].includes(r.status)).length;
    const rejected = reports.filter(r => r.status === 'rejected').length;
    const rate = total > 0 ? Math.round((resolved / total) * 100) : 0;

    return {
      total,
      submitted,
      inProgress,
      resolved,
      rejected,
      rate
    };
  }

  // --- Schedules ---
  getSchedules() {
    return JSON.parse(localStorage.getItem("sijaga_schedules")) || [];
  }

  createSchedule(data) {
    const currentUser = this.getCurrentUser();
    if (!['operator', 'admin'].includes(currentUser.role)) {
      throw new Error("Akses ditolak: Hanya Operator atau Admin yang berhak membuat jadwal pengangkutan.");
    }

    const schedules = JSON.parse(localStorage.getItem("sijaga_schedules")) || [];
    const regions = DEFAULT_REGIONS;
    const reg = regions.find(r => r.id === data.region_id) || regions[0];

    const newSchedule = {
      id: crypto.randomUUID ? crypto.randomUUID() : 'sch-' + Date.now(),
      region_id: reg.id,
      region_name: reg.name,
      title: data.title.trim(),
      scheduled_at: data.scheduled_at,
      status: "planned",
      notes: data.notes ? data.notes.trim() : null
    };

    schedules.push(newSchedule);
    localStorage.setItem("sijaga_schedules", JSON.stringify(schedules));
    return newSchedule;
  }

  // --- Notifications ---
  getNotifications() {
    const notifs = JSON.parse(localStorage.getItem("sijaga_notifications")) || [];
    const currentUser = this.getCurrentUser();
    return notifs.filter(n => n.user_id === currentUser.id);
  }

  addNotification(notif) {
    const notifs = JSON.parse(localStorage.getItem("sijaga_notifications")) || [];
    const newNotif = {
      id: crypto.randomUUID ? crypto.randomUUID() : 'notif-' + Date.now(),
      user_id: notif.user_id,
      title: notif.title,
      body: notif.body,
      read: false,
      created_at: new Date().toISOString()
    };
    notifs.unshift(newNotif);
    localStorage.setItem("sijaga_notifications", JSON.stringify(notifs));
  }

  markNotificationAsRead(id) {
    const notifs = JSON.parse(localStorage.getItem("sijaga_notifications")) || [];
    const notif = notifs.find(n => n.id === id);
    if (notif) {
      notif.read = true;
      localStorage.setItem("sijaga_notifications", JSON.stringify(notifs));
    }
  }

  // --- Export CSV ---
  exportCsv() {
    const currentUser = this.getCurrentUser();
    if (!['operator', 'pimpinan', 'admin'].includes(currentUser.role)) {
      throw new Error("Akses ditolak: Warga dan Petugas Lapangan tidak memiliki hak untuk mengekspor data laporan.");
    }

    const reports = JSON.parse(localStorage.getItem("sijaga_reports")) || [];
    let csv = "Nomor Tiket,Judul Laporan,Kategori,Wilayah,Status,Prioritas,Pelapor,Tanggal Lapor\n";
    reports.forEach(r => {
      const escape = (text) => `"${(text || '').replace(/"/g, '""')}"`;
      csv += `${escape(r.ticket_number)},${escape(r.title)},${escape(r.category_name)},${escape(r.region_name)},${escape(r.status)},${escape(r.priority)},${escape(r.reporter_name)},${escape(r.submitted_at)}\n`;
    });

    const blob = new Blob([csv], { type: "text/csv;charset=utf-8;" });
    const url = URL.createObjectURL(blob);
    const link = document.createElement("a");
    link.setAttribute("href", url);
    link.setAttribute("download", `laporan_sijaga_${new Date().toISOString().slice(0, 10)}.csv`);
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
  }
}

const api = new ApiClient();
