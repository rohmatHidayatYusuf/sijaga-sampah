# ========================================================================
# SIJAGA SAMPAH - PowerShell Local Web Server Launcher
# Runs a zero-dependency static HTTP server using .NET HttpListener
# ========================================================================

$Port = 3000
$RootPath = Join-Path $PSScriptRoot "frontend"

if (-not (Test-Path $RootPath)) {
    Write-Host "Direktori frontend tidak ditemukan di $RootPath" -ForegroundColor Red
    exit 1
}

$Listener = New-Object System.Net.HttpListener
$Prefix = "http://localhost:$Port/"
$Listener.Prefixes.Add($Prefix)

try {
    $Listener.Start()
} catch {
    Write-Host "Port $Port mungkin sedang digunakan. Mencoba port 3001..." -ForegroundColor Yellow
    $Port = 3001
    $Prefix = "http://localhost:$Port/"
    $Listener = New-Object System.Net.HttpListener
    $Listener.Prefixes.Add($Prefix)
    $Listener.Start()
}

Write-Host "==========================================================" -ForegroundColor Green
Write-Host " SIJAGA SAMPAH - Aplikasi Web Berhasil Dijalankan! " -ForegroundColor Green
Write-Host "==========================================================" -ForegroundColor Green
Write-Host "Akses aplikasi di browser: $Prefix" -ForegroundColor Cyan
Write-Host "Tekan Ctrl+C untuk menghentikan server." -ForegroundColor Yellow
Write-Host ""

# Buka browser secara otomatis
Start-Process $Prefix

$MimeTypes = @{
    ".html" = "text/html; charset=utf-8"
    ".css"  = "text/css; charset=utf-8"
    ".js"   = "application/javascript; charset=utf-8"
    ".json" = "application/json; charset=utf-8"
    ".svg"  = "image/svg+xml"
    ".png"  = "image/png"
    ".jpg"  = "image/jpeg"
    ".jpeg" = "image/jpeg"
    ".ico"  = "image/x-icon"
}

try {
    while ($Listener.IsListening) {
        $Context = $Listener.GetContext()
        $Request = $Context.Request
        $Response = $Context.Response

        $LocalPath = $Request.Url.LocalPath.TrimStart('/')
        if ([string]::IsNullOrEmpty($LocalPath) -or $LocalPath -eq "/") {
            $LocalPath = "index.html"
        }

        $FilePath = Join-Path $RootPath $LocalPath

        if (Test-Path $FilePath -PathType Leaf) {
            $Extension = [System.IO.Path]::GetExtension($FilePath).ToLower()
            $ContentType = $MimeTypes[$Extension]
            if (-not $ContentType) { $ContentType = "application/octet-stream" }

            $Response.ContentType = $ContentType
            $Response.AddHeader("Cache-Control", "no-cache")
            $Bytes = [System.IO.File]::ReadAllBytes($FilePath)
            $Response.ContentLength64 = $Bytes.Length
            $Response.OutputStream.Write($Bytes, 0, $Bytes.Length)
        } else {
            $Response.StatusCode = 404
            $Buffer = [System.Text.Encoding]::UTF8.GetBytes("404 Not Found")
            $Response.ContentLength64 = $Buffer.Length
            $Response.OutputStream.Write($Buffer, 0, $Buffer.Length)
        }

        $Response.OutputStream.Close()
    }
} finally {
    $Listener.Stop()
    $Listener.Close()
    Write-Host "Server telah dihentikan." -ForegroundColor Gray
}
