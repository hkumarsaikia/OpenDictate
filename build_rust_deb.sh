#!/usr/bin/env bash
set -eo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
VERSION="$(grep -m1 '^version = ' "$PROJECT_ROOT/Cargo.toml" | cut -d '"' -f2 2>/dev/null || echo "2.0.0")"
[ -z "$VERSION" ] && VERSION="2.0.0"
ARCH="$(dpkg --print-architecture 2>/dev/null || echo "amd64")"
DIST_DIR="$PROJECT_ROOT/dist"

VARIANT="universal"
REBUILD=0

while [[ $# -gt 0 ]]; do
    case "$1" in
        --variant)
            if [ -z "${2:-}" ]; then
                echo "Error: --variant requires an argument (universal, portable, avx2, or all)" >&2
                exit 1
            fi
            VARIANT="$2"
            shift 2
            ;;
        --variant=*)
            VARIANT="${1#*=}"
            shift 1
            ;;
        --rebuild)
            REBUILD=1
            shift 1
            ;;
        -h|--help)
            echo "Usage: $0 [--rebuild] [--variant <universal|portable|avx2|all>]"
            echo ""
            echo "Options:"
            echo "  --rebuild             Force a fresh build in isolated staging directories without reusing cached target artifacts"
            echo "  --variant universal   Build dual-binary package (portable SSE2 launcher + AVX2 sibling) (default)"
            echo "  --variant portable    Build single-binary package with portable x86-64-v1 (SSE2) baseline only"
            echo "  --variant avx2        Build single-binary package with AVX2/FMA/F16C/BMI2/SSE4.2 acceleration only"
            echo "  --variant all         Build universal, portable, and avx2 packages sequentially"
            echo "  -h, --help            Show this help message"
            exit 0
            ;;
        *)
            echo "Error: Unknown argument '$1'" >&2
            echo "Usage: $0 [--rebuild] [--variant <universal|portable|avx2|all>]" >&2
            exit 1
            ;;
    esac
done

if [ "$REBUILD" -eq 1 ]; then
    REBUILD_TARGET_ROOT="$PROJECT_ROOT/build/deb-rebuild-targets"
    PORTABLE_TARGET_DIR="$REBUILD_TARGET_ROOT/portable"
    AVX2_TARGET_DIR="$REBUILD_TARGET_ROOT/avx2"
else
    PORTABLE_TARGET_DIR="$PROJECT_ROOT/target/portable"
    AVX2_TARGET_DIR="$PROJECT_ROOT/target/avx2"
fi

PORTABLE_BIN="$PORTABLE_TARGET_DIR/release/opendictate"
AVX2_BIN="$AVX2_TARGET_DIR/release/opendictate"
PORTABLE_BUILT_IN_RUN=0
AVX2_BUILT_IN_RUN=0

build_portable_binary() {
    local portable_target="$PORTABLE_TARGET_DIR"
    if [ "$PORTABLE_BUILT_IN_RUN" -eq 1 ] && [ -f "$PORTABLE_BIN" ]; then
        echo "Using portable (x86-64-v1 / SSE2) release binary compiled in this run at $PORTABLE_BIN"
        return 0
    fi
    if [ "$REBUILD" -eq 1 ]; then
        echo "Cleaning isolated portable rebuild staging directory at $portable_target..."
        rm -rf "$portable_target"
    fi
    echo "Compiling portable (x86-64-v1 / SSE2) release binary into $portable_target..."
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
        cargo build --release --manifest-path "$PROJECT_ROOT/Cargo.toml" --target-dir "$portable_target"
    PORTABLE_BUILT_IN_RUN=1
}

build_avx2_binary() {
    local avx2_target="$AVX2_TARGET_DIR"
    if [ "$AVX2_BUILT_IN_RUN" -eq 1 ] && [ -f "$AVX2_BIN" ]; then
        echo "Using optimized (AVX2 + FMA + F16C + BMI2 + SSE4.2) release binary compiled in this run at $AVX2_BIN"
        return 0
    fi
    if [ "$REBUILD" -eq 1 ]; then
        echo "Cleaning isolated AVX2 rebuild staging directory at $avx2_target..."
        rm -rf "$avx2_target"
    fi
    echo "Compiling optimized (AVX2 + FMA + F16C + BMI2 + SSE4.2) release binary into $avx2_target..."
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
        cargo build --release --manifest-path "$PROJECT_ROOT/Cargo.toml" --target-dir "$avx2_target"
    AVX2_BUILT_IN_RUN=1
}

build_variant() {
    local variant="$1"
    local variant_desc=""

    case "$variant" in
        universal)
            variant_desc="dual-binary universal x86_64 compatibility (SSE2 baseline launcher + runtime AVX2 dispatch)"
            ;;
        portable)
            variant_desc="portable x86-64-v1 (SSE2) single-binary baseline without AVX2/FMA requirements"
            ;;
        avx2)
            variant_desc="optimized single-binary with AVX2, FMA, F16C, BMI2, and SSE4.2 vector acceleration"
            ;;
        *)
            echo "Error: Unknown variant '$variant'. Must be 'universal', 'portable', or 'avx2'." >&2
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
    echo "======================================================================"

    # 1. Stage packaging filesystem
    echo "Staging Debian package hierarchy in $build_dir..."
    rm -rf "$build_dir"
    mkdir -p "$DIST_DIR"
    mkdir -p "$build_dir/DEBIAN"
    mkdir -p "$build_dir/usr/bin"
    mkdir -p "$build_dir/usr/share/applications"
    mkdir -p "$build_dir/usr/share/metainfo"
    mkdir -p "$build_dir/usr/share/icons/hicolor/scalable/apps"

    # 2. Compile and install native executable(s) from isolated target directories
    if [ "$variant" = "universal" ]; then
        build_portable_binary
        build_avx2_binary
        install -m 755 "$PORTABLE_BIN" "$build_dir/usr/bin/opendictate"
        install -m 755 "$AVX2_BIN" "$build_dir/usr/bin/opendictate-avx2"
        strip "$build_dir/usr/bin/opendictate" "$build_dir/usr/bin/opendictate-avx2"
    elif [ "$variant" = "portable" ]; then
        build_portable_binary
        install -m 755 "$PORTABLE_BIN" "$build_dir/usr/bin/opendictate"
        strip "$build_dir/usr/bin/opendictate"
    else
        build_avx2_binary
        install -m 755 "$AVX2_BIN" "$build_dir/usr/bin/opendictate"
        strip "$build_dir/usr/bin/opendictate"
    fi


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
Depends: libgtk-4-1, libadwaita-1-0, libasound2t64 | libasound2, wireplumber | pulseaudio-utils
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
    portable)
        build_variant "portable"
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
        build_variant "portable"
        build_variant "avx2"
        ;;
    *)
        echo "Error: Unknown variant '$VARIANT'. Must be 'universal', 'portable', 'avx2', or 'all'." >&2
        exit 1
        ;;
esac
