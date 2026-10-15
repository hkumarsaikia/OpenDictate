#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BUILD_SCRIPT="$REPO_ROOT/build_rust_deb.sh"
PORTABLE_BIN="$REPO_ROOT/target/portable/release/opendictate"
AVX2_BIN="$REPO_ROOT/target/avx2/release/opendictate"

echo "=== OpenDictate Debian Packaging Integration Validator ==="

# 1. Verify required system tools
if ! command -v dpkg-deb >/dev/null 2>&1; then
    echo "Error: 'dpkg-deb' is required to assemble and inspect Debian package archives." >&2
    echo "Please install dpkg-dev (or dpkg) on your system before running this validator." >&2
    exit 1
fi

# 2. Verify packaging script exists
if [ ! -x "$BUILD_SCRIPT" ] && [ ! -f "$BUILD_SCRIPT" ]; then
    echo "Error: Packaging script missing at $BUILD_SCRIPT" >&2
    exit 1
fi

# 3. Verify prebuilt binaries are available (do not trigger unexpected rebuilds)
if [ ! -f "$PORTABLE_BIN" ]; then
    echo "Error: Prebuilt portable release binary not found at:" >&2
    echo "  $PORTABLE_BIN" >&2
    echo "To compile it without modifying production artifacts, run:" >&2
    echo "  cargo build --release --target-dir target/portable" >&2
    exit 1
fi

if [ ! -f "$AVX2_BIN" ]; then
    echo "Error: Prebuilt AVX2 release binary not found at:" >&2
    echo "  $AVX2_BIN" >&2
    echo "To compile it without modifying production artifacts, run:" >&2
    echo "  cargo build --release --target-dir target/avx2" >&2
    exit 1
fi

# 4. Create isolated temporary output directory with automatic trap cleanup
TEMP_DIST="$(mktemp -d /tmp/opendictate-deb-verify-XXXXXX)"
cleanup() {
    if [ -d "$TEMP_DIST" ]; then
        rm -rf "$TEMP_DIST"
    fi
}
trap cleanup EXIT INT TERM

echo "Using isolated output directory: $TEMP_DIST"

# 5. Build all three Debian variants into the isolated directory
echo "Executing build_rust_deb.sh --variant all with DIST_DIR=$TEMP_DIST..."
DIST_DIR="$TEMP_DIST" "$BUILD_SCRIPT" --variant all > /dev/null

# 6. Inspect each generated Debian archive
VARIANTS=("universal" "portable" "avx2")
for variant in "${VARIANTS[@]}"; do
    deb_file=$(find "$TEMP_DIST" -maxdepth 1 -name "opendictate_*_${variant}_*.deb" | head -n 1)
    
    if [ -z "$deb_file" ] || [ ! -f "$deb_file" ]; then
        echo "FAIL: Expected Debian package for variant '$variant' was not generated in $TEMP_DIST" >&2
        exit 1
    fi

    echo "Verifying package contents: $(basename "$deb_file")..."
    listing=$(dpkg-deb -c "$deb_file")

    # Check 1: Assert canonical launcher is present
    if echo "$listing" | grep -q "usr/share/applications/io.github.hkumarsaikia.OpenDictate.desktop"; then
        echo "  [OK] Canonical launcher present: usr/share/applications/io.github.hkumarsaikia.OpenDictate.desktop"
    else
        echo "  [FAIL] Missing canonical launcher in $(basename "$deb_file")!" >&2
        exit 1
    fi

    # Check 2: Assert redundant legacy launcher/symlink is absent
    if echo "$listing" | grep -q "usr/share/applications/opendictate.desktop"; then
        echo "  [FAIL] Redundant legacy launcher found in $(basename "$deb_file")!" >&2
        exit 1
    else
        echo "  [OK] Redundant legacy launcher absent: usr/share/applications/opendictate.desktop"
    fi

    # Check 3: Assert EXACTLY ONE .desktop file exists directly under usr/share/applications/
    desktop_entries=()
    while IFS= read -r entry; do
        if [ -n "$entry" ]; then
            desktop_entries+=("$entry")
        fi
    done < <(echo "$listing" | grep -oE '\.?/usr/share/applications/[^/[:space:]]+\.desktop' | sed -E 's|^\./|/|' | sort -u)

    desktop_count="${#desktop_entries[@]}"
    if [ "$desktop_count" -ne 1 ]; then
        echo "  [FAIL] Variant '$variant' must contain exactly 1 desktop launcher directly in usr/share/applications/, found $desktop_count:" >&2
        for entry in "${desktop_entries[@]}"; do
            echo "    - $entry" >&2
        done
        exit 1
    fi

    if [ "${desktop_entries[0]}" != "/usr/share/applications/io.github.hkumarsaikia.OpenDictate.desktop" ]; then
        echo "  [FAIL] Expected only '/usr/share/applications/io.github.hkumarsaikia.OpenDictate.desktop' in variant '$variant', found: '${desktop_entries[0]}'" >&2
        exit 1
    fi

    echo "  [OK] Exactly 1 desktop launcher verified in usr/share/applications/: ${desktop_entries[0]}"
done

echo ""
echo "=== All 3 Debian package variants successfully verified! ==="
echo "Canonical launcher retained, redundant launcher removed, exactly 1 desktop launcher per variant, and no production directories modified."
