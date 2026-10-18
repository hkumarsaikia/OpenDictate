#!/usr/bin/env bash
set -eo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
BUILD_DIR="$PROJECT_ROOT/build/rpmbuild"
DIST_DIR="${DIST_DIR:-$PROJECT_ROOT/dist}"
ARCH="$(uname -m)"
VERSION="$(grep -m1 '^version = ' "$PROJECT_ROOT/Cargo.toml" | cut -d '"' -f2 2>/dev/null || echo "2.0.3")"
[ -z "$VERSION" ] && VERSION="2.0.3"

echo "=== Building OpenDictate RPM Package ($VERSION-$ARCH) ==="

REBUILD=0
for arg in "$@"; do
    case "$arg" in
        --rebuild)
            REBUILD=1
            ;;
        -h|--help)
            echo "Usage: $0 [--rebuild]"
            exit 0
            ;;
    esac
done

# 1. Compile or verify binaries
PORTABLE_TARGET_DIR="$PROJECT_ROOT/target/portable"
AVX2_TARGET_DIR="$PROJECT_ROOT/target/avx2"
PORTABLE_BIN="$PORTABLE_TARGET_DIR/release/opendictate"
AVX2_BIN="$AVX2_TARGET_DIR/release/opendictate"

if [ ! -f "$PORTABLE_BIN" ] || [ "$REBUILD" -eq 1 ]; then
    echo "Compiling portable (x86-64-v1 / SSE2) release binary..."
    env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS \
        CARGO_BUILD_RUSTFLAGS="-C target-cpu=x86-64 -C target-feature=-sse3,-ssse3,-sse4.1,-sse4.2,-avx,-avx2,-fma,-f16c,-bmi1,-bmi2,-lzcnt,-popcnt,-movbe" \
        GGML_NATIVE=OFF GGML_SSE42=OFF GGML_AVX=OFF GGML_AVX2=OFF GGML_BMI2=OFF GGML_FMA=OFF GGML_F16C=OFF \
        cargo build --release --manifest-path "$PROJECT_ROOT/Cargo.toml" --target-dir "$PORTABLE_TARGET_DIR"
fi

if [ ! -f "$AVX2_BIN" ] || [ "$REBUILD" -eq 1 ]; then
    echo "Compiling optimized (AVX2 + FMA + F16C + BMI2 + SSE4.2) release binary..."
    env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS \
        CARGO_BUILD_RUSTFLAGS="-C target-cpu=x86-64 -C target-feature=+sse3,+ssse3,+sse4.1,+sse4.2,+avx,+avx2,+fma,+f16c,+bmi2,-bmi1,-lzcnt,-popcnt,-movbe" \
        GGML_NATIVE=OFF GGML_SSE42=ON GGML_AVX=ON GGML_AVX2=ON GGML_BMI2=ON GGML_FMA=ON GGML_F16C=ON \
        cargo build --release --manifest-path "$PROJECT_ROOT/Cargo.toml" --target-dir "$AVX2_TARGET_DIR"
fi

# 2. Stage RPM directory tree
echo "Staging RPM buildroot at $BUILD_DIR..."
rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR"/{SPECS,SOURCES,BUILD,RPMS,SRPMS,rpmdb}
mkdir -p "$DIST_DIR"

# 3. Create or export source tarball
SOURCE_TARBALL="$BUILD_DIR/SOURCES/opendictate-${VERSION}.tar.gz"
echo "Creating source archive at $SOURCE_TARBALL..."
if [ -d "$PROJECT_ROOT/.git" ]; then
    git -C "$PROJECT_ROOT" archive --format=tar.gz --prefix="opendictate-${VERSION}/" HEAD -o "$SOURCE_TARBALL"
else
    tar --exclude="./target" --exclude="./build" --exclude="./dist" --exclude="./.git" \
        --transform "s,^\.,opendictate-${VERSION}," \
        -czf "$SOURCE_TARBALL" -C "$PROJECT_ROOT" .
fi

# 4. Copy binaries to SOURCES
cp -a "$PORTABLE_BIN" "$BUILD_DIR/SOURCES/opendictate"
cp -a "$AVX2_BIN" "$BUILD_DIR/SOURCES/opendictate-avx2"
strip "$BUILD_DIR/SOURCES/opendictate" "$BUILD_DIR/SOURCES/opendictate-avx2"

# 5. Copy spec file
cp -a "$SCRIPT_DIR/opendictate.spec" "$BUILD_DIR/SPECS/"

# 6. Build RPM package
echo "Running rpmbuild..."
rpmbuild --define "_topdir $BUILD_DIR" --define "_dbpath $BUILD_DIR/rpmdb" -bb "$BUILD_DIR/SPECS/opendictate.spec"

# 7. Collect and inspect artifacts
RPM_OUT="$(find "$BUILD_DIR/RPMS" -name "*.rpm" | head -n 1)"
if [ -z "$RPM_OUT" ] || [ ! -f "$RPM_OUT" ]; then
    echo "Error: RPM build failed; no output found." >&2
    exit 1
fi

DEST_FILE="$DIST_DIR/$(basename "$RPM_OUT")"
cp -a "$RPM_OUT" "$DEST_FILE"
echo "=== RPM Package Built Successfully: $DEST_FILE ==="
rpm --dbpath "$BUILD_DIR/rpmdb" -qpi "$DEST_FILE"
