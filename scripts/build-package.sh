#!/usr/bin/env bash
set -euo pipefail

VERSION="${1:-0.1.0}"
OUTPUT_DIR="${2:-}"
SKIP_BUILD="${3:-0}"
SKIP_TESTS="${4:-0}"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
CLI_PROJECT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Check for dependencies in both locations (CI subdirectory or local sibling directory)
COMPILER_PROJECT="$CLI_PROJECT/compilador-portugues"
if [ ! -d "$COMPILER_PROJECT" ]; then
    COMPILER_PROJECT="$REPO_ROOT/compilador-portugues"
fi

STDLIB_PROJECT="$CLI_PROJECT/sistema-padrao"
if [ ! -d "$STDLIB_PROJECT" ]; then
    STDLIB_PROJECT="$REPO_ROOT/sistema-padrao"
fi

if [ -z "$OUTPUT_DIR" ]; then
    OUTPUT_DIR="$CLI_PROJECT/dist"
fi

PACKAGE_DIR="$OUTPUT_DIR/pordosol-sdk-v$VERSION-linux-x64"
BIN_DIR="$PACKAGE_DIR/bin"
TOOLS_DIR="$PACKAGE_DIR/tools"
TEMPLATES_DIR="$PACKAGE_DIR/templates"
STDLIB_DEST="$TOOLS_DIR/stdlib"

echo "== Build Package Por do Sol SDK v$VERSION =="
echo "Output: $PACKAGE_DIR"
echo ""

cargo_build() {
    local workdir="$1"
    shift
    echo "Building in $workdir..."
    (cd "$workdir" && cargo build --release "$@")
}

test_artifact_exists() {
    local path="$1"
    if [ ! -f "$path" ]; then
        echo "ERROR: Artifact not found: $path"
        exit 1
    fi
    echo "✓ $path"
}

if [ "$SKIP_BUILD" -eq 0 ]; then
    echo "=== Building Components ==="
    
    # Build CLI
    cargo_build "$CLI_PROJECT" --bin pordosol
    
    # Build compiler and interpreter
    cargo_build "$COMPILER_PROJECT" --bin compilador --bin interpretador
    
    echo ""
fi

if [ "$SKIP_TESTS" -eq 0 ]; then
    echo "=== Running Tests ==="
    
    echo "Testing CLI..."
    (cd "$CLI_PROJECT" && cargo test)
    
    echo "Testing Compiler..."
    (cd "$COMPILER_PROJECT" && cargo test)
    
    echo ""
fi

echo "=== Building Standard Library ==="

COMPILER_BINARY="$COMPILER_PROJECT/target/release/compilador"
test_artifact_exists "$COMPILER_BINARY"

echo "Compiling stdlib..."
(cd "$STDLIB_PROJECT" && "$COMPILER_BINARY" --compilar-biblioteca=.)

echo ""

echo "=== Verifying Artifacts ==="

CLI_SOURCE="$CLI_PROJECT/target/release/pordosol"
COMP_SOURCE="$COMPILER_PROJECT/target/release/compilador"
INTERP_SOURCE="$COMPILER_PROJECT/target/release/interpretador"
TEMPLATES_SOURCE="$CLI_PROJECT/templates"

test_artifact_exists "$CLI_SOURCE"
test_artifact_exists "$COMP_SOURCE"
test_artifact_exists "$INTERP_SOURCE"

if [ ! -d "$TEMPLATES_SOURCE" ]; then
    echo "ERROR: Templates directory not found: $TEMPLATES_SOURCE"
    exit 1
fi

if [ ! -d "$STDLIB_PROJECT" ]; then
    echo "ERROR: Standard library not found at: $STDLIB_PROJECT"
    exit 1
fi

echo ""

echo "=== Creating Package Structure ==="

# Clean and create directories
rm -rf "$PACKAGE_DIR"
mkdir -p "$BIN_DIR" "$TOOLS_DIR" "$TEMPLATES_DIR"

echo "Copying binaries..."
cp "$CLI_SOURCE" "$BIN_DIR/pordosol"
cp "$COMP_SOURCE" "$TOOLS_DIR/compilador"
cp "$INTERP_SOURCE" "$TOOLS_DIR/interpretador"
chmod +x "$BIN_DIR/pordosol" "$TOOLS_DIR/compilador" "$TOOLS_DIR/interpretador"

echo "Copying templates..."
cp -R "$TEMPLATES_SOURCE"/* "$TEMPLATES_DIR/"

echo "Copying stdlib..."
# Copy only the compiled stdlib artifacts, not the source code
STDLIB_DIST="$STDLIB_PROJECT/dist"
if [ -d "$STDLIB_DIST" ]; then
    # Create the destination directory first
    mkdir -p "$STDLIB_DEST"
    # Copy only the compiled files (not the source directory structure)
    cp "$STDLIB_DIST"/* "$STDLIB_DEST/"
else
    # Fallback: copy the entire stdlib project if dist doesn't exist
    cp -R "$STDLIB_PROJECT" "$STDLIB_DEST"
fi

echo "Copying install scripts..."
cp "$CLI_PROJECT/install.sh" "$PACKAGE_DIR/"

echo "Copying README..."
cp "$CLI_PROJECT/README.md" "$PACKAGE_DIR/"

echo ""

echo "=== Creating Package Archive ==="

TAR_FILE="$OUTPUT_DIR/pordosol-sdk-v$VERSION-linux-x64.tar.gz"
rm -f "$TAR_FILE"

tar -czf "$TAR_FILE" -C "$OUTPUT_DIR" "$(basename "$PACKAGE_DIR")"

echo "Package created: $TAR_FILE"

echo ""

echo "=== Generating Checksums ==="

CHECKSUM_FILE="$OUTPUT_DIR/pordosol-sdk-v$VERSION-linux-x64.sha256"
sha256sum "$(basename "$TAR_FILE")" > "$CHECKSUM_FILE"

CHECKSUM=$(sha256sum "$TAR_FILE" | cut -d ' ' -f 1)
echo "Checksum: $CHECKSUM"
echo "Checksum file: $CHECKSUM_FILE"

echo ""
echo "=== Build Complete ==="
echo "Package: $TAR_FILE"
echo "Checksum: $CHECKSUM_FILE"
echo ""
echo "To test installation:"
echo "  tar -xzf '$TAR_FILE' -C '$TMPDIR'"
echo "  cd '$TMPDIR/$(basename "$PACKAGE_DIR")'"
echo "  ./install.sh"