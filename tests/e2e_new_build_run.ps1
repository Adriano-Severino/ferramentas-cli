Param()
$ErrorActionPreference = 'Stop'

$tmp = Join-Path $env:TEMP ([System.Guid]::NewGuid().ToString())
New-Item -ItemType Directory -Path $tmp | Out-Null
Write-Host "[pordosol-e2e] usando tmpdir: $tmp"
Set-Location -Path $tmp

$runId = $env:GITHUB_RUN_ID
if (-not $runId) { $runId = [int][double]::Parse((Get-Date -UFormat %s)) }
if ($env:KEEP_ARTIFACTS -and $env:GITHUB_WORKSPACE) {
    $artifactDir = Join-Path $env:GITHUB_WORKSPACE "ferramentas-cli-e2e-artifacts-$runId"
    New-Item -ItemType Directory -Path $artifactDir -Force | Out-Null
    $logFile = Join-Path $artifactDir 'e2e.log'
} else {
    $logFile = Join-Path $tmp 'e2e.log'
}

Start-Transcript -Path $logFile -Force | Out-Null

# Detecta comando pordosol
if (Get-Command pordosol -ErrorAction SilentlyContinue) {
    $pordosol = 'pordosol'
} elseif (Test-Path .\pordosol.exe) {
    $pordosol = '.\pordosol.exe'
} else {
    Write-Error "pordosol não encontrado no PATH. Execute após instalar o CLI ou coloque o binário no PATH."
    Stop-Transcript | Out-Null
    exit 2
}

Write-Host "[pordosol-e2e] criando novo projeto 'testapp'"
Start-Process -FilePath $pordosol -ArgumentList 'new','console','-n','testapp','-o','testapp' -Wait -NoNewWindow
Set-Location -Path (Join-Path $tmp 'testapp')

Write-Host "[pordosol-e2e] build"
Start-Process -FilePath $pordosol -ArgumentList 'build' -Wait -NoNewWindow

Write-Host "[pordosol-e2e] run (com --no-build)"
Start-Process -FilePath $pordosol -ArgumentList 'run','--no-build' -Wait -NoNewWindow

Write-Host "[pordosol-e2e] SUCESSO: new -> build -> run concluído"

Stop-Transcript | Out-Null

if ($env:KEEP_ARTIFACTS -and $env:GITHUB_WORKSPACE) {
    Write-Host "[pordosol-e2e] preservando artefatos em: $artifactDir"
    Copy-Item -Recurse -Force -Path $tmp -Destination (Join-Path $artifactDir 'tmp')
} else {
    Write-Host "[pordosol-e2e] removendo tempdir"
    Set-Location -Path $env:TEMP
    Remove-Item -Recurse -Force $tmp
}

Write-Host "[pordosol-e2e] fim do teste"
