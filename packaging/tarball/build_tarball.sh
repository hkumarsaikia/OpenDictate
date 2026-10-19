#!/usr/bin/env bash
set -eo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
VERSION="$(grep -m1 '^version = ' "$PROJECT_ROOT/Cargo.toml" | cut -d '"' -f2 2>/dev/null || echo "2.0.3")"
[ -z "$VERSION" ] && VERSION="2.0.3"
DIST_DIR="${DIST_DIR:-$PROJECT_ROOT/dist}"

STAGE_DIR="$PROJECT_ROOT/build/tarball-stage"
PKG_DIR_NAME="opendictate-${VERSION}-linux-x86_64"
TARGET_STAGE="$STAGE_DIR/$PKG_DIR_NAME"

echo "=== Building Portable Tarballs for OpenDictate $VERSION (x86_64) ==="

rm -rf "$STAGE_DIR"
mkdir -p "$TARGET_STAGE"/{share/applications,share/metainfo,share/icons/hicolor/scalable/apps}
mkdir -p "$DIST_DIR"

# 1. Binaries (stripped)
PORTABLE_BIN="$PROJECT_ROOT/target/portable/release/opendictate"
AVX2_BIN="$PROJECT_ROOT/target/avx2/release/opendictate"

if [ ! -f "$PORTABLE_BIN" ] || [ ! -f "$AVX2_BIN" ]; then
    echo "Error: Release binaries not found in target directories. Compile them first." >&2
    exit 1
fi

install -m 755 "$PORTABLE_BIN" "$TARGET_STAGE/opendictate"
install -m 755 "$AVX2_BIN" "$TARGET_STAGE/opendictate-avx2"
strip "$TARGET_STAGE/opendictate" "$TARGET_STAGE/opendictate-avx2"

# 2. Launcher script
cat << 'EOF' > "$TARGET_STAGE/opendictate.sh"
#!/usr/bin/env bash
set -e
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
export PATH="$HERE:$PATH"
export XDG_DATA_DIRS="$HERE/share:${XDG_DATA_DIRS:-/usr/local/share:/usr/share}"
exec "$HERE/opendictate" "$@"
EOF
chmod 755 "$TARGET_STAGE/opendictate.sh"

# 3. Documentation & licenses
install -m 644 "$PROJECT_ROOT/README.md" "$TARGET_STAGE/README.md"
install -m 644 "$PROJECT_ROOT/LICENSE" "$TARGET_STAGE/LICENSE"
install -m 644 "$PROJECT_ROOT/licenses/THIRD_PARTY_LICENSES.md" "$TARGET_STAGE/THIRD_PARTY_LICENSES.md"

# 4. Desktop entry & AppStream metainfo
install -m 644 "$PROJECT_ROOT/data/io.github.hkumarsaikia.OpenDictate.desktop" "$TARGET_STAGE/share/applications/io.github.hkumarsaikia.OpenDictate.desktop"
install -m 644 "$PROJECT_ROOT/data/io.github.hkumarsaikia.OpenDictate.metainfo.xml" "$TARGET_STAGE/share/metainfo/io.github.hkumarsaikia.OpenDictate.metainfo.xml"

# 5. Icons
install -m 644 "$PROJECT_ROOT/src/ui/assets/brand/opendictate.svg" "$TARGET_STAGE/share/icons/hicolor/scalable/apps/opendictate.svg"
install -m 644 "$PROJECT_ROOT/src/ui/assets/brand/opendictate.svg" "$TARGET_STAGE/share/icons/hicolor/scalable/apps/io.github.hkumarsaikia.OpenDictate.svg"
install -m 644 "$PROJECT_ROOT/src/ui/assets/brand/opendictate-dark.svg" "$TARGET_STAGE/share/icons/hicolor/scalable/apps/opendictate-dark.svg"
install -m 644 "$PROJECT_ROOT/src/ui/assets/brand/opendictate-light.svg" "$TARGET_STAGE/share/icons/hicolor/scalable/apps/opendictate-light.svg"
install -m 644 "$PROJECT_ROOT/src/ui/assets/brand/opendictate-dark.svg" "$TARGET_STAGE/share/icons/hicolor/scalable/apps/io.github.hkumarsaikia.OpenDictate-dark.svg"
install -m 644 "$PROJECT_ROOT/src/ui/assets/brand/opendictate-light.svg" "$TARGET_STAGE/share/icons/hicolor/scalable/apps/io.github.hkumarsaikia.OpenDictate-light.svg"

for size in 16x16 24x24 32x32 48x48 64x64 128x128 256x256 512x512; do
    mkdir -p "$TARGET_STAGE/share/icons/hicolor/${size}/apps"
    install -m 644 "$PROJECT_ROOT/src/ui/assets/brand/png/${size}.png" "$TARGET_STAGE/share/icons/hicolor/${size}/apps/opendictate.png"
    install -m 644 "$PROJECT_ROOT/src/ui/assets/brand/png/${size}.png" "$TARGET_STAGE/share/icons/hicolor/${size}/apps/io.github.hkumarsaikia.OpenDictate.png"
done

# 6. Normalize permissions
find "$TARGET_STAGE" -type d -exec chmod 755 {} +
find "$TARGET_STAGE" -type f -exec chmod 644 {} +
chmod 755 "$TARGET_STAGE/opendictate" "$TARGET_STAGE/opendictate-avx2" "$TARGET_STAGE/opendictate.sh"

# 7. Create tar.gz and tar.xz archives
TAR_GZ="$DIST_DIR/${PKG_DIR_NAME}.tar.gz"
TAR_XZ="$DIST_DIR/${PKG_DIR_NAME}.tar.xz"

echo "Creating $TAR_GZ..."
tar -C "$STAGE_DIR" -czf "$TAR_GZ" "$PKG_DIR_NAME"

echo "Creating $TAR_XZ..."
tar -C "$STAGE_DIR" -cJf "$TAR_XZ" "$PKG_DIR_NAME"

echo "=== Portable Tarballs Generated Successfully ==="
ls -lh "$TAR_GZ" "$TAR_XZ"
