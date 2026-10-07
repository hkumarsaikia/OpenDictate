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
        runtime_version, "48",
        "Flatpak runtime-version must be '48'"
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
