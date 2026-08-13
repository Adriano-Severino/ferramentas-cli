param(
    [string]$Version = "0.1.0",
    [string]$OutputDir = "",
    [switch]$SkipBuild,
    [switch]$SkipTests
)

$ErrorActionPreference = "Stop"

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$cliProject = Split-Path -Parent $scriptDir
$repoRoot = Split-Path -Parent $cliProject
$compilerProject = Join-Path $repoRoot "compilador-portugues"
$stdlibProject = Join-Path $repoRoot "sistema-padrao"

if ([string]::IsNullOrWhiteSpace($OutputDir)) {
    $OutputDir = Join-Path $repoRoot "dist"
}

$packageDir = Join-Path $OutputDir "pordosol-sdk-v$Version-windows-x64"
$binDir = Join-Path $packageDir "bin"
$toolsDir = Join-Path $packageDir "tools"
$templatesDir = Join-Path $packageDir "templates"
$stdlibDest = Join-Path $toolsDir "stdlib"

Write-Host "== Build Package Por do Sol SDK v$Version =="
Write-Host "Output: $packageDir"
Write-Host ""

function Invoke-CargoBuild {
    param(
        [string]$WorkDir,
        [string[]]$ExtraArgs
    )

    Push-Location $WorkDir
    try {
        Write-Host "Building in $WorkDir..."
        & cargo build --release @ExtraArgs
        if ($LASTEXITCODE -ne 0) {
            throw "Falha ao executar cargo build em $WorkDir"
        }
    }
    finally {
        Pop-Location
    }
}

function Test-ArtifactExists {
    param([string]$Path)
    if (-not (Test-Path -LiteralPath $Path)) {
        throw "Artefato nao encontrado: $Path"
    }
    Write-Host " $Path"
}

if (-not $SkipBuild) {
    Write-Host "=== Building Components ==="
    
    Invoke-CargoBuild -WorkDir $cliProject -ExtraArgs @("--bin", "pordosol")
    Invoke-CargoBuild -WorkDir $compilerProject -ExtraArgs @("--bin", "compilador", "--bin", "interpretador")
    
    Write-Host ""
}

if (-not $SkipTests) {
    Write-Host "=== Running Tests ==="
    
    Push-Location $cliProject
    try {
        Write-Host "Testing CLI..."
        & cargo test
        if ($LASTEXITCODE -ne 0) {
            throw "Falha nos testes da CLI"
        }
    }
    finally {
        Pop-Location
    }
    
    Push-Location $compilerProject
    try {
        Write-Host "Testing Compiler..."
        & cargo test
        if ($LASTEXITCODE -ne 0) {
            throw "Falha nos testes do compilador"
        }
    }
    finally {
        Pop-Location
    }
    
    Write-Host ""
}

Write-Host "=== Building Standard Library ==="

$compilerBinary = Join-Path $compilerProject "target\release\compilador.exe"
Test-ArtifactExists -Path $compilerBinary

Push-Location $stdlibProject
try {
    Write-Host "Compiling stdlib..."
    & $compilerBinary --compilar-biblioteca=.
    if ($LASTEXITCODE -ne 0) {
        throw "Falha ao compilar biblioteca padrao"
    }
}
finally {
    Pop-Location
}

Write-Host ""

Write-Host "=== Verifying Artifacts ==="

$cliSource = Join-Path $cliProject "target\release\pordosol.exe"
$compSource = Join-Path $compilerProject "target\release\compilador.exe"
$interpSource = Join-Path $compilerProject "target\release\interpretador.exe"
$templatesSource = Join-Path $cliProject "templates"

Test-ArtifactExists -Path $cliSource
Test-ArtifactExists -Path $compSource
Test-ArtifactExists -Path $interpSource
Test-ArtifactExists -Path $templatesSource

if (-not (Test-Path -LiteralPath $stdlibProject)) {
    throw "Biblioteca padrao nao encontrada em: $stdlibProject"
}

Write-Host ""

Write-Host "=== Creating Package Structure ==="

if (Test-Path -LiteralPath $packageDir) {
    Remove-Item -LiteralPath $packageDir -Recurse -Force
}
New-Item -ItemType Directory -Force -Path $binDir | Out-Null
New-Item -ItemType Directory -Force -Path $toolsDir | Out-Null
New-Item -ItemType Directory -Force -Path $templatesDir | Out-Null

Write-Host "Copying binaries..."
Copy-Item -LiteralPath $cliSource -Destination (Join-Path $binDir "pordosol.exe") -Force
Copy-Item -LiteralPath $compSource -Destination (Join-Path $toolsDir "compilador.exe") -Force
Copy-Item -LiteralPath $interpSource -Destination (Join-Path $toolsDir "interpretador.exe") -Force

Write-Host "Copying templates..."
Copy-Item -LiteralPath (Join-Path $templatesSource "*") -Destination $templatesDir -Recurse -Force

Write-Host "Copying stdlib..."
Copy-Item -LiteralPath $stdlibProject -Destination $stdlibDest -Recurse -Force

Write-Host "Copying install scripts..."
Copy-Item -LiteralPath (Join-Path $cliProject "install.ps1") -Destination $packageDir -Force
Copy-Item -LiteralPath (Join-Path $cliProject "install.sh") -Destination $packageDir -Force

Write-Host "Copying README..."
Copy-Item -LiteralPath (Join-Path $cliProject "README.md") -Destination $packageDir -Force

Write-Host ""

Write-Host "=== Creating Package Archive ==="

$zipFile = Join-Path $OutputDir "pordosol-sdk-v$Version-windows-x64.zip"
if (Test-Path -LiteralPath $zipFile) {
    Remove-Item -LiteralPath $zipFile -Force
}

Compress-Archive -LiteralPath $packageDir -DestinationPath $zipFile

Write-Host "Package created: $zipFile"

Write-Host ""

Write-Host "=== Generating Checksums ==="

$checksumFile = Join-Path $OutputDir "pordosol-sdk-v$Version-windows-x64.sha256"
$hash = Get-FileHash -LiteralPath $zipFile -Algorithm SHA256
$hash.Hash + "  " + (Split-Path $zipFile -Leaf) | Out-File -LiteralPath $checksumFile -Encoding ASCII

Write-Host "Checksum: $($hash.Hash)"
Write-Host "Checksum file: $checksumFile"

Write-Host ""
Write-Host "=== Build Complete ==="
Write-Host "Package: $zipFile"
Write-Host "Checksum: $checksumFile"
Write-Host ""
Write-Host "To test installation:"
Write-Host "  Expand-Archive -LiteralPath '$zipFile' -DestinationPath '$env:TEMP\pordosol-test'"
Write-Host "  cd '$env:TEMP\pordosol-test\pordosol-sdk-v$Version-windows-x64'"
Write-Host "  .\install.ps1"
