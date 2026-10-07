# OpenDictate

[![Rust](https://img.shields.io/badge/rust-2021%20edition-orange.svg)](https://www.rust-lang.org)
[![GTK4](https://img.shields.io/badge/GTK-4.12+-blue.svg)](https://gtk.org)
[![Libadwaita](https://img.shields.io/badge/Libadwaita-1.5+-purple.svg)](https://gnome.pages.gitlab.gnome.org/libadwaita/)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![i18n: 28 Languages](https://img.shields.io/badge/i18n-28%20Languages-blue.svg)](docs/PACKAGING.md#7-internationalization-i18n--localized-desktops)
[![Flatpak](https://img.shields.io/badge/Flatpak-GNOME%2046-blue)](packaging/flatpak/io.github.opendictate.OpenDictate.yml)
[![Snap](https://img.shields.io/badge/Snap-core24-orange)](packaging/snap/snapcraft.yaml)
[![Debian](https://img.shields.io/badge/Debian-Universal%20%7C%20AVX2-red)](build_rust_deb.sh)
[![Distro Matrix](https://img.shields.io/badge/Distros-Ubuntu%20%7C%20Mint%20%7C%20Debian%20%7C%20Arch%20%7C%20Fedora%20%7C%20Kali%20%7C%20Manjaro%20%7C%20openSUSE%20%7C%20elementary%20%7C%20Zorin%20%7C%20Linux%20Lite%20%7C%20SteamOS-informational)](docs/PACKAGING.md)
[![Compatibility](https://img.shields.io/badge/Compatibility-2018--2026%20Linux%20Releases-success)](docs/PACKAGING.md)

**OpenDictate** is a native Linux voice dictation and AI speech-to-text application. It combines private offline transcription with multi-provider Cloud AI models to turn spoken voice into clean, polished text.

Built with Rust, GTK 4.12+, Libadwaita 1.5+, and Relm4, OpenDictate integrates with both Wayland and X11 desktop environments, providing a floating MiniBar recording overlay, system tray indicator, and dual-engine speech recognition powered by embedded `whisper.cpp` and Cloud AI providers.

---

## Key Features

- **Native GTK4 & Libadwaita Interface:** Built with Relm4, GTK 4.12+, and Libadwaita 1.5+ with adaptive dark and light theme synchronization.
- **Offline On-Device Transcription:** In-process `whisper.cpp` speech engine (`whisper-rs 0.16.0`) supporting quantized GGML models (`tiny`, `base`, `small`, `medium`).
- **Dual-Engine Speech Recognition:** Switch between local offline Whisper models and Cloud AI providers (Groq, Google Gemini, OpenAI, SambaNova, OpenRouter, and Hugging Face) with live audio capability and plan-tier verification.
- **Hardware-Aware Microphone Routing:** Automatically detects connected Bluetooth earphones/headsets (`Handsfree`), wired 3.5mm TRRS / USB headsets (`Headphones`), or internal microphones (`System Default`), supports manual user overrides, and enforces exclusive single-device audio capture.
- **28-Language Interface Localization:** Full desktop UI translated across 28 languages with real-time in-place retranslation (`ui_language` configuration) and Right-to-Left layout support.
- **In-Popover Dictation History:** Header menu with sliding stack navigation, embedded SQLite history storage (`history.db`), full-transcript hover tooltips, clipboard copy, and individual record deletion.
- **Dual-Variant Hardware Support:** Ships both a baseline `universal` x86_64 build (for legacy CPUs without AVX2) and an `avx2` vector-accelerated build (for Intel Haswell 2013+ and AMD Zen 2017+ CPUs).
- **Wayland & X11 Desktop Integration:** XDG Desktop Portal integration, StatusNotifierItem system tray (`ksni`), global shortcuts, and automatic text insertion.
- **Floating MiniBar:** Compact recording overlay (`opendictate --minibar`) with real-time acoustic waveform visualization.
- **Multi-Rate Audio Pipeline:** Automatic sample rate conversion (8 kHz to 192 kHz → 16 kHz mono PCM) using `cpal` and `rubato` with DC offset removal.

---

## Architecture Overview

```text
OpenDictate/
├── Cargo.toml                 # Crate manifest and dependency specifications
├── data/                      # FreeDesktop .desktop entry and AppStream metainfo.xml
├── docs/                      # Packaging and distribution documentation
├── packaging/                 # Flatpak, Snap, and AppImage build manifests
├── src/
│   ├── main.rs                # Application entrypoint and single-instance / CLI dispatch
│   ├── lib.rs                 # Library root exporting core modules
│   ├── cli.rs                 # Command-line argument parser (--minibar, --toggle, etc.)
│   ├── config.rs              # Persistent JSON configuration (~/.config/opendictate/config.json)
│   ├── audio/                 # CPAL audio capture, Rubato 16 kHz resampler, and RMS level meter
│   ├── services/
│   │   ├── ai/                # Local Whisper GGML engine and Cloud AI provider clients
│   │   ├── dictation_worker.rs# Background transcription & text enhancement worker
│   │   ├── hotkey.rs          # XDG GlobalShortcuts portal and desktop hotkey integration
│   │   ├── i18n.rs            # Static 28-language UI translation catalog
│   │   ├── storage.rs         # SQLite persistence for dictation history (history.db)
│   │   └── tray.rs            # D-Bus StatusNotifierItem system tray service
│   └── ui/
│       ├── main_window/       # Libadwaita main window, header menu popover, and settings view
│       ├── mini_bar/          # Floating recording pill, waveform visualizer, and drawer
│       ├── theme.rs           # Adaptive CSS theme compiler and stylesheet loader
│       └── assets/brand/      # Scalable SVG and hicolor PNG application icons
└── tests/                     # Integration, UI, audio, storage, and packaging test suites
```

---

## 28-Language UI Localization & In-Popover History

OpenDictate includes an embedded compile-time translation catalog for global Linux desktop environments:

### Worldwide 28-Language Coverage
Users can switch languages on the fly via the Main Menu popover:
- **In-Place Retranslation:** Changing language immediately updates labels, preferences, tooltips, dialogs, and the floating MiniBar without restarting the application.
- **Supported Locales (28 Languages):** English (`en`), Spanish (`es`), German (`de`), French (`fr`), Simplified Chinese (`zh-CN`), Russian (`ru`), Brazilian Portuguese (`pt-BR`), Japanese (`ja`), Italian (`it`), Polish (`pl`), Dutch (`nl`), Turkish (`tr`), Korean (`ko`), Ukrainian (`uk`), Hindi (`hi`), Arabic (`ar`, with Right-to-Left widget direction), Indonesian (`id`), Vietnamese (`vi`), Traditional Chinese (`zh-TW`), Czech (`cs`), Swedish (`sv`), Hungarian (`hu`), Romanian (`ro`), Greek (`el`), Danish (`da`), Finnish (`fi`), Norwegian Bokmål (`nb`), and Bengali (`bn`).
- **Configuration Storage:** Persisted under `ui_language` in `~/.config/opendictate/config.json`.

### Main Menu & Dictation History Viewer
The top-right header menu uses sliding stack navigation (`gtk4::Stack`):
- **Header Menu Structure:** Contains zoom controls, **Language (→)**, **History (→)**, **Buy me a coffee**, and **About OpenDictate**.
- **In-Popover Dictation History:** Queries the local SQLite database (`~/.local/share/opendictate/history.db`):
  - **Timestamped Transcripts:** Displays multi-word dictation previews with relative timestamps.
  - **Hover Tooltips:** Hovering over any entry displays the full transcript text.
  - **Copy to Clipboard:** Copies the transcript to the system clipboard with visual confirmation.
  - **Delete Records:** Deletes individual entries from SQLite and updates the list immediately.

---

## Installation & Packaging

OpenDictate is distributed across multiple formats to ensure compatibility from older long-term support distributions to cutting-edge rolling releases.

For comprehensive distribution instructions, refer to the **[Universal Packaging Guide](docs/PACKAGING.md)**.

### 1. Debian & Ubuntu (`.deb`)

Download or build the native `.deb` package matching your CPU architecture:

```bash
# Check if your CPU supports AVX2 and FMA
grep -E '(avx2.*fma|fma.*avx2)' /proc/cpuinfo >/dev/null && echo "Use AVX2" || echo "Use Universal"

# Option A: AVX2-accelerated (Intel Haswell 2013+, AMD Zen 2017+)
sudo dpkg -i dist/opendictate_2.0.0_avx2_amd64.deb

# Option B: Universal Baseline (Intel Core 2, Pentium, Celeron, VMs)
sudo dpkg -i dist/opendictate_2.0.0_universal_amd64.deb

# Resolve any missing host dependencies
sudo apt-get install -f -y
```

To build packages from source:
```bash
# Build Universal, AVX2, or both
bash build_rust_deb.sh --variant all
```

> **Note on Debian 12 (Bookworm):** Debian 12 standard repositories provide GTK 4.10 and Libadwaita 1.3, which do not satisfy OpenDictate's GTK 4.12+ and Libadwaita 1.5+ requirements. End users on stock Debian 12 should install via **Flatpak**. Native compilation on Debian 12 requires pinning or overlaying Debian 13 (Trixie) or backports for `libgtk-4-dev` and `libadwaita-1-dev`.

### 2. Flatpak (Flathub)

Recommended for older distributions (Ubuntu 18.04–22.04, Debian 10–12 stock) and immutable operating systems (Fedora Silverblue, SteamOS) because it bundles the GNOME 46 runtime with GTK 4.16+ and Libadwaita 1.6+.

```bash
# Install runtime prerequisites
flatpak install -y flathub org.gnome.Platform//46 org.gnome.Sdk//46

# Build and install locally
flatpak-builder --user --install --force-clean build-flatpak packaging/flatpak/io.github.opendictate.OpenDictate.yml

# Run
flatpak run io.github.opendictate.OpenDictate
```

### 3. Canonical Snap

Available with strict confinement (`base: core24`) and GNOME desktop integration:

```bash
# Build and install local snap
snapcraft --use-lxd
sudo snap install --dangerous opendictate_2.0.0_amd64.snap

# Connect audio capture interface
sudo snap connect opendictate:audio-record
```

### 4. Portable AppImage

Standalone bundle containing desktop integration and icon assets:

```bash
# Generate AppImage or AppDir
bash packaging/appimage/build_appimage.sh

# Run directly
./build/AppDir/AppRun
```

### 5. Build from Source (Cargo)

```bash
# Install dependencies (Ubuntu/Debian)
sudo apt-get install -y build-essential cmake clang pkg-config libssl-dev libasound2-dev libgtk-4-dev libadwaita-1-dev libdbus-1-dev

# Compile release binary
cargo build --release

# Run
./target/release/opendictate
```

*(Note: Debian 12 Bookworm standard repositories provide GTK 4.10 and Libadwaita 1.3; native building requires pinning/overlaying Debian 13 Trixie or backports for GTK 4.12+ and Libadwaita 1.5+, while stock Debian 12 users should install via Flatpak).*

---

## Distribution & Compatibility Matrix (2018–2026)

OpenDictate is engineered with a **dual runtime execution tier** model to guarantee compatibility across all 12 core Linux distributions and every release from 2018 through 2026:

- **Tier 1 (Modern Native Systems: 2023–2026):** On systems where repositories provide GTK 4.12+ and Libadwaita 1.4+/1.5+, OpenDictate runs as a native host binary with zero sandboxing overhead via native packages (`.deb`, RPM, AUR, Portage ebuild) or compiling from source.
- **Tier 2 (Universal Sandboxed Systems: 2018–2022 & Immutable Desktops):** On legacy LTS distributions with older host libraries (GTK 3 or GTK 4.6) and atomic desktops (SteamOS, Fedora Silverblue), OpenDictate runs seamlessly through **Flatpak (`org.gnome.Platform//46`)** or **Snap (`core24`)**. The sandbox bundles GTK 4.16+, Libadwaita 1.6+, and glibc 2.38+, punching secure portals for audio capture and display.

| Distribution Family | Verified Releases (2018–2026) | Execution Tier | Recommended Package Format | Hardware Tier | Matrix Status |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **1. Ubuntu** | `26.04 LTS`, `25.10`, `25.04`, `24.10`, `24.04 LTS`, `23.10` | **Tier 1 (Native)** | Native `.deb` / Snap / Source | AVX2 & Universal | Verified (Podman) |
| | `23.04`, `22.10`, `22.04 LTS`, `21.10`–`20.04 LTS`, `19.10`–`18.04 LTS` | **Tier 2 (Sandbox)** | Flatpak / Snap | Universal & AVX2 | Verified (Sandbox) |
| **2. Linux Mint** | `Mint 22`–`22.3` (Wilma / Xia / Zara) & `LMDE 7` | **Tier 1 (Native)** | Native `.deb` / Flatpak | AVX2 & Universal | Verified (Podman) |
| | `Mint 19`–`19.3`, `20`–`20.3`, `21`–`21.3` & `LMDE 3`–`6` | **Tier 2 (Sandbox)** | Flatpak (Flathub built-in) | Universal & AVX2 | Verified (Sandbox) |
| **3. Debian** | `Debian 13` (Trixie) & `Sid` | **Tier 1 (Native)** | Native `.deb` / Source | AVX2 & Universal | Verified (Podman) |
| | `Debian 12` (Bookworm) | **Tier 1 / Tier 2** | Flatpak (Stock) / Native `.deb` (Trixie overlay) | AVX2 & Universal | Verified (Podman overlay) |
| | `Debian 10` (Buster) & `Debian 11` (Bullseye) | **Tier 2 (Sandbox)** | Flatpak / Snap | Universal & AVX2 | Verified (Sandbox) |
| **4. Arch Linux** | Rolling (`2018.01`–`2026.10`) | **Tier 1 (Native)** | Native AUR / Cargo / Flatpak | AVX2 & Universal | Verified (Podman) |
| **5. Fedora** | `Fedora 39`–`44` Workstation | **Tier 1 (Native)** | Native RPM / Flatpak / Cargo | AVX2 & Universal | Verified (Podman) |
| | `Fedora 28`–`38` & `Fedora Silverblue` | **Tier 2 (Sandbox)** | Flatpak | Universal & AVX2 | Verified (Sandbox) |
| **6. Kali Linux** | Rolling `2023.3`–`2026.3` | **Tier 1 (Native)** | Native `.deb` / Cargo / Flatpak | AVX2 & Universal | Verified (Podman) |
| | Rolling `2018.1`–`2023.2` | **Tier 2 (Sandbox)** | Flatpak | Universal & AVX2 | Verified (Sandbox) |
| **7. Manjaro** | `23.0 Uranos`–`26.0` (`Wynsdey`, `Xahea`, `Yonada`, `Zetar`) | **Tier 1 (Native)** | Native Pacman / AUR / Flatpak | AVX2 & Universal | Verified (Podman) |
| | `18.0 Illyria`–`22.1 Talos` | **Tier 1 (Rolling) / Tier 2** | Pacman Upgrade / Flatpak | Universal & AVX2 | Verified (Sandbox) |
| **8. openSUSE** | `Tumbleweed` (`2018`–`2026` Rolling) & `Leap 16.0` | **Tier 1 (Native)** | Native Zypper RPM / Flatpak | AVX2 & Universal | Verified (Podman) |
| | `Leap 15.0`–`15.6` & `MicroOS / Aeon` | **Tier 2 (Sandbox)** | Flatpak | Universal & AVX2 | Verified (Sandbox) |
| **9. elementary OS** | `8.0` / `8.1` (Circe) | **Tier 1 (Native)** | Native `.deb` / Flatpak | AVX2 & Universal | Verified (Podman) |
| | `5.0` (Juno), `5.1` (Hera), `6.0` (Odin), `6.1` (Jólnir), `7.0` / `7.1` (Horus) | **Tier 2 (Sandbox)** | Flatpak (AppCenter / Sideload) | Universal & AVX2 | Verified (Sandbox) |
| **10. Zorin OS** | `Zorin OS 18` | **Tier 1 (Native)** | Native `.deb` / Flatpak | AVX2 & Universal | Verified (Podman) |
| | `Zorin OS 15`–`15.3`, `16`–`16.3`, `17`–`17.3` | **Tier 2 (Sandbox)** | Flatpak (Flathub pre-installed) | Universal & AVX2 | Verified (Sandbox) |
| **11. Linux Lite** | `Linux Lite 7.0` / `7.2` / `7.4` (Galena) | **Tier 1 (Native)** | Native `.deb` / Flatpak | AVX2 & Universal | Verified (Podman) |
| | `Linux Lite 4.0`–`4.8`, `5.0`–`5.8`, `6.0`–`6.6` | **Tier 2 (Sandbox)** | Flatpak / Snap | Universal & AVX2 | Verified (Sandbox) |
| **12. SteamOS** | `SteamOS 3.0`–`3.7` (Holo) & `2.195` (Brewmaster) | **Tier 2 (Sandbox)** | Flatpak (`flatpak --user` on Steam Deck LCD/OLED) | AVX2 (AMD Zen 2 APU) & Universal | Verified (Podman / Sandbox) |
| **Gentoo Linux** | Stage 3 Rolling (`2018`–`2026`) | **Tier 1 (Native)** | Portage / Binhost / Cargo | AVX2 & Universal | Verified (Podman Binhost) |

For complete packaging instructions, system dependencies, and sandbox permissions, see the **[Universal Packaging Guide](docs/PACKAGING.md)**.

### Hardware Compatibility: Universal vs. AVX2

OpenDictate ships dual-variant binaries to guarantee performance on high-end hardware without sacrificing compatibility on older machines:

- **Universal Variant (`universal`):** Built with `-DGGML_AVX2=OFF -DGGML_FMA=OFF`. Compatible with any standard 64-bit x86 processor (Intel Core 2 Duo, Pentium, Celeron, Athlon, and hypervisor VMs lacking AVX2 passthrough). Eliminates `SIGILL` (Illegal Instruction) crashes.
- **AVX2 Variant (`avx2`):** Built with `-DGGML_AVX2=ON -DGGML_FMA=ON`. Delivers 2.5× to 3× faster local speech transcription on Intel Haswell (2013+) and AMD Ryzen / Zen (2017+) or newer processors.
- **ARM64 (`aarch64`):** Fully supports 64-bit ARM with NEON vector extensions across Raspberry Pi 4/5, Asahi Linux, and ARM-based laptops.

**Detecting your CPU capabilities:**
```bash
grep -E '(avx2.*fma|fma.*avx2)' /proc/cpuinfo >/dev/null && echo "Use AVX2 build" || echo "Use Universal build"
```

---

## Command Line Usage

OpenDictate can be controlled via command-line arguments and bound to system keyboard shortcuts:

```bash
# Launch main window
opendictate

# Launch floating Mini-Bar
opendictate --minibar

# Toggle recording on/off (ideal for global desktop hotkeys)
opendictate --toggle

# Display version and CLI help
opendictate --version
opendictate --help
```

---

## Development & Testing

### Running Full Test Suite
```bash
cargo test -- --test-threads=1
```

### Automated Multi-Distro Container Matrix
Validate native compilation and dependency resolution across distributions with Podman:

```bash
# Dry run specification check
bash scripts/test_distro_matrix.sh --dry-run

# Test individual distributions across all 12 families
bash scripts/test_distro_matrix.sh --distro ubuntu-26.04
bash scripts/test_distro_matrix.sh --distro debian-13
bash scripts/test_distro_matrix.sh --distro mint-22
bash scripts/test_distro_matrix.sh --distro arch
bash scripts/test_distro_matrix.sh --distro fedora
bash scripts/test_distro_matrix.sh --distro kali
bash scripts/test_distro_matrix.sh --distro manjaro
bash scripts/test_distro_matrix.sh --distro opensuse
bash scripts/test_distro_matrix.sh --distro elementary-8
bash scripts/test_distro_matrix.sh --distro zorin-18
bash scripts/test_distro_matrix.sh --distro linux-lite-7
bash scripts/test_distro_matrix.sh --distro steamos-3

# Run full batch across all standard distributions
bash scripts/test_distro_matrix.sh --distro all
```

---

## Documentation

- **[Packaging & Distribution Guide](docs/PACKAGING.md)**: System requirements, packaging manifests, step-by-step builds, and troubleshooting.
- **[AppStream Metadata](data/io.github.opendictate.OpenDictate.metainfo.xml)**: FreeDesktop specification metadata for Linux software centers.

---

## License

OpenDictate is free and open-source software licensed under the [MIT License](LICENSE).
