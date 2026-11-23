# Builds both:
#   dist\GuristaSovMap.exe                 portable, just run it
#   dist\GuristaSovMap-Setup-<version>.exe installer (Start menu, shortcut, uninstaller)
# Run from PowerShell:   cd C:\Users\dange\Desktop\sov-map\desktop ; .\build.ps1
# (If Windows blocks the script: powershell -ExecutionPolicy Bypass -File .\build.ps1)
$ErrorActionPreference = 'Stop'
Set-Location $PSScriptRoot
$env:Path += ";$env:USERPROFILE\.cargo\bin"

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) { Write-Host 'Rust not found. Run: rustup default stable' -ForegroundColor Red; exit 1 }

$version = (Select-String -Path Cargo.toml -Pattern '^version\s*=\s*"([^"]+)"' | Select-Object -First 1).Matches[0].Groups[1].Value
Write-Host "Building Gurista Sov Map $version..." -ForegroundColor Cyan
# cmd /c keeps cargo's progress output (stderr) from tripping PowerShell; everything is also saved to build-log.txt
cmd /c "cargo build --release 2>&1" | Tee-Object -FilePath build-log.txt
if ($LASTEXITCODE -ne 0) { Write-Host "`nBuild failed. The full output is saved in build-log.txt - tell Claude and it can read it." -ForegroundColor Red; exit 1 }

New-Item -ItemType Directory -Force dist | Out-Null
Copy-Item target\release\sovmap.exe dist\GuristaSovMap.exe -Force
Write-Host "Portable app: dist\GuristaSovMap.exe" -ForegroundColor Green

function Find-Iscc {
  @("${env:ProgramFiles(x86)}\Inno Setup 6\ISCC.exe", "$env:ProgramFiles\Inno Setup 6\ISCC.exe", "$env:LOCALAPPDATA\Programs\Inno Setup 6\ISCC.exe") |
    Where-Object { Test-Path $_ } | Select-Object -First 1
}
$iscc = Find-Iscc
if (-not $iscc) {
  Write-Host 'Installing Inno Setup (free installer builder)...' -ForegroundColor Cyan
  winget install --id JRSoftware.InnoSetup -e --silent --accept-package-agreements --accept-source-agreements | Out-Null
  $iscc = Find-Iscc
}
if (-not $iscc) { Write-Host 'Inno Setup not found; install it from https://jrsoftware.org/isdl.php and run again.' -ForegroundColor Yellow; exit 1 }
& $iscc "/DAppVersion=$version" installer.iss | Out-Null
if ($LASTEXITCODE -ne 0) { Write-Host 'Installer build failed.' -ForegroundColor Red; exit 1 }
$hash = (Get-FileHash dist\GuristaSovMap.exe -Algorithm SHA256).Hash.ToLower()
"$hash  GuristaSovMap.exe" | Out-File dist\SHA256SUMS.txt -Encoding ascii
Write-Host "Installer:    dist\GuristaSovMap-Setup-$version.exe" -ForegroundColor Green

# Website release folder: upload everything in dist\app to public_html/sov-map/app/ (cPanel).
# The site's landing page and download page read app/latest.json, and installed apps check it
# every 6 hours (a few hundred bytes) and only download when the version is newer.
$app = 'dist\app'
New-Item -ItemType Directory -Force $app | Out-Null
Get-ChildItem $app -Filter *.exe | Remove-Item -Force
@'
# Gurista Sov Map: desktop app downloads
# .exe files always download (never open in the browser), and latest.json is never cached,
# so installed apps and the website always see the newest version straight away.
AddType application/octet-stream .exe
<FilesMatch "\.exe$">
  Header set Content-Disposition "attachment"
  Header set Cache-Control "public, max-age=31536000, immutable"
</FilesMatch>
<Files "latest.json">
  Header set Cache-Control "no-cache, must-revalidate"
  Header set Access-Control-Allow-Origin "*"
</Files>
Options -Indexes
'@ | Set-Content -Path "$app\.htaccess" -Encoding ascii
$exeName = "GuristaSovMap-$version.exe"
$setupName = "GuristaSovMap-Setup-$version.exe"
Copy-Item dist\GuristaSovMap.exe "$app\$exeName" -Force
Copy-Item "dist\GuristaSovMap-Setup-$version.exe" "$app\$setupName" -Force
$notes = ''
if (Test-Path CHANGELOG.txt) { $notes = ((Get-Content CHANGELOG.txt -Raw) -split "\r?\n\r?\n")[0].Trim() }
$latest = [ordered]@{
  version      = $version
  exe          = $exeName
  size         = (Get-Item "$app\$exeName").Length
  sha256       = $hash
  setup        = $setupName
  setup_size   = (Get-Item "$app\$setupName").Length
  setup_sha256 = (Get-FileHash "$app\$setupName" -Algorithm SHA256).Hash.ToLower()
  released     = (Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ssZ')
  notes        = $notes
}
[IO.File]::WriteAllText((Join-Path $PSScriptRoot "$app\latest.json"), ($latest | ConvertTo-Json))
Write-Host "Website files: dist\app  ->  upload to public_html/sov-map/app/" -ForegroundColor Green
Start-Process explorer.exe (Resolve-Path dist)
