// ========================================================================
// Main UI Application Controller for SIJAGA SAMPAH
// ========================================================================

let overviewMap = null;
let pickerMap = null;
let pickerMarker = null;
let reportMarkers = [];

document.addEventListener("DOMContentLoaded", () => {
  initRoleSwitcher();
  initNavigation();
  initFormCategoriesAndRegions();
  initReportForm();
  renderCurrentView();
  initNotifications();
  setupEventListeners();
});

// --- Role Switcher ---
function initRoleSwitcher() {
  const roleSelect = document.getElementById("active-role-select");
  if (!roleSelect) return;

  const users = JSON.parse(localStorage.getItem("sijaga_users")) || DEFAULT_USERS;
  roleSelect.innerHTML = users
    .map(u => `<option value="${u.id}">${u.role_name} (${u.full_name})</option>`)
    .join("");

  const currentUser = api.getCurrentUser();
  roleSelect.value = currentUser.id;

  roleSelect.addEventListener("change", (e) => {
    const selected = users.find(u => u.id === e.target.value);
    if (selected) {
      api.setCurrentUser(selected);
      updateUserBadge();
      renderCurrentView();
      showToast(`Beralih peran sebagai: ${selected.role_name}`, "info");
    }
  });

  updateUserBadge();
}

function updateUserBadge() {
  const user = api.getCurrentUser();
  const nameEl = document.getElementById("header-user-name");
  const roleEl = document.getElementById("header-user-role");
  if (nameEl) nameEl.textContent = user.full_name;
  if (roleEl) roleEl.textContent = user.role.toUpperCase();

  // Control tabs visibility based on role
  document.querySelectorAll("[data-role-required]").forEach(el => {
    const allowed = el.getAttribute("data-role-required").split(",");
    if (allowed.includes(user.role) || user.role === "admin") {
      el.classList.remove("hidden");
    } else {
      el.classList.add("hidden");
    }
  });
}

// --- Navigation & Views ---
function initNavigation() {
  const navButtons = document.querySelectorAll(".nav-tab-btn");
  navButtons.forEach(btn => {
    btn.addEventListener("click", () => {
      const targetView = btn.getAttribute("data-view");
      switchView(targetView);
    });
  });
}

function switchView(viewName) {
  // Hide all view sections
  document.querySelectorAll(".view-section").forEach(sec => sec.classList.add("hidden"));
  
  // Show target section
  const target = document.getElementById(`view-${viewName}`);
  if (target) {
    target.classList.remove("hidden");
  }

  // Update active tab styles
  document.querySelectorAll(".nav-tab-btn").forEach(btn => {
    if (btn.getAttribute("data-view") === viewName) {
      btn.classList.add("text-emerald-700", "border-b-2", "border-emerald-600", "font-bold");
      btn.classList.remove("text-slate-500", "font-medium");
    } else {
      btn.classList.remove("text-emerald-700", "border-b-2", "border-emerald-600", "font-bold");
      btn.classList.add("text-slate-500", "font-medium");
    }
  });

  // Render specific content
  if (viewName === "reports") {
    renderReportsList();
  } else if (viewName === "create") {
    setTimeout(initPickerMap, 200);
  } else if (viewName === "tasks") {
    renderTasksList();
  } else if (viewName === "dashboard") {
    renderDashboardView();
    setTimeout(initOverviewMap, 250);
  } else if (viewName === "schedules") {
    renderSchedulesList();
  }
}

function renderCurrentView() {
  const user = api.getCurrentUser();
  if (user.role === "petugas") {
    switchView("tasks");
  } else if (user.role === "pimpinan" || user.role === "operator" || user.role === "admin") {
    switchView("dashboard");
  } else {
    switchView("reports");
  }
}

// --- Categories and Regions Dropdown Populate ---
function initFormCategoriesAndRegions() {
  const catSelect = document.getElementById("report-category");
  const filterCatSelect = document.getElementById("filter-category");
  const regSelect = document.getElementById("report-region");
  const filterRegSelect = document.getElementById("filter-region");
  const scheduleRegSelect = document.getElementById("schedule-region");

  if (catSelect) {
    catSelect.innerHTML = DEFAULT_CATEGORIES.map(c => `<option value="${c.id}">${c.name}</option>`).join("");
  }
  if (filterCatSelect) {
    filterCatSelect.innerHTML = `<option value="all">Semua Kategori</option>` + DEFAULT_CATEGORIES.map(c => `<option value="${c.id}">${c.name}</option>`).join("");
  }
  if (regSelect) {
    regSelect.innerHTML = DEFAULT_REGIONS.map(r => `<option value="${r.id}">${r.name}</option>`).join("");
  }
  if (filterRegSelect) {
    filterRegSelect.innerHTML = `<option value="all">Semua Wilayah</option>` + DEFAULT_REGIONS.map(r => `<option value="${r.id}">${r.name}</option>`).join("");
  }
  if (scheduleRegSelect) {
    scheduleRegSelect.innerHTML = DEFAULT_REGIONS.map(r => `<option value="${r.id}">${r.name}</option>`).join("");
  }
}

// --- Interactive Leaflet Maps ---
function initOverviewMap() {
  const mapContainer = document.getElementById("map-overview");
  if (!mapContainer) return;

  if (overviewMap) {
    overviewMap.invalidateSize();
    updateOverviewMapMarkers();
    return;
  }

  // Bandar Lampung coordinates default
  overviewMap = L.map("map-overview").setView([-5.418700, 105.259800], 13);
  L.tileLayer("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png", {
    maxZoom: 19,
    attribution: "© OpenStreetMap contributors"
  }).addTo(overviewMap);

  updateOverviewMapMarkers();
}

function updateOverviewMapMarkers() {
  if (!overviewMap) return;

  // Clear existing markers
  reportMarkers.forEach(m => overviewMap.removeLayer(m));
  reportMarkers = [];

  const reports = api.getReports();
  reports.forEach(r => {
    if (r.latitude && r.longitude) {
      const color = getStatusColor(r.status);
      const markerHtml = `<div style="background-color: ${color}; width: 22px; height: 22px; border-radius: 50%; border: 2.5px solid white; box-shadow: 0 2px 5px rgba(0,0,0,0.4); display: flex; align-items: center; justify-content: center; color: white; font-size: 10px; font-weight: bold;">!</div>`;
      const icon = L.divIcon({
        className: "custom-pin",
        html: markerHtml,
        iconSize: [22, 22],
        iconAnchor: [11, 11]
      });

      const marker = L.marker([r.latitude, r.longitude], { icon }).addTo(overviewMap);
      marker.bindPopup(`
        <div class="text-xs font-sans p-1">
          <div class="font-bold text-slate-800">${r.ticket_number}</div>
          <div class="text-emerald-700 font-semibold">${r.category_name}</div>
          <div class="text-slate-600 mt-1">${r.title}</div>
          <div class="mt-2 text-center">
            <button onclick="openReportDetailModal('${r.id}')" class="px-2 py-1 bg-emerald-600 text-white rounded text-xs font-medium hover:bg-emerald-700">Lihat Detail</button>
          </div>
        </div>
      `);
      reportMarkers.push(marker);
    }
  });
}

function initPickerMap() {
  const container = document.getElementById("map-picker");
  if (!container) return;

  if (pickerMap) {
    pickerMap.invalidateSize();
    return;
  }

  const defaultLat = -5.418700;
  const defaultLng = 105.259800;

  pickerMap = L.map("map-picker").setView([defaultLat, defaultLng], 14);
  L.tileLayer("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png", {
    maxZoom: 19,
    attribution: "© OpenStreetMap contributors"
  }).addTo(pickerMap);

  pickerMarker = L.marker([defaultLat, defaultLng], { draggable: true }).addTo(pickerMap);
  updatePickerInputs(defaultLat, defaultLng);

  pickerMap.on("click", (e) => {
    pickerMarker.setLatLng(e.latlng);
    updatePickerInputs(e.latlng.lat, e.latlng.lng);
  });

  pickerMarker.on("dragend", () => {
    const pos = pickerMarker.getLatLng();
    updatePickerInputs(pos.lat, pos.lng);
  });

  const gpsBtn = document.getElementById("btn-use-gps");
  if (gpsBtn) {
    gpsBtn.addEventListener("click", () => {
      if (navigator.geolocation) {
        showToast("Mendapatkan titik koordinat GPS...", "info");
        navigator.geolocation.getCurrentPosition(
          (pos) => {
            const lat = pos.coords.latitude;
            const lng = pos.coords.longitude;
            pickerMap.setView([lat, lng], 16);
            pickerMarker.setLatLng([lat, lng]);
            updatePickerInputs(lat, lng);
            showToast("Lokasi GPS berhasil dikunci!", "success");
          },
          (err) => {
            showToast("Gagal mengambil GPS: " + err.message + ". Menggunakan titik peta.", "warning");
          }
        );
      }
    });
  }
}

function updatePickerInputs(lat, lng) {
  const latInput = document.getElementById("report-lat");
  const lngInput = document.getElementById("report-lng");
  if (latInput) latInput.value = lat.toFixed(6);
  if (lngInput) lngInput.value = lng.toFixed(6);
}

// --- Report Form Submission ---
function initReportForm() {
  const form = document.getElementById("create-report-form");
  if (!form) return;

  // Photo input preview
  const photoInput = document.getElementById("report-photo-input");
  const photoPreview = document.getElementById("photo-preview");
  const previewImg = document.getElementById("photo-preview-img");

  let photoDataUrl = null;

  if (photoInput) {
    photoInput.addEventListener("change", (e) => {
      const file = e.target.files[0];
      if (file) {
        if (file.size > 5 * 1024 * 1024) {
          showToast("Ukuran foto maksimal 5MB.", "error");
          photoInput.value = "";
          return;
        }
        const reader = new FileReader();
        reader.onload = (re) => {
          photoDataUrl = re.target.result;
          previewImg.src = photoDataUrl;
          photoPreview.classList.remove("hidden");
        };
        reader.readAsDataURL(file);
      }
    });
  }

  form.addEventListener("submit", (e) => {
    e.preventDefault();

    const title = document.getElementById("report-title").value.trim();
    const description = document.getElementById("report-description").value.trim();
    const category_id = document.getElementById("report-category").value;
    const region_id = document.getElementById("report-region").value;
    const address_text = document.getElementById("report-address").value.trim();
    const latVal = document.getElementById("report-lat").value;
    const lngVal = document.getElementById("report-lng").value;
    const latitude = latVal ? parseFloat(latVal) : null;
    const longitude = lngVal ? parseFloat(lngVal) : null;

    if (!title || !description) {
      showToast("Judul dan deskripsi laporan wajib diisi.", "error");
      return;
    }

    if (!address_text && (latitude === null || isNaN(latitude))) {
      showToast("Tentukan lokasi pada peta atau masukkan alamat.", "error");
      return;
    }

    if (latitude !== null && !isNaN(latitude) && (latitude < -90.0 || latitude > 90.0)) {
      showToast("Rentang Latitude tidak valid (-90 s/d 90).", "error");
      return;
    }

    if (longitude !== null && !isNaN(longitude) && (longitude < -180.0 || longitude > 180.0)) {
      showToast("Rentang Longitude tidak valid (-180 s/d 180).", "error");
      return;
    }

    try {
      const created = api.createReport({
        title,
        description,
        category_id,
        region_id,
        address_text,
        latitude,
        longitude,
        photo_url: photoDataUrl
      });

      showToast(`Laporan ${created.ticket_number} berhasil dikirim!`, "success");
      form.reset();
      if (photoPreview) photoPreview.classList.add("hidden");
      photoDataUrl = null;

      switchView("reports");
    } catch (err) {
      showToast(err.message, "error");
    }
  });
}

// --- Render Reports View ---
function renderReportsList() {
  const container = document.getElementById("reports-container");
  if (!container) return;

  const statusFilter = document.getElementById("filter-status") ? document.getElementById("filter-status").value : "all";
  const catFilter = document.getElementById("filter-category") ? document.getElementById("filter-category").value : "all";
  const regFilter = document.getElementById("filter-region") ? document.getElementById("filter-region").value : "all";
  const searchFilter = document.getElementById("filter-search") ? document.getElementById("filter-search").value.trim() : "";

  const reports = api.getReports({
    status: statusFilter,
    category_id: catFilter,
    region_id: regFilter,
    search: searchFilter
  });

  const currentUser = api.getCurrentUser();
  const isOperator = ["operator", "admin"].includes(currentUser.role);

  if (reports.length === 0) {
    container.innerHTML = `
      <div class="text-center py-12 bg-white rounded-xl border border-slate-200">
        <svg class="mx-auto h-12 w-12 text-slate-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"/>
        </svg>
        <p class="mt-3 text-slate-600 font-medium">Belum ada laporan yang sesuai dengan filter.</p>
        <button onclick="switchView('create')" class="mt-4 px-4 py-2 bg-emerald-600 text-white rounded-lg text-sm font-semibold hover:bg-emerald-700 shadow-sm transition">
          + Buat Laporan Baru
        </button>
      </div>
    `;
    return;
  }

  container.innerHTML = reports.map(r => `
    <div class="bg-white rounded-xl border border-slate-200 p-5 shadow-sm hover:shadow-md transition">
      <div class="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-2 pb-3 border-b border-slate-100">
        <div>
          <span class="text-xs font-mono font-bold text-slate-500 uppercase tracking-wider">${r.ticket_number}</span>
          <h3 class="text-base font-bold text-slate-900 mt-0.5">${escapeHtml(r.title)}</h3>
        </div>
        <div class="flex items-center gap-2">
          <span class="badge-status badge-${r.status}">${formatStatusLabel(r.status)}</span>
          <span class="text-xs px-2 py-0.5 rounded font-semibold badge-priority-${r.priority}">${r.priority.toUpperCase()}</span>
        </div>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-4 gap-4 mt-4">
        ${r.photo_url ? `
          <div class="md:col-span-1">
            <img src="${r.photo_url}" alt="Foto Laporan" class="w-full h-28 object-cover rounded-lg border border-slate-100 shadow-inner">
          </div>
        ` : ''}
        <div class="${r.photo_url ? 'md:col-span-3' : 'md:col-span-4'} space-y-2">
          <p class="text-sm text-slate-600 line-clamp-2">${escapeHtml(r.description)}</p>
          <div class="flex flex-wrap items-center gap-y-1 gap-x-4 text-xs text-slate-500 pt-1">
            <span class="flex items-center gap-1 font-medium text-emerald-800 bg-emerald-50 px-2 py-0.5 rounded">
              📁 ${r.category_name}
            </span>
            <span class="flex items-center gap-1">
              📍 ${r.region_name || 'Wilayah Belum Disetel'}
            </span>
            <span class="flex items-center gap-1">
              🕒 ${formatDate(r.submitted_at)}
            </span>
            <span class="flex items-center gap-1">
              👤 Pelapor: ${r.reporter_name}
            </span>
          </div>

          ${r.address_text ? `
            <div class="text-xs text-slate-600 bg-slate-50 p-2 rounded border border-slate-100">
              <span class="font-semibold text-slate-700">Alamat:</span> ${escapeHtml(r.address_text)}
            </div>
          ` : ''}

          ${r.rejection_reason ? `
            <div class="text-xs text-rose-700 bg-rose-50 p-2 rounded border border-rose-100 font-medium">
              ⚠️ Alasan Penolakan: ${escapeHtml(r.rejection_reason)}
            </div>
          ` : ''}
        </div>
      </div>

      <div class="flex flex-wrap justify-between items-center gap-2 mt-4 pt-3 border-t border-slate-100">
        <button onclick="openReportDetailModal('${r.id}')" class="text-xs font-semibold text-emerald-700 hover:text-emerald-900 flex items-center gap-1">
          🔍 Lihat Riwayat & Detail Tiket →
        </button>

        <div class="flex items-center gap-2">
          ${isOperator && ['submitted', 'under_review'].includes(r.status) ? `
            <button onclick="openVerifyModal('${r.id}')" class="px-3 py-1.5 bg-emerald-600 hover:bg-emerald-700 text-white rounded-lg text-xs font-semibold shadow-sm">
              ✓ Verifikasi
            </button>
            <button onclick="openRejectModal('${r.id}')" class="px-3 py-1.5 bg-rose-600 hover:bg-rose-700 text-white rounded-lg text-xs font-semibold shadow-sm">
              ✕ Tolak
            </button>
          ` : ''}

          ${isOperator && ['verified', 'assigned', 'reopened'].includes(r.status) ? `
            <button onclick="openAssignModal('${r.id}')" class="px-3 py-1.5 bg-sky-600 hover:bg-sky-700 text-white rounded-lg text-xs font-semibold shadow-sm">
              👥 ${r.assignment ? 'Ubah Petugas' : 'Tugaskan Petugas'}
            </button>
          ` : ''}

          ${isOperator && r.status === 'resolved' ? `
            <button onclick="openCloseModal('${r.id}')" class="px-3 py-1.5 bg-slate-800 hover:bg-slate-900 text-white rounded-lg text-xs font-semibold shadow-sm">
              ✓ Tutup Laporan
            </button>
          ` : ''}

          ${(isOperator || r.reporter_id === currentUser.id) && ['resolved', 'closed', 'rejected'].includes(r.status) ? `
            <button onclick="openReopenModal('${r.id}')" class="px-3 py-1.5 bg-amber-600 hover:bg-amber-700 text-white rounded-lg text-xs font-semibold shadow-sm">
              🔄 Buka Kembali
            </button>
          ` : ''}
        </div>
      </div>
    </div>
  `).join("");
}

// --- Render Tasks View (Petugas Lapangan) ---
function renderTasksList() {
  const container = document.getElementById("tasks-container");
  if (!container) return;

  const tasks = api.getMyTasks();
  if (tasks.length === 0) {
    container.innerHTML = `
      <div class="text-center py-12 bg-white rounded-xl border border-slate-200">
        <svg class="mx-auto h-12 w-12 text-emerald-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
        </svg>
        <p class="mt-3 text-slate-700 font-semibold">Tidak ada tugas aktif yang ditugaskan kepada Anda saat ini.</p>
        <p class="text-xs text-slate-500 mt-1">Tugas baru dari Operator akan otomatis muncul di halaman ini.</p>
      </div>
    `;
    return;
  }

  container.innerHTML = tasks.map(t => `
    <div class="bg-white rounded-xl border border-slate-200 p-5 shadow-sm space-y-4">
      <div class="flex justify-between items-start">
        <div>
          <span class="text-xs font-mono font-bold text-slate-500">${t.ticket_number}</span>
          <h3 class="text-lg font-bold text-slate-900">${escapeHtml(t.title)}</h3>
          <p class="text-xs text-slate-500">📍 ${t.region_name} • 🕒 Ditugaskan: ${formatDate(t.assignment.assigned_at)}</p>
        </div>
        <span class="badge-status badge-${t.status}">${formatStatusLabel(t.status)}</span>
      </div>

      <div class="bg-amber-50 border border-amber-200 rounded-lg p-3 text-xs text-amber-900">
        <div class="font-bold flex items-center gap-1 text-amber-950">📋 Instruksi Kerja Operator:</div>
        <div class="mt-1">${escapeHtml(t.assignment.instructions)}</div>
      </div>

      <div class="text-xs text-slate-600 bg-slate-50 p-2.5 rounded border border-slate-100">
        <span class="font-bold">Alamat Lokasi:</span> ${escapeHtml(t.address_text || 'Sesuai titik koordinat GPS')}
      </div>

      <div class="flex flex-wrap items-center justify-between gap-3 pt-2">
        <button onclick="openReportDetailModal('${t.id}')" class="text-xs font-semibold text-slate-600 hover:text-slate-900">
          Lihat Riwayat Lengkap →
        </button>

        <div class="flex items-center gap-2">
          ${t.status === 'assigned' ? `
            <button onclick="handleAcceptTask('${t.id}')" class="px-4 py-2 bg-amber-600 hover:bg-amber-700 text-white rounded-lg text-xs font-bold shadow-sm transition">
              🚀 Mulai Dikerjakan (In Progress)
            </button>
          ` : ''}

          ${t.status === 'in_progress' ? `
            <button onclick="openCompleteTaskModal('${t.id}')" class="px-4 py-2 bg-emerald-600 hover:bg-emerald-700 text-white rounded-lg text-xs font-bold shadow-sm transition">
              ✓ Unggah Bukti & Selesaikan
            </button>
          ` : ''}

          ${t.status === 'resolved' ? `
            <span class="text-xs text-emerald-700 font-bold bg-emerald-50 px-3 py-1.5 rounded-lg border border-emerald-200">
              ✓ Pekerjaan Rampung & Terverifikasi
            </span>
          ` : ''}
        </div>
      </div>
    </div>
  `).join("");
}

// --- Render Dashboard View (Pimpinan & Operator) ---
function renderDashboardView() {
  const stats = api.getDashboardStats();
  
  const totalEl = document.getElementById("stat-total-reports");
  const submittedEl = document.getElementById("stat-submitted-reports");
  const inProgressEl = document.getElementById("stat-inprogress-reports");
  const resolvedEl = document.getElementById("stat-resolved-reports");
  const rateEl = document.getElementById("stat-resolution-rate");

  if (totalEl) totalEl.textContent = stats.total;
  if (submittedEl) submittedEl.textContent = stats.submitted;
  if (inProgressEl) inProgressEl.textContent = stats.inProgress;
  if (resolvedEl) resolvedEl.textContent = stats.resolved;
  if (rateEl) rateEl.textContent = `${stats.rate}%`;
}

// --- Render Schedules View ---
function renderSchedulesList() {
  const container = document.getElementById("schedules-container");
  if (!container) return;

  const schedules = api.getSchedules();
  container.innerHTML = schedules.map(s => `
    <div class="bg-white rounded-xl border border-slate-200 p-4 shadow-sm flex flex-col sm:flex-row justify-between items-start sm:items-center gap-3">
      <div>
        <div class="flex items-center gap-2">
          <span class="text-xs font-bold text-emerald-700 bg-emerald-50 px-2 py-0.5 rounded">
            ${s.region_name}
          </span>
          <span class="text-xs text-slate-500 font-medium">
            🕒 ${formatDate(s.scheduled_at)}
          </span>
        </div>
        <h4 class="text-base font-bold text-slate-900 mt-1">${escapeHtml(s.title)}</h4>
        <p class="text-xs text-slate-600 mt-1">${escapeHtml(s.notes || 'Pengangkutan rutin persampahan.')}</p>
      </div>
      <div class="flex items-center gap-2">
        <span class="px-2.5 py-1 bg-sky-50 text-sky-800 border border-sky-200 text-xs font-semibold rounded-full uppercase">
          ${s.status}
        </span>
      </div>
    </div>
  `).join("");
}

// --- Modals Controller ---
function openReportDetailModal(reportId) {
  const report = api.getReportById(reportId);
  if (!report) return;

  const modal = document.getElementById("report-detail-modal");
  const body = document.getElementById("report-detail-content");

  body.innerHTML = `
    <div class="space-y-5">
      <div class="flex justify-between items-start pb-3 border-b border-slate-100">
        <div>
          <span class="text-xs font-mono font-bold text-slate-500">${report.ticket_number}</span>
          <h2 class="text-xl font-bold text-slate-900 mt-0.5">${escapeHtml(report.title)}</h2>
        </div>
        <div class="text-right">
          <span class="badge-status badge-${report.status}">${formatStatusLabel(report.status)}</span>
          <div class="text-xs font-bold text-slate-500 mt-1">Prioritas: ${report.priority.toUpperCase()}</div>
        </div>
      </div>

      <div class="grid grid-cols-1 sm:grid-cols-2 gap-4 text-xs bg-slate-50 p-3 rounded-lg border border-slate-100">
        <div><span class="text-slate-500">Kategori:</span> <strong class="text-slate-800">${report.category_name}</strong></div>
        <div><span class="text-slate-500">Wilayah:</span> <strong class="text-slate-800">${report.region_name}</strong></div>
        <div><span class="text-slate-500">Pelapor:</span> <strong class="text-slate-800">${report.reporter_name}</strong></div>
        <div><span class="text-slate-500">Waktu Lapor:</span> <strong class="text-slate-800">${formatDate(report.submitted_at)}</strong></div>
        <div class="sm:col-span-2"><span class="text-slate-500">Alamat:</span> <strong class="text-slate-800">${escapeHtml(report.address_text || '-')}</strong></div>
      </div>

      <div>
        <h4 class="text-xs font-bold uppercase tracking-wider text-slate-500 mb-1">Deskripsi Masalah</h4>
        <p class="text-sm text-slate-700 bg-white p-3 rounded-lg border border-slate-200 whitespace-pre-wrap">${escapeHtml(report.description)}</p>
      </div>

      ${report.photo_url || report.completion_evidence_url ? `
        <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
          ${report.photo_url ? `
            <div>
              <h4 class="text-xs font-bold uppercase tracking-wider text-slate-500 mb-1">Foto Kondisi Awal</h4>
              <img src="${report.photo_url}" class="w-full h-44 object-cover rounded-lg border border-slate-200 shadow-sm">
            </div>
          ` : ''}
          ${report.completion_evidence_url ? `
            <div>
              <h4 class="text-xs font-bold uppercase tracking-wider text-emerald-700 mb-1">Bukti Pekerjaan Selesai</h4>
              <img src="${report.completion_evidence_url}" class="w-full h-44 object-cover rounded-lg border border-emerald-300 shadow-sm">
            </div>
          ` : ''}
        </div>
      ` : ''}

      <div>
        <h4 class="text-xs font-bold uppercase tracking-wider text-slate-500 mb-3">Linimasa Riwayat Status (Audit Trail)</h4>
        <div class="relative pl-6 space-y-4">
          ${report.history.map(h => `
            <div class="timeline-item relative">
              <div class="absolute -left-6 top-1 w-3 h-3 rounded-full bg-emerald-600 ring-4 ring-emerald-100"></div>
              <div class="text-xs font-semibold text-slate-900 flex items-center gap-2">
                <span class="badge-status badge-${h.to}">${formatStatusLabel(h.to)}</span>
                <span class="text-slate-400 font-normal">oleh ${h.by || 'Sistem'}</span>
              </div>
              <div class="text-xs text-slate-600 mt-1">${escapeHtml(h.note || '')}</div>
              <div class="text-[10px] text-slate-400 mt-0.5">${formatDate(h.time)}</div>
            </div>
          `).join("")}
        </div>
      </div>
    </div>
  `;

  modal.classList.remove("hidden");
}

function closeDetailModal() {
  document.getElementById("report-detail-modal").classList.add("hidden");
}

// --- Operator Action Modals ---
let activeTargetReportId = null;

function openVerifyModal(reportId) {
  activeTargetReportId = reportId;
  const modal = document.getElementById("verify-modal");
  modal.classList.remove("hidden");
}

function submitVerifyReport() {
  if (!activeTargetReportId) return;
  const priority = document.getElementById("verify-priority").value;
  const note = document.getElementById("verify-note").value.trim();

  try {
    api.verifyReport(activeTargetReportId, priority, note);
    showToast("Laporan berhasil diverifikasi!", "success");
    document.getElementById("verify-modal").classList.add("hidden");
    renderReportsList();
  } catch (e) {
    showToast(e.message, "error");
  }
}

function openRejectModal(reportId) {
  activeTargetReportId = reportId;
  const modal = document.getElementById("reject-modal");
  modal.classList.remove("hidden");
}

function submitRejectReport() {
  if (!activeTargetReportId) return;
  const reason = document.getElementById("reject-reason").value.trim();
  if (!reason) {
    showToast("Alasan penolakan wajib diisi.", "error");
    return;
  }

  try {
    api.rejectReport(activeTargetReportId, reason);
    showToast("Laporan telah ditolak.", "warning");
    document.getElementById("reject-modal").classList.add("hidden");
    renderReportsList();
  } catch (e) {
    showToast(e.message, "error");
  }
}

function openAssignModal(reportId) {
  activeTargetReportId = reportId;
  const modal = document.getElementById("assign-modal");
  const officerSelect = document.getElementById("assign-officer");

  const users = JSON.parse(localStorage.getItem("sijaga_users")) || DEFAULT_USERS;
  const officers = users.filter(u => u.role === "petugas");

  officerSelect.innerHTML = officers.map(o => `
    <option value="${o.id}">${o.full_name} (${o.phone || 'Petugas Kebersihan'})</option>
  `).join("");

  modal.classList.remove("hidden");
}

function submitAssignReport() {
  if (!activeTargetReportId) return;
  const assigneeId = document.getElementById("assign-officer").value;
  const instructions = document.getElementById("assign-instructions").value.trim();
  const priority = document.getElementById("assign-priority").value;

  try {
    api.assignReport(activeTargetReportId, assigneeId, instructions, priority);
    showToast("Petugas berhasil ditugaskan!", "success");
    document.getElementById("assign-modal").classList.add("hidden");
    renderReportsList();
  } catch (e) {
    showToast(e.message, "error");
  }
}

function openCloseModal(reportId) {
  activeTargetReportId = reportId;
  const modal = document.getElementById("close-modal");
  modal.classList.remove("hidden");
}

function submitCloseReport() {
  if (!activeTargetReportId) return;
  const note = document.getElementById("close-note").value.trim();
  try {
    api.closeReport(activeTargetReportId, note);
    showToast("Laporan telah resmi ditutup (closed).", "success");
    document.getElementById("close-modal").classList.add("hidden");
    renderReportsList();
  } catch (e) {
    showToast(e.message, "error");
  }
}

function openReopenModal(reportId) {
  activeTargetReportId = reportId;
  const modal = document.getElementById("reopen-modal");
  modal.classList.remove("hidden");
}

function submitReopenReport() {
  if (!activeTargetReportId) return;
  const reason = document.getElementById("reopen-reason").value.trim();
  if (!reason) {
    showToast("Alasan pembukaan kembali wajib diisi.", "error");
    return;
  }
  try {
    api.reopenReport(activeTargetReportId, reason);
    showToast("Laporan telah dibuka kembali (reopened).", "warning");
    document.getElementById("reopen-modal").classList.add("hidden");
    renderReportsList();
  } catch (e) {
    showToast(e.message, "error");
  }
}

// --- Officer Task Actions ---
function handleAcceptTask(reportId) {
  try {
    api.acceptTask(reportId);
    showToast("Tugas telah diterima dan status diubah ke In Progress.", "success");
    renderTasksList();
  } catch (e) {
    showToast(e.message, "error");
  }
}

let activeCompleteTaskId = null;
function openCompleteTaskModal(reportId) {
  activeCompleteTaskId = reportId;
  const modal = document.getElementById("complete-task-modal");
  modal.classList.remove("hidden");
}

function submitCompleteTask() {
  if (!activeCompleteTaskId) return;
  const notes = document.getElementById("task-complete-notes").value.trim();
  try {
    api.completeTask(activeCompleteTaskId, notes);
    showToast("Tugas selesai dan bukti telah dilaporkan!", "success");
    document.getElementById("complete-task-modal").classList.add("hidden");
    renderTasksList();
  } catch (e) {
    showToast(e.message, "error");
  }
}

// --- Schedule Creation ---
function initScheduleModal() {
  const btn = document.getElementById("btn-add-schedule");
  const modal = document.getElementById("create-schedule-modal");
  if (btn && modal) {
    btn.addEventListener("click", () => modal.classList.remove("hidden"));
  }

  const form = document.getElementById("create-schedule-form");
  if (form) {
    form.addEventListener("submit", (e) => {
      e.preventDefault();
      const region_id = document.getElementById("schedule-region").value;
      const title = document.getElementById("schedule-title").value.trim();
      const scheduled_at = document.getElementById("schedule-time").value;
      const notes = document.getElementById("schedule-notes").value.trim();

      if (!title || !scheduled_at) {
        showToast("Judul dan jadwal waktu wajib diisi.", "error");
        return;
      }

      api.createSchedule({ region_id, title, scheduled_at, notes });
      showToast("Jadwal pengangkutan baru berhasil disimpan!", "success");
      modal.classList.add("hidden");
      form.reset();
      renderSchedulesList();
    });
  }
}

// --- Notifications Drawer ---
function initNotifications() {
  const btn = document.getElementById("btn-notification-bell");
  const drawer = document.getElementById("notification-drawer");
  const badge = document.getElementById("notification-badge");

  if (!btn || !drawer) return;

  const updateNotifUI = () => {
    const notifs = api.getNotifications();
    const unread = notifs.filter(n => !n.read).length;

    if (badge) {
      if (unread > 0) {
        badge.textContent = unread;
        badge.classList.remove("hidden");
      } else {
        badge.classList.add("hidden");
      }
    }

    const container = document.getElementById("notification-list");
    if (container) {
      if (notifs.length === 0) {
        container.innerHTML = `<div class="p-4 text-xs text-slate-500 text-center">Belum ada notifikasi baru.</div>`;
      } else {
        container.innerHTML = notifs.map(n => `
          <div class="p-3 border-b border-slate-100 ${n.read ? 'bg-white' : 'bg-emerald-50/60'} hover:bg-slate-50 transition cursor-pointer" onclick="handleNotificationClick('${n.id}')">
            <div class="text-xs font-bold text-slate-800">${escapeHtml(n.title)}</div>
            <div class="text-xs text-slate-600 mt-0.5">${escapeHtml(n.body)}</div>
            <div class="text-[10px] text-slate-400 mt-1">${formatDate(n.created_at)}</div>
          </div>
        `).join("");
      }
    }
  };

  btn.addEventListener("click", () => {
    drawer.classList.toggle("hidden");
    updateNotifUI();
  });

  updateNotifUI();
}

function handleNotificationClick(id) {
  api.markNotificationAsRead(id);
  initNotifications();
}

// --- Event Listeners Setup ---
function setupEventListeners() {
  // Filters
  const filterInputs = ["filter-status", "filter-category", "filter-region", "filter-search"];
  filterInputs.forEach(id => {
    const el = document.getElementById(id);
    if (el) {
      el.addEventListener("input", renderReportsList);
      el.addEventListener("change", renderReportsList);
    }
  });

  // CSV Export
  const exportBtn = document.getElementById("btn-export-csv");
  if (exportBtn) {
    exportBtn.addEventListener("click", () => {
      api.exportCsv();
      showToast("File CSV berhasil diunduh!", "success");
    });
  }

  // Schedule modal init
  initScheduleModal();
}

// --- Utilities ---
function formatStatusLabel(status) {
  const map = {
    submitted: "Menunggu Verifikasi",
    under_review: "Sedang Ditinjau",
    verified: "Terverifikasi",
    assigned: "Ditugaskan",
    in_progress: "Sedang Dikerjakan",
    resolved: "Selesai (Resolved)",
    closed: "Ditutup",
    rejected: "Ditolak",
    reopened: "Dibuka Kembali"
  };
  return map[status] || status;
}

function getStatusColor(status) {
  const map = {
    submitted: "#f59e0b",
    under_review: "#6366f1",
    verified: "#0284c7",
    assigned: "#9333ea",
    in_progress: "#ea580c",
    resolved: "#10b981",
    closed: "#059669",
    rejected: "#ef4444",
    reopened: "#64748b"
  };
  return map[status] || "#059669";
}

function formatDate(isoStr) {
  if (!isoStr) return "-";
  try {
    const d = new Date(isoStr);
    return d.toLocaleString("id-ID", {
      day: "numeric",
      month: "short",
      year: "numeric",
      hour: "2-digit",
      minute: "2-digit"
    });
  } catch {
    return isoStr;
  }
}

function escapeHtml(str) {
  if (!str) return "";
  return String(str)
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#039;");
}

function showToast(message, type = "info") {
  const container = document.getElementById("toast-container");
  if (!container) return;

  const bgColors = {
    success: "bg-emerald-600 text-white",
    error: "bg-rose-600 text-white",
    warning: "bg-amber-600 text-white",
    info: "bg-slate-800 text-white"
  };

  const toast = document.createElement("div");
  toast.className = `px-4 py-2.5 rounded-lg shadow-lg text-xs font-semibold flex items-center gap-2 transform transition-all duration-300 translate-y-2 opacity-0 ${bgColors[type] || bgColors.info}`;
  toast.innerHTML = `<span>${escapeHtml(message)}</span>`;

  container.appendChild(toast);
  setTimeout(() => {
    toast.classList.remove("translate-y-2", "opacity-0");
  }, 10);

  setTimeout(() => {
    toast.classList.add("opacity-0", "translate-y-2");
    setTimeout(() => toast.remove(), 300);
  }, 3500);
}
