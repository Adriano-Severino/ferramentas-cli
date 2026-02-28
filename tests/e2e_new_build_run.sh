#!/usr/bin/env bash
set -euo pipefail

TMPDIR=$(mktemp -d)
echo "[pordosol-e2e] usando tmpdir: $TMPDIR"
cd "$TMPDIR"

# Detecta comando pordosol
if command -v pordosol >/dev/null 2>&1; then
  PORDOSOL_CMD="pordosol"
elif [ -x "./pordosol" ]; then
  PORDOSOL_CMD="./pordosol"
else
  echo "pordosol não encontrado no PATH. Execute após instalar o CLI ou coloque o binário no PATH."
  exit 2
fi

echo "[pordosol-e2e] criando novo projeto 'testapp'"
$PORDOSOL_CMD new console -n testapp -o testapp
cd testapp

echo "[pordosol-e2e] build"
$PORDOSOL_CMD build

echo "[pordosol-e2e] run (com --no-build)"
$PORDOSOL_CMD run --no-build

echo "[pordosol-e2e] SUCESSO: new -> build -> run concluído"

# cleanup
cd /
rm -rf "$TMPDIR"
