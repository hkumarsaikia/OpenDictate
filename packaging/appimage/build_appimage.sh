#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
BUILD_DIR="$PROJECT_ROOT/build"
APP_DIR="$BUILD_DIR/AppDir"
DIST_DIR="$PROJECT_ROOT/dist"
ARCH="$(uname -m)"
VERSION="$(grep -m1 '^version = ' "$PROJECT_ROOT/Cargo.toml" | cut -d '"' -f2 2>/dev/null || echo "2.0.0")"
[ -z "$VERSION" ] && VERSION="2.0.0"
APP_NAME="OpenDictate"
APP_ID="io.github.opendictate.OpenDictate"

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

# 1. Compile or verify release binary
RELEASE_BIN="$PROJECT_ROOT/target/release/opendictate"
if [ ! -f "$RELEASE_BIN" ] || [ "$REBUILD" -eq 1 ]; then
    echo "Compiling release binary with Cargo..."
    cargo build --release --manifest-path "$PROJECT_ROOT/Cargo.toml"
fi

if [ ! -f "$RELEASE_BIN" ]; then
    echo "Error: Release binary not found at $RELEASE_BIN" >&2
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

# 3. Install AppRun and main executable
echo "Installing AppRun and executable..."
install -m 755 "$SCRIPT_DIR/AppRun" "$APP_DIR/AppRun"
install -m 755 "$RELEASE_BIN" "$APP_DIR/usr/bin/opendictate"

# 4. Install FreeDesktop desktop entry and AppStream metadata
echo "Installing desktop entry and AppStream metadata..."
install -m 644 "$PROJECT_ROOT/data/$APP_ID.desktop" "$APP_DIR/usr/share/applications/$APP_ID.desktop"
install -m 644 "$PROJECT_ROOT/data/$APP_ID.desktop" "$APP_DIR/$APP_ID.desktop"
ln -sf "$APP_ID.desktop" "$APP_DIR/opendictate.desktop"

install -m 644 "$PROJECT_ROOT/data/$APP_ID.metainfo.xml" "$APP_DIR/usr/share/metainfo/$APP_ID.metainfo.xml"

# 5. Install brand icon suite
echo "Installing application icons..."
install -m 644 "$PROJECT_ROOT/src/ui/assets/brand/opendictate.svg" "$APP_DIR/usr/share/icons/hicolor/scalable/apps/$APP_ID.svg"
install -m 644 "$PROJECT_ROOT/src/ui/assets/brand/opendictate.svg" "$APP_DIR/usr/share/icons/hicolor/scalable/apps/opendictate.svg"
install -m 644 "$PROJECT_ROOT/src/ui/assets/brand/opendictate-dark.svg" "$APP_DIR/usr/share/icons/hicolor/scalable/apps/opendictate-dark.svg"
install -m 644 "$PROJECT_ROOT/src/ui/assets/brand/opendictate-light.svg" "$APP_DIR/usr/share/icons/hicolor/scalable/apps/opendictate-light.svg"
install -m 644 "$PROJECT_ROOT/src/ui/assets/brand/opendictate.svg" "$APP_DIR/$APP_ID.svg"
cp -f "$PROJECT_ROOT/src/ui/assets/brand/opendictate.svg" "$APP_DIR/.DirIcon"

for size in 16x16 24x24 32x32 48x48 64x64 128x128 256x256 512x512; do
    png_file="$PROJECT_ROOT/src/ui/assets/brand/png/${size}.png"
    png_light_file="$PROJECT_ROOT/src/ui/assets/brand/png-light/${size}.png"
    if [ -f "$png_file" ]; then
        mkdir -p "$APP_DIR/usr/share/icons/hicolor/${size}/apps"
        install -m 644 "$png_file" "$APP_DIR/usr/share/icons/hicolor/${size}/apps/$APP_ID.png"
        install -m 644 "$png_file" "$APP_DIR/usr/share/icons/hicolor/${size}/apps/opendictate-dark.png"
    fi
    if [ -f "$png_light_file" ]; then
        mkdir -p "$APP_DIR/usr/share/icons/hicolor/${size}/apps"
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
    echo "appimagetool not found on PATH; generating squashfs bundle with mksquashfs..."
    rm -f "$SQUASHFS_FILE"
    mksquashfs "$APP_DIR" "$SQUASHFS_FILE" -root-owned -noappend
    echo "SquashFS bundle created successfully: $SQUASHFS_FILE"
else
    echo "Notice: Neither appimagetool nor mksquashfs found on PATH. AppDir prepared at: $APP_DIR"
fi

echo "=== OpenDictate AppDir Staging Complete ==="
