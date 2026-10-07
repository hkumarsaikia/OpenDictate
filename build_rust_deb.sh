#!/usr/bin/env bash
set -eo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
VERSION="$(grep -m1 '^version = ' "$PROJECT_ROOT/Cargo.toml" | cut -d '"' -f2 2>/dev/null || echo "2.0.0")"
[ -z "$VERSION" ] && VERSION="2.0.0"
ARCH="$(dpkg --print-architecture 2>/dev/null || echo "amd64")"
DIST_DIR="$PROJECT_ROOT/dist"

VARIANT="universal"

while [[ $# -gt 0 ]]; do
    case "$1" in
        --variant)
            if [ -z "${2:-}" ]; then
                echo "Error: --variant requires an argument (universal, avx2, or all)" >&2
                exit 1
            fi
            VARIANT="$2"
            shift 2
            ;;
        --variant=*)
            VARIANT="${1#*=}"
            shift 1
            ;;
        -h|--help)
            echo "Usage: $0 [--variant <universal|avx2|all>]"
            echo ""
            echo "Options:"
            echo "  --variant universal   Build package without AVX2/FMA for universal x86_64 compatibility (default)"
            echo "  --variant avx2        Build package with AVX2 and FMA acceleration enabled"
            echo "  --variant all         Build both universal and avx2 packages sequentially"
            echo "  -h, --help            Show this help message"
            exit 0
            ;;
        *)
            echo "Error: Unknown argument '$1'" >&2
            echo "Usage: $0 [--variant <universal|avx2|all>]" >&2
            exit 1
            ;;
    esac
done

build_variant() {
    local variant="$1"
    local variant_cmake_args=""
    local variant_desc=""

    case "$variant" in
        universal)
            variant_desc="universal compatibility without AVX2/FMA hardware requirements"
            export GGML_NATIVE=OFF
            export GGML_AVX=OFF
            export GGML_AVX2=OFF
            export GGML_AVX512=OFF
            export GGML_FMA=OFF
            ;;
        avx2)
            variant_desc="optimized with AVX2 and FMA vector acceleration"
            export GGML_NATIVE=OFF
            export GGML_AVX=ON
            export GGML_AVX2=ON
            export GGML_AVX512=OFF
            export GGML_FMA=ON
            ;;
        *)
            echo "Error: Unknown variant '$variant'. Must be 'universal' or 'avx2'." >&2
            exit 1
            ;;
    esac

    local package_name="opendictate_${VERSION}_${variant}_${ARCH}"
    local deb_file="$DIST_DIR/${package_name}.deb"
    local build_dir="$PROJECT_ROOT/build/deb_${variant}"

    echo ""
    echo "======================================================================"
    echo "=== Building Debian Package: $package_name.deb ==="
    echo "=== Hardware Variant: $variant ($variant_desc) ==="
    echo "=== GGML Flags: NATIVE=$GGML_NATIVE AVX=$GGML_AVX AVX2=$GGML_AVX2 FMA=$GGML_FMA ==="
    echo "======================================================================"

    # 1. Clean native Whisper artifacts to ensure variant flags take full effect
    echo "Cleaning whisper-rs-sys, whisper-rs, and opendictate release artifacts..."
    cargo clean --release -p whisper-rs-sys -p whisper-rs -p opendictate --manifest-path "$PROJECT_ROOT/Cargo.toml"

    # 2. Compile native Rust release binary
    echo "Compiling release binary with Cargo (GGML_NATIVE=$GGML_NATIVE GGML_AVX2=$GGML_AVX2)..."
    cargo build --release --manifest-path "$PROJECT_ROOT/Cargo.toml"

    local release_bin="$PROJECT_ROOT/target/release/opendictate"
    if [ ! -f "$release_bin" ]; then
        echo "Error: Release binary not found at $release_bin" >&2
        exit 1
    fi

    echo "Stripping symbols from release binary..."
    strip "$release_bin"

    # 3. Stage packaging filesystem
    echo "Staging Debian package hierarchy in $build_dir..."
    rm -rf "$build_dir"
    mkdir -p "$DIST_DIR"
    mkdir -p "$build_dir/DEBIAN"
    mkdir -p "$build_dir/usr/bin"
    mkdir -p "$build_dir/usr/share/applications"
    mkdir -p "$build_dir/usr/share/metainfo"
    mkdir -p "$build_dir/usr/share/icons/hicolor/scalable/apps"

    # Install native executable
    install -m 755 "$release_bin" "$build_dir/usr/bin/opendictate"

    # Install FreeDesktop desktop entries (both reverse-DNS and legacy names)
    if [ -f "$PROJECT_ROOT/data/io.github.opendictate.OpenDictate.desktop" ]; then
        install -m 644 "$PROJECT_ROOT/data/io.github.opendictate.OpenDictate.desktop" "$build_dir/usr/share/applications/io.github.opendictate.OpenDictate.desktop"
        ln -sf io.github.opendictate.OpenDictate.desktop "$build_dir/usr/share/applications/opendictate.desktop"
    else
        cat << 'EOF' > "$build_dir/usr/share/applications/opendictate.desktop"
[Desktop Entry]
Name=OpenDictate
GenericName=Voice Dictation & Meeting Intelligence
Comment=Pure Rust Linux Voice Dictation, Meeting Transcriber, and AI Text Enhancement
Exec=/usr/bin/opendictate %F
Icon=opendictate
Terminal=false
Type=Application
Categories=Utility;AudioVideo;Accessibility;
Keywords=dictation;speech;voice;transcription;ai;groq;nvidia;gemini;
StartupNotify=true
StartupWMClass=opendictate
Actions=MiniBar;Toggle;

[Desktop Action MiniBar]
Name=Open Floating Mini-Bar
Exec=/usr/bin/opendictate --minibar

[Desktop Action Toggle]
Name=Toggle Dictation
Exec=/usr/bin/opendictate --toggle
EOF
        chmod 644 "$build_dir/usr/share/applications/opendictate.desktop"
    fi

    # Install AppStream metainfo
    if [ -f "$PROJECT_ROOT/data/io.github.opendictate.OpenDictate.metainfo.xml" ]; then
        echo "Installing AppStream metainfo..."
        install -m 644 "$PROJECT_ROOT/data/io.github.opendictate.OpenDictate.metainfo.xml" "$build_dir/usr/share/metainfo/io.github.opendictate.OpenDictate.metainfo.xml"
    else
        echo "Warning: AppStream metainfo not found at $PROJECT_ROOT/data/io.github.opendictate.OpenDictate.metainfo.xml" >&2
    fi

    # Install brand icons (SVG + multi-res PNG suite)
    echo "Installing application icons..."
    install -m 644 "$PROJECT_ROOT/src/ui/assets/brand/opendictate.svg" "$build_dir/usr/share/icons/hicolor/scalable/apps/opendictate.svg"
    install -m 644 "$PROJECT_ROOT/src/ui/assets/brand/opendictate.svg" "$build_dir/usr/share/icons/hicolor/scalable/apps/io.github.opendictate.OpenDictate.svg"

    for size in 16x16 24x24 32x32 48x48 64x64 128x128 256x256 512x512; do
        local png_file="$PROJECT_ROOT/src/ui/assets/brand/png/${size}.png"
        if [ -f "$png_file" ]; then
            mkdir -p "$build_dir/usr/share/icons/hicolor/${size}/apps"
            install -m 644 "$png_file" "$build_dir/usr/share/icons/hicolor/${size}/apps/opendictate.png"
            install -m 644 "$png_file" "$build_dir/usr/share/icons/hicolor/${size}/apps/io.github.opendictate.OpenDictate.png"
        else
            echo "Error: Missing brand PNG icon for $size: $png_file" >&2
            exit 1
        fi
    done

    # 4. Create DEBIAN/control
    cat << EOF > "$build_dir/DEBIAN/control"
Package: opendictate
Version: ${VERSION}
Section: utils
Priority: optional
Architecture: ${ARCH}
Maintainer: H. K. Saikia <https://github.com/hkumarsaikia/OpenDictate>
Depends: libgtk-4-1, libadwaita-1-0, libasound2t64 | libasound2
Description: Voice dictation and AI speech-to-text application for Linux
 OpenDictate is a native Linux voice dictation and AI speech-to-text
 desktop application built with Rust, GTK4, and Libadwaita. It combines
 private offline Whisper transcription with multi-provider Cloud AI models
 to turn spoken voice into clean, polished text.
 Hardware variant: ${variant} (${variant_desc}).
EOF

    # 5. Create DEBIAN/postinst
    cat << 'EOF' > "$build_dir/DEBIAN/postinst"
#!/usr/bin/env bash
set -e

if command -v update-desktop-database >/dev/null 2>&1; then
    update-desktop-database -q /usr/share/applications || true
fi

if command -v gtk-update-icon-cache >/dev/null 2>&1; then
    gtk-update-icon-cache -q -t -f /usr/share/icons/hicolor || true
fi

exit 0
EOF
    chmod 755 "$build_dir/DEBIAN/postinst"

    # 6. Create DEBIAN/postrm
    cat << 'EOF' > "$build_dir/DEBIAN/postrm"
#!/usr/bin/env bash
set -e

if command -v update-desktop-database >/dev/null 2>&1; then
    update-desktop-database -q /usr/share/applications || true
fi

if command -v gtk-update-icon-cache >/dev/null 2>&1; then
    gtk-update-icon-cache -q -t -f /usr/share/icons/hicolor || true
fi

exit 0
EOF
    chmod 755 "$build_dir/DEBIAN/postrm"

    # 7. Build Debian Package
    echo "Packaging into $deb_file..."
    dpkg-deb --build --root-owner-group "$build_dir" "$deb_file"

    # Clean temporary build directory
    rm -rf "$build_dir"

    # 8. Manage backwards-compatible symlink for universal variant
    if [ "$variant" = "universal" ]; then
        local symlink_file="$DIST_DIR/opendictate_${VERSION}_${ARCH}.deb"
        echo "Creating backwards-compatible symlink: $symlink_file -> $package_name.deb"
        (cd "$DIST_DIR" && ln -sf "$package_name.deb" "opendictate_${VERSION}_${ARCH}.deb")
    fi

    echo "=== Build Complete for $variant! ==="
    echo "Package generated at: $deb_file"
    echo ""
    echo "Package contents verification ($deb_file):"
    dpkg -c "$deb_file"
    echo ""
}

case "$VARIANT" in
    universal)
        build_variant "universal"
        ;;
    avx2)
        build_variant "avx2"
        # If universal package already exists, ensure symlink exists
        if [ -f "$DIST_DIR/opendictate_${VERSION}_universal_${ARCH}.deb" ]; then
            (cd "$DIST_DIR" && ln -sf "opendictate_${VERSION}_universal_${ARCH}.deb" "opendictate_${VERSION}_${ARCH}.deb")
        fi
        ;;
    all)
        build_variant "universal"
        build_variant "avx2"
        ;;
    *)
        echo "Error: Unknown variant '$VARIANT'. Must be 'universal', 'avx2', or 'all'." >&2
        exit 1
        ;;
esac
