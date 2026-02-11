param(
    [Parameter(Mandatory = $true)]
    [string]$Version,
    [Parameter(Mandatory = $true)]
    [string]$Platform,
    [Parameter(Mandatory = $true)]
    [string]$Binary,
    [string]$Output = "dist"
)

$ErrorActionPreference = "Stop"

if (-not (Test-Path -LiteralPath $Binary)) {
    throw "Binario nao encontrado: $Binary"
}

$pkgName = "pordosol-$Version-$Platform"
$pkgDir = Join-Path $Output $pkgName
$archivePath = Join-Path $Output "$pkgName.zip"

New-Item -ItemType Directory -Force -Path $Output | Out-Null
if (Test-Path -LiteralPath $pkgDir) {
    Remove-Item -LiteralPath $pkgDir -Recurse -Force
}

New-Item -ItemType Directory -Force -Path (Join-Path $pkgDir "bin") | Out-Null
New-Item -ItemType Directory -Force -Path (Join-Path $pkgDir "tools") | Out-Null

$binName = if ($Binary.ToLowerInvariant().EndsWith(".exe")) { "pordosol.exe" } else { "pordosol" }
Copy-Item -LiteralPath $Binary -Destination (Join-Path $pkgDir "bin\$binName") -Force

if (Test-Path -LiteralPath "templates") {
    Copy-Item -LiteralPath "templates" -Destination (Join-Path $pkgDir "templates") -Recurse -Force
}
else {
    New-Item -ItemType Directory -Force -Path (Join-Path $pkgDir "templates") | Out-Null
}

foreach ($arquivo in @("install.sh", "install.ps1", "INSTALACAO.md", "README.md", "LICENSE")) {
    if (Test-Path -LiteralPath $arquivo) {
        Copy-Item -LiteralPath $arquivo -Destination (Join-Path $pkgDir $arquivo) -Force
    }
}

if ($env:PORDOSOL_COMPILER_BIN -and (Test-Path -LiteralPath $env:PORDOSOL_COMPILER_BIN)) {
    Copy-Item -LiteralPath $env:PORDOSOL_COMPILER_BIN -Destination (Join-Path $pkgDir "tools\compilador.exe") -Force
}
if ($env:PORDOSOL_INTERPRETER_BIN -and (Test-Path -LiteralPath $env:PORDOSOL_INTERPRETER_BIN)) {
    Copy-Item -LiteralPath $env:PORDOSOL_INTERPRETER_BIN -Destination (Join-Path $pkgDir "tools\interpretador.exe") -Force
}
if ($env:PORDOSOL_STDLIB_DIR -and (Test-Path -LiteralPath $env:PORDOSOL_STDLIB_DIR)) {
    Copy-Item -LiteralPath $env:PORDOSOL_STDLIB_DIR -Destination (Join-Path $pkgDir "tools\stdlib") -Recurse -Force
}

if (Test-Path -LiteralPath $archivePath) {
    Remove-Item -LiteralPath $archivePath -Force
}
Compress-Archive -Path "$pkgDir\*" -DestinationPath $archivePath -CompressionLevel Optimal -Force

$hash = (Get-FileHash -Path $archivePath -Algorithm SHA256).Hash.ToLowerInvariant()
$hashFile = "$archivePath.sha256"
"$hash  $([System.IO.Path]::GetFileName($archivePath))" | Set-Content -Path $hashFile -Encoding ascii

Write-Host "Pacote criado: $archivePath"
