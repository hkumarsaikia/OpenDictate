use std::fs;
use std::path::PathBuf;

fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Helper to extract simple top-level key-value pairs from YAML content.
fn extract_top_level_yaml_value(content: &str, key: &str) -> Option<String> {
    for line in content.lines() {
        if line.starts_with(' ') || line.starts_with('\t') {
            continue;
        }
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix(key)
            && let Some(val) = rest.strip_prefix(':')
        {
            let clean_val = val.split('#').next().unwrap_or(val).trim();
            let v = clean_val.trim_matches('\'').trim_matches('"');
            return Some(v.to_string());
        }
    }
    None
}

/// Helper to extract list items under a specific YAML key (e.g. `finish-args:` or `plugs:`).
fn extract_yaml_list_under_key(content: &str, key: &str) -> Vec<String> {
    let mut collecting = false;
    let mut items = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with('#') {
            continue;
        }

        if !collecting {
            if trimmed == format!("{}:", key) || trimmed.starts_with(&format!("{}:", key)) {
                collecting = true;
            }
        } else if let Some(stripped) = trimmed.strip_prefix("- ") {
            let raw_item = stripped.trim();
            let clean_item = raw_item.split('#').next().unwrap_or(raw_item).trim();
            let item = clean_item.trim_matches('\'').trim_matches('"');
            items.push(item.to_string());
        } else if !trimmed.is_empty() && !trimmed.starts_with('-') {
            break;
        }
    }

    items
}

#[test]
fn test_flatpak_manifest_core_attributes() {
    let flatpak_path =
        project_root().join("packaging/flatpak/io.github.opendictate.OpenDictate.yml");
    assert!(
        flatpak_path.exists(),
        "Flatpak manifest must exist at packaging/flatpak/io.github.opendictate.OpenDictate.yml"
    );

    let content = fs::read_to_string(&flatpak_path).expect("Failed to read Flatpak manifest");

    let app_id = extract_top_level_yaml_value(&content, "app-id")
        .expect("Missing app-id in Flatpak manifest");
    assert_eq!(
        app_id, "io.github.opendictate.OpenDictate",
        "Flatpak app-id must be io.github.opendictate.OpenDictate"
    );

    let runtime = extract_top_level_yaml_value(&content, "runtime")
        .expect("Missing runtime in Flatpak manifest");
    assert_eq!(
        runtime, "org.gnome.Platform",
        "Flatpak runtime must be org.gnome.Platform"
    );

    let runtime_version = extract_top_level_yaml_value(&content, "runtime-version")
        .expect("Missing runtime-version in Flatpak manifest");
    assert_eq!(
        runtime_version, "51",
        "Flatpak runtime-version must be '51'"
    );

    let sdk =
        extract_top_level_yaml_value(&content, "sdk").expect("Missing sdk in Flatpak manifest");
    assert_eq!(sdk, "org.gnome.Sdk", "Flatpak sdk must be org.gnome.Sdk");

    let command = extract_top_level_yaml_value(&content, "command")
        .expect("Missing command in Flatpak manifest");
    assert_eq!(
        command, "opendictate",
        "Flatpak command must be opendictate"
    );

    let sdk_extensions = extract_yaml_list_under_key(&content, "sdk-extensions");
    assert!(
        sdk_extensions.contains(&"org.freedesktop.Sdk.Extension.rust-stable".to_string()),
        "Flatpak sdk-extensions must include org.freedesktop.Sdk.Extension.rust-stable"
    );
    assert!(
        sdk_extensions.contains(&"org.freedesktop.Sdk.Extension.llvm22".to_string()),
        "Flatpak sdk-extensions must include org.freedesktop.Sdk.Extension.llvm22"
    );
    assert!(
        content.contains("/usr/lib/sdk/llvm22/bin")
            && content.contains("/usr/lib/sdk/llvm22/lib")
            && !content.contains("llvm18"),
        "Flatpak manifest must reference /usr/lib/sdk/llvm22/bin and /usr/lib/sdk/llvm22/lib without legacy llvm18 paths"
    );
}

#[test]
fn test_flatpak_manifest_finish_args() {
    let flatpak_path =
        project_root().join("packaging/flatpak/io.github.opendictate.OpenDictate.yml");
    let content = fs::read_to_string(&flatpak_path).expect("Failed to read Flatpak manifest");

    let finish_args = extract_yaml_list_under_key(&content, "finish-args");
    assert!(
        !finish_args.is_empty(),
        "finish-args must not be empty in Flatpak manifest"
    );

    // Required Flathub-compliant permissions
    let required_args = [
        "--share=ipc",
        "--socket=wayland",
        "--socket=fallback-x11",
        "--device=dri",
        "--socket=pulseaudio",
        "--share=network",
        "--talk-name=org.kde.StatusNotifierWatcher",
        "--filesystem=xdg-data/opendictate:create",
        "--filesystem=xdg-config/opendictate:create",
    ];

    for &arg in &required_args {
        assert!(
            finish_args.iter().any(|a| a == arg),
            "Flatpak finish-args must contain '{}'. Found: {:?}",
            arg,
            finish_args
        );
    }

    // Ensure disallowed Flathub linter flags are absent
    assert!(!finish_args.iter().any(|a| a == "--socket=x11"));
    assert!(!finish_args.iter().any(|a| a == "--device=all"));
    assert!(!finish_args.iter().any(|a| a == "--filesystem=home"));
}

#[test]
fn test_snapcraft_manifest_core_attributes() {
    let snap_path = project_root().join("snap/snapcraft.yaml");
    assert!(
        snap_path.exists(),
        "Snapcraft manifest must exist at snap/snapcraft.yaml"
    );

    let content = fs::read_to_string(&snap_path).expect("Failed to read Snapcraft manifest");

    let name =
        extract_top_level_yaml_value(&content, "name").expect("Missing name in Snapcraft manifest");
    assert_eq!(name, "opendictate", "Snap name must be opendictate");

    let base =
        extract_top_level_yaml_value(&content, "base").expect("Missing base in Snapcraft manifest");
    assert_eq!(base, "core24", "Snap base must be core24");

    let confinement = extract_top_level_yaml_value(&content, "confinement")
        .expect("Missing confinement in Snapcraft manifest");
    assert_eq!(confinement, "strict", "Snap confinement must be strict");

    let version = extract_top_level_yaml_value(&content, "version")
        .expect("Missing version in Snapcraft manifest");
    assert_eq!(version, "2.0.0", "Snap version must be 2.0.0");
}

#[test]
fn test_snapcraft_manifest_plugs() {
    let snap_path = project_root().join("snap/snapcraft.yaml");
    let content = fs::read_to_string(&snap_path).expect("Failed to read Snapcraft manifest");

    let plugs = extract_yaml_list_under_key(&content, "plugs");
    assert!(
        !plugs.is_empty(),
        "plugs must not be empty in Snapcraft manifest"
    );

    let required_plugs = [
        "audio-record",
        "audio-playback",
        "network",
        "desktop",
        "desktop-legacy",
        "wayland",
        "x11",
    ];

    for &plug in &required_plugs {
        assert!(
            plugs.iter().any(|p| p == plug),
            "Snapcraft manifest plugs must include '{}'. Found: {:?}",
            plug,
            plugs
        );
    }

    assert_eq!(
        plugs, required_plugs,
        "Snapcraft plugs list must exactly match the required specification"
    );
}

#[test]
fn test_appstream_metainfo_metadata() {
    let metainfo_path = project_root().join("data/io.github.opendictate.OpenDictate.metainfo.xml");
    assert!(
        metainfo_path.exists(),
        "AppStream metainfo must exist at data/io.github.opendictate.OpenDictate.metainfo.xml"
    );

    let content = fs::read_to_string(&metainfo_path).expect("Failed to read AppStream metainfo");

    assert!(
        content.contains("<id>io.github.opendictate.OpenDictate</id>"),
        "AppStream metadata must specify ID io.github.opendictate.OpenDictate"
    );
    assert!(
        content.contains("<binary>opendictate</binary>"),
        "AppStream metadata must specify binary 'opendictate'"
    );
    assert!(
        content.contains("<name>OpenDictate</name>"),
        "AppStream metadata must specify name 'OpenDictate'"
    );
    assert!(
        content.contains("<release version=\"2.0.0\""),
        "AppStream metadata must specify release version 2.0.0"
    );

    // Verify all 4 screenshot files referenced in metainfo.xml exist in docs/screenshots/
    for screenshot_file in [
        "docs/screenshots/main-window-settings.png",
        "docs/screenshots/header-menu-popover.png",
        "docs/screenshots/floating-minibar.png",
        "docs/screenshots/main-window-light.png",
    ] {
        assert!(
            project_root().join(screenshot_file).exists(),
            "Screenshot file {} must exist on disk",
            screenshot_file
        );
        assert!(
            content.contains(screenshot_file),
            "AppStream metainfo.xml must reference {}",
            screenshot_file
        );
    }
}

#[test]
fn test_cross_packaging_consistency() {
    let flatpak_path =
        project_root().join("packaging/flatpak/io.github.opendictate.OpenDictate.yml");
    let snap_path = project_root().join("snap/snapcraft.yaml");
    let metainfo_path = project_root().join("data/io.github.opendictate.OpenDictate.metainfo.xml");
    let cargo_path = project_root().join("Cargo.toml");

    let flatpak_content = fs::read_to_string(&flatpak_path).unwrap();
    let snap_content = fs::read_to_string(&snap_path).unwrap();
    let metainfo_content = fs::read_to_string(&metainfo_path).unwrap();
    let cargo_content = fs::read_to_string(&cargo_path).unwrap();

    let expected_app_id = "io.github.opendictate.OpenDictate";
    let expected_binary = "opendictate";
    let expected_version = "2.0.0";

    // App ID consistency
    assert!(flatpak_content.contains(&format!("app-id: {}", expected_app_id)));
    assert!(snap_content.contains(&format!("common-id: {}", expected_app_id)));
    assert!(metainfo_content.contains(&format!("<id>{}</id>", expected_app_id)));

    // Binary name consistency
    assert!(flatpak_content.contains(&format!("command: {}", expected_binary)));
    assert!(snap_content.contains(&format!("name: {}", expected_binary)));
    assert!(snap_content.contains(&format!("command: usr/bin/{}", expected_binary)));
    assert!(metainfo_content.contains(&format!("<binary>{}</binary>", expected_binary)));

    // Version & Rust 2024 edition consistency
    assert!(cargo_content.contains(&format!("version = \"{}\"", expected_version)));
    assert!(cargo_content.contains("edition = \"2024\""));
    assert!(snap_content.contains(&format!("version: '{}'", expected_version)));
    assert!(metainfo_content.contains(&format!("<release version=\"{}\"", expected_version)));
}

#[test]
fn test_legacy_tier_sandbox_rules() {
    let flatpak_path =
        project_root().join("packaging/flatpak/io.github.opendictate.OpenDictate.yml");
    let snap_path = project_root().join("snap/snapcraft.yaml");

    let flatpak_content = fs::read_to_string(&flatpak_path).unwrap();
    let snap_content = fs::read_to_string(&snap_path).unwrap();

    let flatpak_finish_args = extract_yaml_list_under_key(&flatpak_content, "finish-args");
    let snap_plugs = extract_yaml_list_under_key(&snap_content, "plugs");

    // Audio & GPU access
    assert!(flatpak_finish_args.contains(&"--socket=pulseaudio".to_string()));
    assert!(flatpak_finish_args.contains(&"--device=dri".to_string()));
    assert!(snap_plugs.contains(&"audio-record".to_string()));
    assert!(snap_plugs.contains(&"audio-playback".to_string()));

    // Display fallback on X11 and Wayland sessions
    assert!(flatpak_finish_args.contains(&"--socket=wayland".to_string()));
    assert!(flatpak_finish_args.contains(&"--socket=fallback-x11".to_string()));
    assert!(snap_plugs.contains(&"wayland".to_string()));
    assert!(snap_plugs.contains(&"x11".to_string()));
    assert!(snap_plugs.contains(&"desktop-legacy".to_string()));

    // Persistent storage for offline models and history
    assert!(flatpak_finish_args.contains(&"--filesystem=xdg-data/opendictate:create".to_string()));
    assert!(
        flatpak_finish_args.contains(&"--filesystem=xdg-config/opendictate:create".to_string())
    );
}

#[test]
fn test_snapcraft_alsa_pulseaudio_routing() {
    let snap_path = project_root().join("snap/snapcraft.yaml");
    let content = fs::read_to_string(&snap_path).expect("Failed to read snap/snapcraft.yaml");

    assert!(
        content.contains("libasound2-plugins"),
        "Snapcraft manifest must stage libasound2-plugins so cpal/ALSA can route audio through PulseAudio/PipeWire"
    );
    assert!(
        content.contains("pulseaudio-utils"),
        "Snapcraft manifest must stage pulseaudio-utils (pactl) for audio profile detection inside the Snap"
    );
    assert!(
        content.contains("ALSA_CONFIG_PATH"),
        "Snapcraft manifest must set ALSA_CONFIG_PATH so libasound.so.2 routes default PCM to pulse"
    );
    assert!(
        content.contains("/usr/share/alsa:") && content.contains("alsa-lib:"),
        "Snapcraft manifest must bind /usr/share/alsa and alsa-lib via layout:"
    );
}

#[test]
fn test_dual_binary_packaging_and_amd64_platform_scope() {
    let snap_content =
        fs::read_to_string(project_root().join("snap/snapcraft.yaml")).expect("read snapcraft");
    let flatpak_content = fs::read_to_string(
        project_root().join("packaging/flatpak/io.github.opendictate.OpenDictate.yml"),
    )
    .expect("read flatpak");
    let appimage_content =
        fs::read_to_string(project_root().join("packaging/appimage/build_appimage.sh"))
            .expect("read build_appimage.sh");
    let deb_content =
        fs::read_to_string(project_root().join("build_rust_deb.sh")).expect("read build_rust_deb");

    // 1. Snapcraft must declare amd64 only (no untested arm64 claim) and package both binaries
    assert!(
        snap_content.contains("platforms:\n  amd64:\n") && !snap_content.contains("arm64:"),
        "snap/snapcraft.yaml must declare amd64 only without claiming untested arm64 support"
    );
    assert!(
        snap_content.contains("opendictate-avx2") && snap_content.contains("target/avx2"),
        "snap/snapcraft.yaml must build and stage opendictate-avx2 from isolated target/avx2"
    );

    // 2. Flatpak must build both target/portable and target/avx2 and install opendictate + opendictate-avx2
    assert!(
        flatpak_content.contains("target/portable")
            && flatpak_content.contains("target/avx2")
            && flatpak_content.contains("/app/bin/opendictate-avx2"),
        "Flatpak manifest must build isolated portable and avx2 targets and install /app/bin/opendictate-avx2"
    );

    // 3. AppImage must build both target/portable and target/avx2 and install opendictate + opendictate-avx2
    assert!(
        appimage_content.contains("target/portable")
            && appimage_content.contains("target/avx2")
            && appimage_content.contains("usr/bin/opendictate-avx2"),
        "build_appimage.sh must build isolated portable and avx2 targets and install usr/bin/opendictate-avx2"
    );

    // 4. Debian build script must build both target/portable and target/avx2 and install opendictate-avx2 in universal variant
    assert!(
        deb_content.contains("target/portable")
            && deb_content.contains("target/avx2")
            && deb_content.contains("usr/bin/opendictate-avx2"),
        "build_rust_deb.sh must build isolated portable and avx2 targets and install usr/bin/opendictate-avx2"
    );

    // 5. All four packaging configurations must enforce deterministic portable and AVX2 Rust/GGML CPU-tier policies
    let portable_rustflags = "-C target-cpu=x86-64 -C target-feature=-sse3,-ssse3,-sse4.1,-sse4.2,-avx,-avx2,-fma,-f16c,-bmi1,-bmi2,-lzcnt,-popcnt,-movbe";
    let avx2_rustflags = "-C target-cpu=x86-64 -C target-feature=+sse3,+ssse3,+sse4.1,+sse4.2,+avx,+avx2,+fma,+f16c,+bmi2,-bmi1,-lzcnt,-popcnt,-movbe";

    for (name, content) in [
        ("Flatpak manifest", &flatpak_content),
        ("Snapcraft manifest", &snap_content),
        ("AppImage build script", &appimage_content),
        ("Debian build script", &deb_content),
    ] {
        assert!(
            content.contains(portable_rustflags),
            "{name} must specify the portable x86-64-v1 CARGO_BUILD_RUSTFLAGS policy"
        );
        assert!(
            content.contains(avx2_rustflags),
            "{name} must specify the AVX2-tier CARGO_BUILD_RUSTFLAGS policy"
        );
        assert!(
            content.contains("RUSTFLAGS") && content.contains("CARGO_ENCODED_RUSTFLAGS"),
            "{name} must unset inherited RUSTFLAGS and CARGO_ENCODED_RUSTFLAGS"
        );
        for enabled_avx2_flag in [
            "GGML_SSE42=ON",
            "GGML_AVX=ON",
            "GGML_AVX2=ON",
            "GGML_BMI2=ON",
            "GGML_FMA=ON",
            "GGML_F16C=ON",
        ] {
            assert!(
                content.contains(enabled_avx2_flag),
                "{name} must explicitly enable {enabled_avx2_flag} for the AVX2 build"
            );
        }
        for disabled_higher_tier_flag in [
            "GGML_NATIVE=OFF",
            "GGML_AVX_VNNI=OFF",
            "GGML_AVX512=OFF",
            "GGML_AVX512_VBMI=OFF",
            "GGML_AVX512_VNNI=OFF",
            "GGML_AVX512_BF16=OFF",
            "GGML_AMX_TILE=OFF",
            "GGML_AMX_INT8=OFF",
            "GGML_AMX_BF16=OFF",
        ] {
            assert!(
                content.contains(disabled_higher_tier_flag),
                "{name} must explicitly set {disabled_higher_tier_flag}"
            );
        }
        for portable_off_key in [
            "GGML_SSE42",
            "GGML_AVX",
            "GGML_AVX2",
            "GGML_BMI2",
            "GGML_FMA",
            "GGML_F16C",
        ] {
            let env_syntax = format!("{portable_off_key}=OFF");
            let yaml_syntax = format!("{portable_off_key}: \"OFF\"");
            assert!(
                content.contains(&env_syntax) || content.contains(&yaml_syntax),
                "{name} must explicitly disable {portable_off_key} for the portable build"
            );
        }
    }

    // 6. AppImage and Debian scripts must support --rebuild using isolated rebuild staging directories
    assert!(
        appimage_content.contains("--rebuild")
            && appimage_content.contains("appimage-rebuild-targets"),
        "build_appimage.sh must support --rebuild with isolated appimage-rebuild-targets directories"
    );
    assert!(
        deb_content.contains("--rebuild") && deb_content.contains("deb-rebuild-targets"),
        "build_rust_deb.sh must support --rebuild with isolated deb-rebuild-targets directories"
    );
}
