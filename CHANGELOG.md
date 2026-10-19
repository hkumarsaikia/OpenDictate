# Changelog

All notable changes to OpenDictate are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.0.3] - 2026-10-18

### Added
- **Multi-Format Linux Packaging**: Official packaging across Debian (`.deb`), Red Hat/Fedora/openSUSE (`.rpm`), AppImage (`.AppImage`), Canonical Snap (`.snap`), Flatpak (`.flatpak`), and portable tarballs (`.tar.gz`, `.tar.xz`).
- **Reproducible RPM Packaging**: Added `packaging/rpm/opendictate.spec` and `packaging/rpm/build_rpm.sh` with manual pages (`opendictate.1.gz`, `opendictate-avx2.1.gz`) and soname library auto-resolution.
- **Reproducible Portable Tarballs**: Added `packaging/tarball/build_tarball.sh` generating self-contained distribution tarballs with runtime shell launcher (`opendictate.sh`).
- **CycloneDX Software Bill of Materials (SBOM)**: Generated and schema-validated `opendictate-2.0.3-sbom.cdx.json` via `cargo-cyclonedx` and `cyclonedx-cli`.
- **Supply Chain Security**: Added `deny.toml` configuring cargo-deny for license, ban, and advisory auditing.

### Changed
- **Canonical Reverse-DNS Identity**: Consolidated application namespace strictly to `io.github.hkumarsaikia.OpenDictate` across desktop entries, AppStream metadata, D-Bus interfaces, and sandbox manifests.
- **Canonical Single Debian Package**: Unified Debian distribution under `opendictate_2.0.3_amd64.deb` featuring internal dual-binary CPU dispatch (`universal` SSE2 launcher and AVX2 vector sibling).
- **Security Updates**: Updated `rustls` dependency to `0.23.46` to resolve upstream advisory, with corresponding offline crate manifest synchronization in `cargo-sources.json`.
- **Packaging Documentation**: Updated `README.md` and `docs/PACKAGING.md` to reflect unified installation workflows and release verification.

### Fixed
- **System Tray in Flatpak**: Enhanced system tray integration with unique StatusNotifierItem D-Bus name registration (`org.kde.StatusNotifierItem-1-1`).
- **AppStream Validation**: Pinned screenshot assets to immutable release tags with required descriptive captions, passing `appstreamcli validate` with zero errors.

---

## [2.0.2] - 2026-10-13

### Changed
- **Desktop Entry Consolidation**: Resolved duplicate launcher entries in GNOME Shell application grid by installing strictly canonical `io.github.hkumarsaikia.OpenDictate.desktop`.
- **Quicklist Actions**: Maintained desktop Quicklist actions for opening floating MiniBar and toggling dictation.

### Fixed
- **Upgrade Path**: Ensured clean upgrade removal of obsolete package-owned symlinks.

---

## [2.0.1] - 2026-10-12

### Added
- **Dual-Binary CPU Dispatch**: Introduced portable baseline (`x86-64-v1` SSE2) with transparent runtime dispatch to accelerated `AVX2 + FMA + F16C + BMI2` vector engine.
- **Microphone Routing**: Hardware-aware audio routing prioritizing internal microphones over silent Bluetooth profiles.

### Changed
- **Audio Processing**: Zero-copy 16 kHz audio sample processing for local Whisper streaming transcription.
- **Runtime Alignment**: GNOME 51 runtime and Freedesktop SDK 26.08 packaging alignment.

---

## [2.0.0] - 2026-10-07

### Added
- **Pure Rust Rewrite**: Complete desktop application rewrite in 100% pure Rust using GTK 4.12+, Libadwaita 1.5+, and Relm4 0.9.
- **Dual-Engine Speech Recognition**: Offline on-device `whisper.cpp` speech recognition paired with multi-provider Cloud AI transcription (Groq, Gemini, OpenAI, Claude, Mistral, Ollama, and more).
- **Floating MiniBar Overlay**: Compact recording pill with 60 FPS acoustic waveform visualization, tone presets, and live transcript drawer.
- **28-Language Localization**: Real-time interface localization across 28 desktop languages with RTL text direction support.
- **In-Popover Dictation History**: Persistent SQLite dictation history (`history.db`) with one-click clipboard copying and record deletion.
- **Automated Crash Recovery**: Built-in panic hook with privacy-sanitized diagnostics dialog.
