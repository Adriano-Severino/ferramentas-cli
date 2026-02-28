Param()
$ErrorActionPreference = 'Stop'

$tmp = Join-Path $env:TEMP ([System.Guid]::NewGuid().ToString())
New-Item -ItemType Directory -Path $tmp | Out-Null
Write-Host "[pordosol-e2e] usando tmpdir: $tmp"
Set-Location -Path $tmp

# Detecta comando pordosol
if (Get-Command pordosol -ErrorAction SilentlyContinue) {
    $pordosol = 'pordosol'
} elseif (Test-Path .\pordosol.exe) {
    $pordosol = '.\pordosol.exe'
} else {
    Write-Error "pordosol não encontrado no PATH. Execute após instalar o CLI ou coloque o binário no PATH."
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

# cleanup
Set-Location -Path $env:TEMP
Remove-Item -Recurse -Force $tmp
