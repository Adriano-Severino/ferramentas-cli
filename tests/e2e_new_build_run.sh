#!/usr/bin/env bash
set -euo pipefail

# Preparar tempdir e artefatos
TMPDIR=$(mktemp -d)
echo "[pordosol-e2e] usando tmpdir: $TMPDIR"
cd "$TMPDIR"

RUN_ID=${GITHUB_RUN_ID:-$(date +%s)}
if [ -n "${KEEP_ARTIFACTS:-}" ] && [ -n "${GITHUB_WORKSPACE:-}" ]; then
  ARTIFACT_DIR="$GITHUB_WORKSPACE/ferramentas-cli-e2e-artifacts-$RUN_ID"
  mkdir -p "$ARTIFACT_DIR"
  LOGFILE="$ARTIFACT_DIR/e2e.log"
else
  LOGFILE="$TMPDIR/e2e.log"
fi

exec > >(tee -a "$LOGFILE") 2>&1

echo "[pordosol-e2e] inicio do teste: $(date -u)"

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

# coletar artefatos se solicitado
if [ -n "${KEEP_ARTIFACTS:-}" ] && [ -n "${GITHUB_WORKSPACE:-}" ]; then
  echo "[pordosol-e2e] preservando artefatos em: $ARTIFACT_DIR"
  cp -r "$TMPDIR" "$ARTIFACT_DIR/tmp"
else
  echo "[pordosol-e2e] removendo tempdir"
  cd /
  rm -rf "$TMPDIR"
fi

echo "[pordosol-e2e] fim do teste: $(date -u)"
