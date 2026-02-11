param(
    [string]$InstallRoot = "",
    [string]$CliPath = "",
    [string]$CompilerPath = "",
    [string]$StdlibPath = "",
    [switch]$SkipBuild,
    [switch]$NoPath
)

$ErrorActionPreference = "Stop"

function Resolve-ExistingPath {
    param([string]$PathValue)
    return (Resolve-Path -LiteralPath $PathValue).Path
}

function Normalize-Path {
    param([string]$PathValue)
    return [System.IO.Path]::GetFullPath($PathValue)
}

function Ensure-Command {
    param([string]$Name)
    if (-not (Get-Command $Name -ErrorAction SilentlyContinue)) {
        throw "Comando '$Name' nao encontrado no PATH."
    }
}

function Invoke-CargoBuild {
    param(
        [string]$WorkDir,
        [string[]]$ExtraArgs
    )

    Push-Location $WorkDir
    try {
        & cargo build --release @ExtraArgs
        if ($LASTEXITCODE -ne 0) {
            throw "Falha ao executar cargo build em $WorkDir"
        }
    }
    finally {
        Pop-Location
    }
}

function Add-ToUserPath {
    param([string]$Entry)

    $entryNorm = Normalize-Path $Entry
    $current = [Environment]::GetEnvironmentVariable("Path", "User")
    $items = @()
    if (-not [string]::IsNullOrWhiteSpace($current)) {
        $items = $current.Split(";") | Where-Object { -not [string]::IsNullOrWhiteSpace($_) }
    }

    $exists = $false
    foreach ($item in $items) {
        if ((Normalize-Path $item).TrimEnd('\') -ieq $entryNorm.TrimEnd('\')) {
            $exists = $true
            break
        }
    }

    if (-not $exists) {
        $newPath = if ($items.Count -gt 0) { ($items + $entryNorm) -join ";" } else { $entryNorm }
        [Environment]::SetEnvironmentVariable("Path", $newPath, "User")
        Write-Host "PATH de usuario atualizado com: $entryNorm"
    }
    else {
        Write-Host "PATH de usuario ja contem: $entryNorm"
    }
}

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$repoRoot = Split-Path -Parent $scriptDir
$isWindows = $true
$exeSuffix = if ($isWindows) { ".exe" } else { "" }

if ([string]::IsNullOrWhiteSpace($CliPath)) {
    $CliPath = $scriptDir
}
if ([string]::IsNullOrWhiteSpace($CompilerPath)) {
    $CompilerPath = Join-Path $repoRoot "compilador-portugues"
}
if ([string]::IsNullOrWhiteSpace($StdlibPath)) {
    $StdlibPath = Join-Path $repoRoot "sistema-padrao"
}
if ([string]::IsNullOrWhiteSpace($InstallRoot)) {
    $InstallRoot = if ($env:PORDOSOL_HOME) { $env:PORDOSOL_HOME } else { Join-Path $HOME ".pordosol" }
}

$CliPath = Resolve-ExistingPath $CliPath
$CompilerPath = Resolve-ExistingPath $CompilerPath
$StdlibPath = Resolve-ExistingPath $StdlibPath
$InstallRoot = Normalize-Path $InstallRoot

$binDir = Join-Path $InstallRoot "bin"
$toolsDir = Join-Path $InstallRoot "tools"
$templatesDir = Join-Path $InstallRoot "templates"

Write-Host "== Instalador Por do Sol CLI =="
Write-Host "CLI:         $CliPath"
Write-Host "Compilador:  $CompilerPath"
Write-Host "Stdlib:      $StdlibPath"
Write-Host "Destino:     $InstallRoot"

Ensure-Command "cargo"

if (-not $SkipBuild) {
    Write-Host "Compilando CLI..."
    Invoke-CargoBuild -WorkDir $CliPath -ExtraArgs @("--bin", "pordosol")
    Write-Host "Compilando compilador/interpretador..."
    Invoke-CargoBuild -WorkDir $CompilerPath -ExtraArgs @("--bin", "compilador", "--bin", "interpretador")
}
else {
    Write-Host "SkipBuild ativo: usando artefatos existentes."
}

$cliSource = Join-Path $CliPath "target\release\pordosol$exeSuffix"
$compSource = Join-Path $CompilerPath "target\release\compilador$exeSuffix"
$interpSource = Join-Path $CompilerPath "target\release\interpretador$exeSuffix"
$templatesSource = Join-Path $CliPath "templates"

foreach ($arquivo in @($cliSource, $compSource, $interpSource)) {
    if (-not (Test-Path -LiteralPath $arquivo)) {
        throw "Artefato nao encontrado: $arquivo"
    }
}
if (-not (Test-Path -LiteralPath $templatesSource)) {
    throw "Templates nao encontrados em: $templatesSource"
}
if (-not (Test-Path -LiteralPath $StdlibPath)) {
    throw "Biblioteca padrao nao encontrada em: $StdlibPath"
}

New-Item -ItemType Directory -Force -Path $binDir | Out-Null
New-Item -ItemType Directory -Force -Path $toolsDir | Out-Null
New-Item -ItemType Directory -Force -Path $templatesDir | Out-Null

Copy-Item -LiteralPath $cliSource -Destination (Join-Path $binDir "pordosol$exeSuffix") -Force
Copy-Item -LiteralPath $compSource -Destination (Join-Path $toolsDir "compilador$exeSuffix") -Force
Copy-Item -LiteralPath $interpSource -Destination (Join-Path $toolsDir "interpretador$exeSuffix") -Force

if (Test-Path -LiteralPath $templatesDir) {
    Remove-Item -Path (Join-Path $templatesDir "*") -Force -Recurse -ErrorAction SilentlyContinue
}
Copy-Item -LiteralPath (Join-Path $templatesSource "*") -Destination $templatesDir -Recurse -Force

$stdlibDest = Join-Path $toolsDir "stdlib"
if (Test-Path -LiteralPath $stdlibDest) {
    Remove-Item -LiteralPath $stdlibDest -Recurse -Force
}
Copy-Item -LiteralPath $StdlibPath -Destination $stdlibDest -Recurse -Force

[Environment]::SetEnvironmentVariable("PORDOSOL_HOME", $InstallRoot, "User")
$env:PORDOSOL_HOME = $InstallRoot

if (-not $NoPath) {
    Add-ToUserPath -Entry $binDir
    if (-not ($env:Path.Split(";") | Where-Object { $_ -ieq $binDir })) {
        $env:Path = "$env:Path;$binDir"
    }
}
else {
    Write-Host "NoPath ativo: PATH de usuario nao foi alterado."
}

Write-Host ""
Write-Host "Instalacao concluida."
Write-Host "PORDOSOL_HOME = $InstallRoot"
Write-Host "Binario: $(Join-Path $binDir "pordosol$exeSuffix")"
Write-Host ""
Write-Host "Reabra o terminal e execute:"
Write-Host "  pordosol doctor"
