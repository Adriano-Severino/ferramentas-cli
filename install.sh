#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

INSTALL_ROOT="${PORDOSOL_HOME:-$HOME/.pordosol}"
CLI_PATH="$SCRIPT_DIR"
COMPILER_PATH="$REPO_ROOT/compilador-portugues"
STDLIB_PATH="$REPO_ROOT/sistema-padrao"
SKIP_BUILD=0
NO_PATH=0

usage() {
  cat <<'EOF'
Uso: ./install.sh [opcoes]

Opcoes:
  --install-root <dir>   Diretorio de instalacao (padrao: $PORDOSOL_HOME ou ~/.pordosol)
  --cli-path <dir>       Caminho do projeto ferramentas-cli
  --compiler-path <dir>  Caminho do projeto compilador-portugues
  --stdlib-path <dir>    Caminho da biblioteca padrao (sistema-padrao)
  --skip-build           Nao executar cargo build --release
  --no-path              Nao modificar arquivos de profile para PATH
  -h, --help             Exibe esta ajuda
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --install-root)
      INSTALL_ROOT="$2"
      shift 2
      ;;
    --cli-path)
      CLI_PATH="$2"
      shift 2
      ;;
    --compiler-path)
      COMPILER_PATH="$2"
      shift 2
      ;;
    --stdlib-path)
      STDLIB_PATH="$2"
      shift 2
      ;;
    --skip-build)
      SKIP_BUILD=1
      shift
      ;;
    --no-path)
      NO_PATH=1
      shift
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

require_cmd() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "Comando '$1' nao encontrado no PATH." >&2
    exit 1
  fi
}

upsert_profile_block() {
  local profile="$1"
  local begin="# >>> pordosol cli >>>"
  local end="# <<< pordosol cli <<<"
  local tmp
  tmp="$(mktemp)"

  touch "$profile"
  if grep -Fq "$begin" "$profile"; then
    awk -v b="$begin" -v e="$end" '
      $0 == b { skip = 1; next }
      $0 == e { skip = 0; next }
      skip == 0 { print }
    ' "$profile" > "$tmp"
    mv "$tmp" "$profile"
  else
    rm -f "$tmp"
  fi

  {
    echo "$begin"
    echo "export PORDOSOL_HOME=\"$INSTALL_ROOT\""
    if [[ $NO_PATH -eq 0 ]]; then
      echo "export PATH=\"\$PORDOSOL_HOME/bin:\$PATH\""
    fi
    echo "$end"
  } >> "$profile"
}

echo "== Instalador Por do Sol CLI =="
echo "CLI:         $CLI_PATH"
echo "Compilador:  $COMPILER_PATH"
echo "Stdlib:      $STDLIB_PATH"
echo "Destino:     $INSTALL_ROOT"

require_cmd cargo

if [[ $SKIP_BUILD -eq 0 ]]; then
  echo "Compilando CLI..."
  (cd "$CLI_PATH" && cargo build --release --bin pordosol)
  echo "Compilando compilador/interpretador..."
  (cd "$COMPILER_PATH" && cargo build --release --bin compilador --bin interpretador)
else
  echo "SkipBuild ativo: usando artefatos existentes."
fi

CLI_SOURCE="$CLI_PATH/target/release/pordosol"
COMP_SOURCE="$COMPILER_PATH/target/release/compilador"
INTERP_SOURCE="$COMPILER_PATH/target/release/interpretador"
TEMPLATES_SOURCE="$CLI_PATH/templates"

for f in "$CLI_SOURCE" "$COMP_SOURCE" "$INTERP_SOURCE"; do
  if [[ ! -f "$f" ]]; then
    echo "Artefato nao encontrado: $f" >&2
    exit 1
  fi
done

if [[ ! -d "$TEMPLATES_SOURCE" ]]; then
  echo "Templates nao encontrados em: $TEMPLATES_SOURCE" >&2
  exit 1
fi

if [[ ! -d "$STDLIB_PATH" ]]; then
  echo "Biblioteca padrao nao encontrada em: $STDLIB_PATH" >&2
  exit 1
fi

BIN_DIR="$INSTALL_ROOT/bin"
TOOLS_DIR="$INSTALL_ROOT/tools"
TEMPLATES_DIR="$INSTALL_ROOT/templates"

mkdir -p "$BIN_DIR" "$TOOLS_DIR" "$TEMPLATES_DIR"

cp "$CLI_SOURCE" "$BIN_DIR/pordosol"
cp "$COMP_SOURCE" "$TOOLS_DIR/compilador"
cp "$INTERP_SOURCE" "$TOOLS_DIR/interpretador"
chmod +x "$BIN_DIR/pordosol" "$TOOLS_DIR/compilador" "$TOOLS_DIR/interpretador"

rm -rf "$TEMPLATES_DIR"
mkdir -p "$TEMPLATES_DIR"
cp -R "$TEMPLATES_SOURCE/." "$TEMPLATES_DIR"

rm -rf "$TOOLS_DIR/stdlib"
cp -R "$STDLIB_PATH" "$TOOLS_DIR/stdlib"

export PORDOSOL_HOME="$INSTALL_ROOT"
if [[ $NO_PATH -eq 0 ]]; then
  export PATH="$PORDOSOL_HOME/bin:$PATH"
fi

profiles=()
if [[ -n "${SHELL:-}" ]]; then
  shell_name="$(basename "$SHELL")"
  case "$shell_name" in
    zsh) profiles+=("$HOME/.zshrc") ;;
    bash) profiles+=("$HOME/.bashrc") ;;
  esac
fi
for profile in "$HOME/.bashrc" "$HOME/.zshrc" "$HOME/.zprofile"; do
  if [[ -f "$profile" ]]; then
    profiles+=("$profile")
  fi
done

if [[ ${#profiles[@]} -eq 0 ]]; then
  profiles+=("$HOME/.bashrc")
fi

unique_profiles=()
for profile in "${profiles[@]}"; do
  already=0
  for existing in "${unique_profiles[@]}"; do
    if [[ "$existing" == "$profile" ]]; then
      already=1
      break
    fi
  done
  if [[ $already -eq 0 ]]; then
    unique_profiles+=("$profile")
  fi
done

for profile in "${unique_profiles[@]}"; do
  upsert_profile_block "$profile"
done

echo ""
echo "Instalacao concluida."
echo "PORDOSOL_HOME = $INSTALL_ROOT"
echo "Binario: $BIN_DIR/pordosol"
if [[ $NO_PATH -eq 0 ]]; then
  echo "PATH atualizado nos profiles: ${unique_profiles[*]}"
else
  echo "PATH nao foi alterado (--no-path)."
fi
echo ""
echo "Reabra o terminal e execute:"
echo "  pordosol doctor"
