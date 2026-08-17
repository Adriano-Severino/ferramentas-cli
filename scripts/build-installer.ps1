param(
    [string]$Version = "0.1.0",
    [string]$OutputDir = "",
    [switch]$SkipPackage
)

$ErrorActionPreference = "Stop"

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$cliProject = Split-Path -Parent $scriptDir
$wixConfig = Join-Path $scriptDir "wix-config.xml"

if ([string]::IsNullOrWhiteSpace($OutputDir)) {
    $OutputDir = Join-Path $cliProject "dist"
}

$packageDir = Join-Path $OutputDir "pordosol-sdk-v$Version-windows-x64"
$msiFile = Join-Path $OutputDir "pordosol-sdk-v$Version-windows-x64.msi"

Write-Host "== Build MSI Installer Por do Sol SDK v$Version =="
Write-Host "Output: $msiFile"
Write-Host ""

function Test-WiXInstalled {
    try {
        $candle = Get-Command "candle" -ErrorAction Stop
        $light = Get-Command "light" -ErrorAction Stop
        $heat = Get-Command "heat" -ErrorAction Stop
        Write-Host "WiX Toolset found"
        Write-Host "  candle: $($candle.Source)"
        Write-Host "  light: $($light.Source)"
        Write-Host "  heat: $($heat.Source)"
        return $true
    }
    catch {
        Write-Host "WiX Toolset not found"
        Write-Host "  Install from: https://wixtoolset.org/releases/"
        return $false
    }
}

if (-not (Test-WiXInstalled)) {
    throw "WiX Toolset is required to build MSI installer"
}

if (-not $SkipPackage) {
    Write-Host "=== Building Package First ==="
    & "$scriptDir\build-package.ps1" -Version $Version -OutputDir $OutputDir
    if ($LASTEXITCODE -ne 0) {
        throw "Falha ao buildar pacote"
    }
    Write-Host ""
}

if (-not (Test-Path -LiteralPath $packageDir)) {
    throw "Pacote nao encontrado: $packageDir"
}

$objDir = Join-Path $env:TEMP "pordosol-wix-obj-$Version"
if (Test-Path -LiteralPath $objDir) {
    Remove-Item -LiteralPath $objDir -Recurse -Force
}
New-Item -ItemType Directory -Force -Path $objDir | Out-Null

Write-Host "=== Harvesting Directory Contents (heat.exe) ==="
$harvestFile = Join-Path $objDir "HarvestedComponents.wxs"
# Use -b to bind SourceDir to packageDir and -srd to suppress the root DirectoryRef.
# -dr INSTALLFOLDER makes the harvested files land under INSTALLFOLDER.
# -sreg suppresses SelfReg harvesting; heat.exe is 32-bit and fails with error 193
# (ERROR_BAD_EXE_FORMAT) when it tries to load our 64-bit .exe binaries, which then
# leaves broken <File Source="<packageDir>"/> elements in the .wxs and breaks candle.
# -scom and -svb6 likewise avoid harvesting COM/VB6 registration entries we don't need.
Push-Location $packageDir
try {
    & heat dir "." -b "$packageDir" -gg -sfrag -srd -sreg -scom -svb6 -dr INSTALLFOLDER -cg HarvestedComponents -out $harvestFile
    if ($LASTEXITCODE -ne 0) {
        throw "Falha ao analisar o diretorio do pacote com heat.exe"
    }
}
finally {
    Pop-Location
}
Write-Host "Directory harvest complete."
Write-Host ""

Write-Host "=== Preparing WiX Configuration ==="
$wixContent = Get-Content -LiteralPath $wixConfig -Raw
$wixContent = $wixContent -replace '\{VERSION\}', $Version
$tempWix = Join-Path $objDir "pordosol-wix-config-$Version.xml"
$wixContent | Out-File -LiteralPath $tempWix -Encoding UTF8
Write-Host "WiX configuration prepared"
Write-Host ""

Write-Host "=== Compiling WiX Source (candle.exe) ==="
# -b binds the SourceDir reference used by heat-generated <File Source="SourceDir\..."/> elements.
& candle -out "$objDir\\" -b "$packageDir" -ext WixUIExtension $tempWix $harvestFile
if ($LASTEXITCODE -ne 0) {
    throw "Falha ao compilar WiX source com candle.exe"
}
Write-Host "WiX source compiled"
Write-Host ""

Write-Host "=== Linking MSI (light.exe) ==="
# -b $packageDir binds the harvested SourceDir references to the actual package directory.
# Enumerate the .wixobj files explicitly so PowerShell's wildcard expansion doesn't
# surprise us (e.g. if there's exactly one file, the glob never expands).
$wixObjFiles = Get-ChildItem -LiteralPath $objDir -Filter '*.wixobj' | Select-Object -ExpandProperty FullName
if (-not $wixObjFiles -or $wixObjFiles.Count -eq 0) {
    throw "Nenhum arquivo .wixobj encontrado em $objDir"
}
& light -out $msiFile -b "$packageDir" -ext WixUIExtension @wixObjFiles
if ($LASTEXITCODE -ne 0) {
    throw "Falha ao linkar MSI com light.exe"
}

Write-Host "MSI created"
Write-Host ""

Remove-Item -LiteralPath $objDir -Recurse -Force

Write-Host "=== Generating MSI Checksum ==="

$checksumFile = Join-Path $OutputDir "pordosol-sdk-v$Version-windows-x64.msi.sha256"
$hash = Get-FileHash -LiteralPath $msiFile -Algorithm SHA256
$hash.Hash + "  " + (Split-Path $msiFile -Leaf) | Out-File -LiteralPath $checksumFile -Encoding ASCII

Write-Host "Checksum: $($hash.Hash)"
Write-Host "Checksum file: $checksumFile"

Write-Host ""
Write-Host "=== MSI Build Complete ==="
Write-Host "Installer: $msiFile"
Write-Host "Checksum: $checksumFile"
Write-Host ""
Write-Host "To test installation:"
Write-Host "  msiexec /i '$msiFile'"
Write-Host ""
Write-Host "To uninstall:"
Write-Host "  msiexec /x '$msiFile'"