#!/usr/bin/env bash
set -euo pipefail

VERSION=""
PLATFORM=""
BINARY=""
OUTPUT_DIR="dist"

usage() {
  cat <<'EOF'
Uso: package.sh --version <versao> --platform <plataforma> --binary <arquivo> [--output <dir>]

Exemplo:
  ./scripts/release/package.sh --version 0.1.0 --platform linux-x64 --binary target/release/pordosol
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --version)
      VERSION="$2"
      shift 2
      ;;
    --platform)
      PLATFORM="$2"
      shift 2
      ;;
    --binary)
      BINARY="$2"
      shift 2
      ;;
    --output)
      OUTPUT_DIR="$2"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "Opcao invalida: $1" >&2
      usage
      exit 1
      ;;
  esac
done

if [[ -z "$VERSION" || -z "$PLATFORM" || -z "$BINARY" ]]; then
  echo "Parametros obrigatorios ausentes." >&2
  usage
  exit 1
fi

if [[ ! -f "$BINARY" ]]; then
  echo "Binario nao encontrado: $BINARY" >&2
  exit 1
fi

ROOT_DIR="$(pwd)"
PKG_NAME="pordosol-${VERSION}-${PLATFORM}"
PKG_DIR="${OUTPUT_DIR}/${PKG_NAME}"
BIN_NAME="pordosol"

mkdir -p "$OUTPUT_DIR"
rm -rf "$PKG_DIR"
mkdir -p "$PKG_DIR/bin" "$PKG_DIR/tools"

if [[ "$BINARY" == *.exe ]]; then
  BIN_NAME="pordosol.exe"
fi
cp "$BINARY" "$PKG_DIR/bin/${BIN_NAME}"
chmod +x "$PKG_DIR/bin/${BIN_NAME}" || true

if [[ -d "templates" ]]; then
  cp -R "templates" "$PKG_DIR/templates"
else
  mkdir -p "$PKG_DIR/templates"
fi

for f in install.sh install.ps1 INSTALACAO.md README.md LICENSE; do
  if [[ -f "$f" ]]; then
    cp "$f" "$PKG_DIR/$f"
  fi
done

if [[ -n "${PORDOSOL_COMPILER_BIN:-}" && -f "${PORDOSOL_COMPILER_BIN}" ]]; then
  cp "${PORDOSOL_COMPILER_BIN}" "$PKG_DIR/tools/compilador"
  chmod +x "$PKG_DIR/tools/compilador" || true
fi
if [[ -n "${PORDOSOL_INTERPRETER_BIN:-}" && -f "${PORDOSOL_INTERPRETER_BIN}" ]]; then
  cp "${PORDOSOL_INTERPRETER_BIN}" "$PKG_DIR/tools/interpretador"
  chmod +x "$PKG_DIR/tools/interpretador" || true
fi
if [[ -n "${PORDOSOL_STDLIB_DIR:-}" && -d "${PORDOSOL_STDLIB_DIR}" ]]; then
  cp -R "${PORDOSOL_STDLIB_DIR}" "$PKG_DIR/tools/stdlib"
fi

ARCHIVE_PATH="${OUTPUT_DIR}/${PKG_NAME}.tar.gz"
rm -f "$ARCHIVE_PATH" "${ARCHIVE_PATH}.sha256"

tar -C "$OUTPUT_DIR" -czf "$ARCHIVE_PATH" "$PKG_NAME"

if command -v sha256sum >/dev/null 2>&1; then
  hash="$(sha256sum "$ARCHIVE_PATH" | awk '{print $1}')"
elif command -v shasum >/dev/null 2>&1; then
  hash="$(shasum -a 256 "$ARCHIVE_PATH" | awk '{print $1}')"
else
  echo "Ferramenta de hash nao encontrada (sha256sum/shasum)." >&2
  exit 1
fi

echo "${hash}  $(basename "$ARCHIVE_PATH")" > "${ARCHIVE_PATH}.sha256"
echo "Pacote criado: ${ARCHIVE_PATH}"
