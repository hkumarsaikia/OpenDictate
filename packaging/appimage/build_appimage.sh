#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
BUILD_DIR="$PROJECT_ROOT/build"
APP_DIR="$BUILD_DIR/AppDir"
DIST_DIR="${DIST_DIR:-$PROJECT_ROOT/dist}"
ARCH="$(uname -m)"
VERSION="$(grep -m1 '^version = ' "$PROJECT_ROOT/Cargo.toml" | cut -d '"' -f2 2>/dev/null || echo "2.0.3")"
[ -z "$VERSION" ] && VERSION="2.0.3"
APP_NAME="OpenDictate"
APP_ID="io.github.hkumarsaikia.OpenDictate"

echo "=== Building OpenDictate AppImage ($VERSION-$ARCH) ==="

REBUILD=0
APPDIR_ONLY=0

for arg in "$@"; do
    case "$arg" in
        --rebuild)
            REBUILD=1
            ;;
        --appdir-only)
            APPDIR_ONLY=1
            ;;
        -h|--help)
            echo "Usage: $0 [--rebuild] [--appdir-only]"
            exit 0
            ;;
    esac
done

# 1. Compile or verify portable (SSE2) and optimized (AVX2) release binaries
if [ "$REBUILD" -eq 1 ]; then
    REBUILD_TARGET_ROOT="$BUILD_DIR/appimage-rebuild-targets"
    echo "Preparing isolated fresh build directories under $REBUILD_TARGET_ROOT..."
    rm -rf "$REBUILD_TARGET_ROOT"
    PORTABLE_TARGET_DIR="$REBUILD_TARGET_ROOT/portable"
    AVX2_TARGET_DIR="$REBUILD_TARGET_ROOT/avx2"
else
    PORTABLE_TARGET_DIR="$PROJECT_ROOT/target/portable"
    AVX2_TARGET_DIR="$PROJECT_ROOT/target/avx2"
fi

PORTABLE_BIN="$PORTABLE_TARGET_DIR/release/opendictate"
AVX2_BIN="$AVX2_TARGET_DIR/release/opendictate"

if [ ! -f "$PORTABLE_BIN" ] || [ "$REBUILD" -eq 1 ]; then
    echo "Compiling portable (x86-64-v1 / SSE2) release binary into $PORTABLE_TARGET_DIR..."
    env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS \
        CARGO_BUILD_RUSTFLAGS="-C target-cpu=x86-64 -C target-feature=-sse3,-ssse3,-sse4.1,-sse4.2,-avx,-avx2,-fma,-f16c,-bmi1,-bmi2,-lzcnt,-popcnt,-movbe" \
        GGML_NATIVE=OFF \
        GGML_SSE42=OFF \
        GGML_AVX=OFF \
        GGML_AVX2=OFF \
        GGML_BMI2=OFF \
        GGML_FMA=OFF \
        GGML_F16C=OFF \
        GGML_AVX_VNNI=OFF \
        GGML_AVX512=OFF \
        GGML_AVX512_VBMI=OFF \
        GGML_AVX512_VNNI=OFF \
        GGML_AVX512_BF16=OFF \
        GGML_AMX_TILE=OFF \
        GGML_AMX_INT8=OFF \
        GGML_AMX_BF16=OFF \
        cargo build --release --manifest-path "$PROJECT_ROOT/Cargo.toml" --target-dir "$PORTABLE_TARGET_DIR"
fi

if [ ! -f "$AVX2_BIN" ] || [ "$REBUILD" -eq 1 ]; then
    echo "Compiling optimized (AVX2 + FMA + F16C + BMI2 + SSE4.2) release binary into $AVX2_TARGET_DIR..."
    env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS \
        CARGO_BUILD_RUSTFLAGS="-C target-cpu=x86-64 -C target-feature=+sse3,+ssse3,+sse4.1,+sse4.2,+avx,+avx2,+fma,+f16c,+bmi2,-bmi1,-lzcnt,-popcnt,-movbe" \
        GGML_NATIVE=OFF \
        GGML_SSE42=ON \
        GGML_AVX=ON \
        GGML_AVX2=ON \
        GGML_BMI2=ON \
        GGML_FMA=ON \
        GGML_F16C=ON \
        GGML_AVX_VNNI=OFF \
        GGML_AVX512=OFF \
        GGML_AVX512_VBMI=OFF \
        GGML_AVX512_VNNI=OFF \
        GGML_AVX512_BF16=OFF \
        GGML_AMX_TILE=OFF \
        GGML_AMX_INT8=OFF \
        GGML_AMX_BF16=OFF \
        cargo build --release --manifest-path "$PROJECT_ROOT/Cargo.toml" --target-dir "$AVX2_TARGET_DIR"
fi

if [ ! -f "$PORTABLE_BIN" ]; then
    echo "Error: Portable release binary not found at $PORTABLE_BIN" >&2
    exit 1
fi
if [ ! -f "$AVX2_BIN" ]; then
    echo "Error: Optimized AVX2 release binary not found at $AVX2_BIN" >&2
    exit 1
fi

# 2. Stage AppDir directory hierarchy
echo "Staging AppDir filesystem at $APP_DIR..."
rm -rf "$APP_DIR"
mkdir -p "$APP_DIR/usr/bin"
mkdir -p "$APP_DIR/usr/lib"
mkdir -p "$APP_DIR/usr/share/applications"
mkdir -p "$APP_DIR/usr/share/metainfo"
mkdir -p "$APP_DIR/usr/share/icons/hicolor/scalable/apps"
mkdir -p "$APP_DIR/usr/share/glib-2.0/schemas"
mkdir -p "$DIST_DIR"

# 3. Install AppRun and dual binaries (portable launcher + AVX2 sibling)
echo "Installing AppRun and dual executables..."
install -m 755 "$SCRIPT_DIR/AppRun" "$APP_DIR/AppRun"
install -m 755 "$PORTABLE_BIN" "$APP_DIR/usr/bin/opendictate"
install -m 755 "$AVX2_BIN" "$APP_DIR/usr/bin/opendictate-avx2"

# 4. Install FreeDesktop desktop entry and AppStream metadata
echo "Installing desktop entry and AppStream metadata..."
install -m 644 "$PROJECT_ROOT/data/$APP_ID.desktop" "$APP_DIR/usr/share/applications/$APP_ID.desktop"
install -m 644 "$PROJECT_ROOT/data/$APP_ID.desktop" "$APP_DIR/$APP_ID.desktop"
ln -sf "$APP_ID.desktop" "$APP_DIR/opendictate.desktop"

install -m 644 "$PROJECT_ROOT/data/$APP_ID.metainfo.xml" "$APP_DIR/usr/share/metainfo/$APP_ID.metainfo.xml"

# Install application and third-party licenses
mkdir -p "$APP_DIR/usr/share/licenses/opendictate" "$APP_DIR/usr/share/doc/opendictate"
install -m 644 "$PROJECT_ROOT/LICENSE" "$APP_DIR/usr/share/licenses/opendictate/LICENSE"
install -m 644 "$PROJECT_ROOT/LICENSE" "$APP_DIR/usr/share/doc/opendictate/copyright"
if [ -d "$PROJECT_ROOT/licenses" ]; then
    cp -f "$PROJECT_ROOT/licenses/"* "$APP_DIR/usr/share/licenses/opendictate/" 2>/dev/null || true
    install -m 644 "$PROJECT_ROOT/licenses/THIRD_PARTY_LICENSES.md" "$APP_DIR/usr/share/doc/opendictate/THIRD_PARTY_LICENSES.md"
fi

# 5. Install brand icon suite
echo "Installing application icons..."
install -m 644 "$PROJECT_ROOT/src/ui/assets/brand/opendictate.svg" "$APP_DIR/usr/share/icons/hicolor/scalable/apps/$APP_ID.svg"
install -m 644 "$PROJECT_ROOT/src/ui/assets/brand/opendictate.svg" "$APP_DIR/usr/share/icons/hicolor/scalable/apps/opendictate.svg"
install -m 644 "$PROJECT_ROOT/src/ui/assets/brand/opendictate-dark.svg" "$APP_DIR/usr/share/icons/hicolor/scalable/apps/opendictate-dark.svg"
install -m 644 "$PROJECT_ROOT/src/ui/assets/brand/opendictate-light.svg" "$APP_DIR/usr/share/icons/hicolor/scalable/apps/opendictate-light.svg"
install -m 644 "$PROJECT_ROOT/src/ui/assets/brand/opendictate-dark.svg" "$APP_DIR/usr/share/icons/hicolor/scalable/apps/$APP_ID-dark.svg"
install -m 644 "$PROJECT_ROOT/src/ui/assets/brand/opendictate-light.svg" "$APP_DIR/usr/share/icons/hicolor/scalable/apps/$APP_ID-light.svg"
install -m 644 "$PROJECT_ROOT/src/ui/assets/brand/opendictate.svg" "$APP_DIR/$APP_ID.svg"
cp -f "$PROJECT_ROOT/src/ui/assets/brand/opendictate.svg" "$APP_DIR/.DirIcon"

for size in 16x16 24x24 32x32 48x48 64x64 128x128 256x256 512x512; do
    png_file="$PROJECT_ROOT/src/ui/assets/brand/png/${size}.png"
    png_light_file="$PROJECT_ROOT/src/ui/assets/brand/png-light/${size}.png"
    if [ -f "$png_file" ]; then
        mkdir -p "$APP_DIR/usr/share/icons/hicolor/${size}/apps"
        install -m 644 "$png_file" "$APP_DIR/usr/share/icons/hicolor/${size}/apps/$APP_ID.png"
        install -m 644 "$png_file" "$APP_DIR/usr/share/icons/hicolor/${size}/apps/$APP_ID-dark.png"
        install -m 644 "$png_file" "$APP_DIR/usr/share/icons/hicolor/${size}/apps/opendictate-dark.png"
    fi
    if [ -f "$png_light_file" ]; then
        mkdir -p "$APP_DIR/usr/share/icons/hicolor/${size}/apps"
        install -m 644 "$png_light_file" "$APP_DIR/usr/share/icons/hicolor/${size}/apps/$APP_ID-light.png"
        install -m 644 "$png_light_file" "$APP_DIR/usr/share/icons/hicolor/${size}/apps/opendictate-light.png"
    fi
done

# 6. Compile GSettings schemas if present
if command -v glib-compile-schemas &>/dev/null; then
    if compgen -G "$APP_DIR/usr/share/glib-2.0/schemas/*.gschema.xml" > /dev/null; then
        echo "Compiling GLib schemas..."
        glib-compile-schemas "$APP_DIR/usr/share/glib-2.0/schemas"
    fi
fi

if [ "$APPDIR_ONLY" -eq 1 ]; then
    echo "AppDir staged successfully at $APP_DIR"
    exit 0
fi

# 7. Package SquashFS / AppImage
APPIMAGE_FILE="$DIST_DIR/${APP_NAME}-${VERSION}-${ARCH}.AppImage"
SQUASHFS_FILE="$DIST_DIR/${APP_NAME}-${VERSION}-${ARCH}.squashfs"

if command -v appimagetool &>/dev/null; then
    echo "Generating AppImage with system appimagetool..."
    ARCH="$ARCH" appimagetool "$APP_DIR" "$APPIMAGE_FILE"
    chmod +x "$APPIMAGE_FILE"
    echo "AppImage created successfully: $APPIMAGE_FILE"
elif [ -f "$BUILD_DIR/appimagetool" ]; then
    echo "Generating AppImage with cached appimagetool..."
    ARCH="$ARCH" "$BUILD_DIR/appimagetool" "$APP_DIR" "$APPIMAGE_FILE"
    chmod +x "$APPIMAGE_FILE"
    echo "AppImage created successfully: $APPIMAGE_FILE"
elif command -v mksquashfs &>/dev/null; then
    echo "Generating SquashFS payload with mksquashfs..."
    rm -f "$SQUASHFS_FILE" "$APPIMAGE_FILE"
    mksquashfs "$APP_DIR" "$SQUASHFS_FILE" -root-owned -noappend -comp zstd
    RUNTIME_BIN="$BUILD_DIR/runtime-${ARCH}"
    if [ ! -f "$RUNTIME_BIN" ] && command -v curl &>/dev/null; then
        echo "Downloading AppImage Type-2 runtime for ${ARCH}..."
        curl -fsSL "https://github.com/AppImage/type2-runtime/releases/download/continuous/runtime-${ARCH}" -o "$RUNTIME_BIN" || rm -f "$RUNTIME_BIN"
    fi
    if [ -f "$RUNTIME_BIN" ] && [ -s "$RUNTIME_BIN" ]; then
        cat "$RUNTIME_BIN" "$SQUASHFS_FILE" > "$APPIMAGE_FILE"
        chmod +x "$APPIMAGE_FILE"
        rm -f "$SQUASHFS_FILE"
        echo "AppImage created successfully: $APPIMAGE_FILE"
    else
        echo "SquashFS bundle created successfully: $SQUASHFS_FILE"
    fi
else
    echo "Notice: Neither appimagetool nor mksquashfs found on PATH. AppDir prepared at: $APP_DIR"
fi

echo "=== OpenDictate AppImage Build Complete ==="
