# ── offpkg Windows PowerShell Installer ───────────────────────────────────────
$ErrorActionPreference = "Stop"

$Repo = "https://github.com/aswin402/offpkg"
$BinaryName = "offpkg.exe"
$InstallDir = if ($env:INSTALL_DIR) { $env:INSTALL_DIR } else { Join-Path $env:USERPROFILE ".offpkg\bin" }
$BinaryPath = Join-Path $InstallDir $BinaryName

function Print-Logo {
    Write-Host ""
    Write-Host "╔═╗╔═╗╔═╗╔═╗╦╔═╔═╗" -ForegroundColor Cyan -NoNewline
    Write-Host ""
    Write-Host "║ ║╠╣ ╠╣ ╠═╝╠╩╗║ ╦" -ForegroundColor Cyan -NoNewline
    Write-Host ""
    Write-Host "╚═╝╚  ╚  ╩  ╩ ╩╚═╝" -ForegroundColor Cyan -NoNewline
    Write-Host ""
    Write-Host "  offpkg · universal offline package manager" -ForegroundColor DarkGray
    Write-Host ""
}

function Step($msg) { Write-Host "  →  " -ForegroundColor Cyan -NoNewline; Write-Host $msg }
function Done($msg) { Write-Host "  ✓  " -ForegroundColor Green -NoNewline; Write-Host $msg }
function Warn($msg) { Write-Host "  !  " -ForegroundColor Yellow -NoNewline; Write-Host $msg }
function Fail($msg) { Write-Host "  ✗  " -ForegroundColor Red -NoNewline; Write-Host $msg; exit 1 }

Print-Logo

# ── Detect Architecture ───────────────────────────────────────────────────────
$Arch = if ([Environment]::Is64BitOperatingSystem) { "x86_64" } else { "x86" }
if ($env:PROCESSOR_ARCHITECTURE -eq "ARM64") {
    $Arch = "aarch64"
}

Step "detected: windows / $Arch"

if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

# ── Download pre-built binary ────────────────────────────────────────────────
$ReleaseUrl = "$Repo/releases/latest/download/offpkg-windows-$Arch.exe"
$Downloaded = $false

Step "checking for pre-built Windows binary..."
try {
    [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
    $TempFile = [System.IO.Path]::GetTempFileName()
    Invoke-WebRequest -Uri $ReleaseUrl -OutFile $TempFile -UseBasicParsing -TimeoutSec 30
    if ((Get-Item $TempFile).Length -gt 10000) {
        Move-Item -Path $TempFile -Destination $BinaryPath -Force
        Done "binary installed to $BinaryPath"
        $Downloaded = $true
    } else {
        Remove-Item -Path $TempFile -Force -ErrorAction SilentlyContinue
    }
} catch {
    Warn "pre-built binary not found on release. Checking for cargo/source build..."
}

# ── Fallback: Build from source if Rust is installed ──────────────────────────
if (-not $Downloaded) {
    if (Get-Command cargo -ErrorAction SilentlyContinue) {
        Step "building offpkg from source with cargo..."
        $TempSrc = Join-Path ([System.IO.Path]::GetTempPath()) "offpkg_build_$(Get-Random)"
        try {
            git clone --depth 1 $Repo $TempSrc
            Set-Location $TempSrc
            cargo build --release --quiet
            Copy-Item "target\release\offpkg.exe" $BinaryPath -Force
            Done "built and installed to $BinaryPath"
            $Downloaded = $true
        } finally {
            Set-Location $PSScriptRoot
            Remove-Item $TempSrc -Recurse -Force -ErrorAction SilentlyContinue
        }
    } else {
        Fail "No pre-built binary available and Cargo (Rust) was not found. Install Rust from https://rustup.rs"
    }
}

# ── Add to User PATH ─────────────────────────────────────────────────────────
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$InstallDir*") {
    $NewPath = if ($UserPath.EndsWith(";")) { "$UserPath$InstallDir" } else { "$UserPath;$InstallDir" }
    [Environment]::SetEnvironmentVariable("Path", $NewPath, "User")
    $env:Path = "$env:Path;$InstallDir"
    Done "added $InstallDir to User PATH"
}

# ── Verification ─────────────────────────────────────────────────────────────
Write-Host ""
if (Test-Path $BinaryPath) {
    & $BinaryPath --version
    Done "offpkg installed successfully!"
} else {
    Warn "installation completed but $BinaryPath was not found"
}

Write-Host ""
Write-Host "  get started:" -ForegroundColor DarkGray
Write-Host "  offpkg doctor" -ForegroundColor Cyan
Write-Host "  offpkg stack list" -ForegroundColor Cyan
Write-Host ""
