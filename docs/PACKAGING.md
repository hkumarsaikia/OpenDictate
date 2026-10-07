# OpenDictate Universal Packaging & Distribution Guide

OpenDictate is a native Linux voice dictation and AI speech-to-text desktop application built with Rust, GTK 4.12+, Libadwaita 1.5+, and Relm4 0.9, embedding an in-process `whisper.cpp` speech recognition engine via `whisper-rs`.

This guide provides instructions for building, packaging, verifying, and troubleshooting OpenDictate across Linux distributions.

---

## 1. System Requirements & Architecture

### Operating System & Kernel
- **Kernel:** Linux 5.4 or later (64-bit kernel required).
- **Display Servers:**
  - **Wayland:** Native Wayland support via GTK4 and Libadwaita.
  - **X11:** Fully supported via GTK4 X11 backend fallback.
- **Audio Servers:**
  - **PipeWire:** Recommended (native PulseAudio and ALSA emulation).
  - **PulseAudio:** Fully supported via `cpal` audio backend.
  - **ALSA:** Direct hardware fallback supported.

### Hardware Architectures
- **x86_64 (AMD64):**
  - **Universal Baseline:** Compatible with any standard 64-bit x86 processor (SSE2 minimum).
  - **AVX2 / FMA Acceleration:** Optimized for Intel Haswell (2013+) and AMD Zen (2017+) or newer processors.
- **aarch64 (ARM64):**
  - Standard 64-bit ARM with NEON vector instructions (Raspberry Pi 4/5, Asahi Linux, ARM-based servers and laptops).

### Memory & Storage
- **Memory (RAM):**
  - Minimum: 2 GB RAM (with Cloud AI transcription or Whisper `tiny` model).
  - Recommended: 4 GB+ RAM (for local Whisper `base` or `small` models).
  - Power User: 8 GB+ RAM (for local Whisper `medium` model).
- **Disk Space & Persistent Storage:**
  - Application binary: ~10 MB (stripped).
  - Configuration: `~/.config/opendictate/config.json` (stores user preferences including `ui_language`).
  - Dictation History (SQLite): `~/.local/share/opendictate/history.db` (local SQLite database for transcript history).
  - Local AI Whisper models (`~/.local/share/opendictate/models/`):
    - `ggml-tiny.bin`: ~75 MB
    - `ggml-base.bin`: ~142 MB
    - `ggml-small.bin`: ~466 MB
    - `ggml-medium.bin`: ~1.5 GB

---

## 2. Linux Distribution Strategy & 2018–2026 Compatibility Matrix

Linux distributions feature varying library release cycles and long-term support windows. Because OpenDictate is built on GTK 4.12+ and Libadwaita 1.5+, our packaging architecture is organized around **two distinct runtime execution tiers**:

### 2.1 Dual Runtime Execution Tiers

#### Tier 1: Modern Native Targets (2023–2026)
- **Target Systems:** Ubuntu 23.10 / 24.04 LTS / 24.10 / 25.04 / 25.10 / 26.04 LTS, Linux Mint 22.x / LMDE 7, Debian 13 (Trixie) / Sid, Debian 12 (with Trixie overlay/backports), Arch Linux (2023–2026), Fedora 39–44 Workstation, Kali Linux (2023.3–2026.3), Manjaro Linux (23.x–26.x), openSUSE Tumbleweed / Leap 16.0, elementary OS 8.x (Circe), Zorin OS 18, Linux Lite 7.x (Galena), and Gentoo Linux.
- **Packaging Format:** Native distribution packages (`.deb`, RPM, AUR PKGBUILD, Gentoo Portage ebuild) or compiling from source via Cargo.
- **Characteristics:** Native system repositories supply modern C libraries (glibc ≥ 2.38), GTK 4.12+ (up to GTK 4.22+ in Ubuntu 26.04), and Libadwaita 1.4+/1.5+ (up to Libadwaita 1.9+ in Ubuntu 26.04). *(Note: Debian 12 Bookworm standard repos provide GTK 4.10 / Libadwaita 1.3; native compilation on Debian 12 requires pinning/overlaying Debian 13 Trixie or backports for GTK 4.12+ and Libadwaita 1.5+, whereas end users on stock Debian 12 should install via Flatpak).*
- **Execution:** OpenDictate executes as a native host binary with zero container or sandboxing overhead, connecting directly to host PipeWire / PulseAudio / ALSA audio devices (`System Default`, `Headphones`, `Handsfree`), Wayland compositors, and system D-Bus.

#### Tier 2: Universal Sandboxed Targets (2018–2022 & Immutable Desktops)
- **Target Systems:** Legacy LTS and interim releases including Ubuntu 18.04 LTS–23.04; Linux Mint 19.x, 20.x, 21.x, and LMDE 3–6; Debian 10 (Buster), 11 (Bullseye), and stock Debian 12 (Bookworm); Fedora 28–38; Kali Linux 2018.1–2023.2; Manjaro 18.x–22.x; openSUSE Leap 15.0–15.6; elementary OS 5.x (Juno/Hera), 6.x (Odin/Jólnir), and 7.x (Horus); Zorin OS 15.x, 16.x, and 17.x; Linux Lite 4.x, 5.x, and 6.x; as well as immutable/atomic desktop and handheld gaming systems (**SteamOS 2.195 Brewmaster & SteamOS 3.0–3.7 Holo**, Fedora Silverblue, openSUSE MicroOS / Aeon).
- **Packaging Format:** Universal sandboxed bundles: **Flatpak (`org.gnome.Platform//48`)** and **Snap (`core24`)**.
- **Backward Compatibility Mechanism:**
  - **Runtime Encapsulation:** Older distribution repositories only ship GTK 3 (3.22/3.24) or early GTK 4.6, lacking Libadwaita. The Flatpak GNOME 48 runtime bundles GTK 4.18+, Libadwaita 1.7+, and glibc 2.40+ inside an isolated sandbox, allowing systems as old as Ubuntu 18.04 (Linux kernel ≥ 4.15) to run OpenDictate out of the box without altering host system libraries.
  - **Snap Core24 Base:** The Snap package is constructed upon Canonical's Ubuntu 24.04 LTS (`core24`) foundation with GNOME extension plugins, enabling clean compatibility across all system configurations equipped with `snapd`.
  - **Permission Portals:** Display connectivity is provided through Wayland and X11 fallback sockets (`--socket=wayland`, `--socket=fallback-x11`); microphone recording is routed through PulseAudio/PipeWire interfaces (`--socket=pulseaudio`, `--device=dri`, `plugs: [audio-record, audio-playback]`); and application data persistence (offline Whisper models and the SQLite history database `history.db`) is maintained via dedicated XDG data roots (`--filesystem=xdg-data/opendictate:create`, `--filesystem=xdg-config/opendictate:create`).

---

### 2.2 Comprehensive 2018–2026 Multi-Distribution Compatibility Matrix

| Distribution Family | Version / Release (2018–2026) | Year | Runtime Tier | Primary Package | Test Method & Container Image | Minimum Floor / Shipped Libraries |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **1. Ubuntu** | **26.04 / 26.04.1 LTS** (Resolute Raccoon) | 2026 | **Tier 1 (Native)** | Native `.deb` & Source | `docker.io/library/ubuntu:resolute` (Podman) | GTK 4.22+, Libadwaita 1.9+, glibc 2.41+ |
| | **24.10 / 25.04 / 25.10** (Oracular / Plucky / Questing) | 2024–2025 | **Tier 1 (Native)** | Native `.deb`, Snap, Source | Ubuntu 24.04+ ABI & repo verification | GTK 4.16+, Libadwaita 1.6+, glibc 2.40+ |
| | **24.04 LTS** (Noble Numbat) | 2024 | **Tier 1 (Native)** | Native `.deb`, Snap (`core24`), Source | `docker.io/library/ubuntu:24.04` (Podman) | GTK 4.14+, Libadwaita 1.5+, glibc 2.39 |
| | **23.10** (Mantic Minotaur) | 2023 | **Tier 1 (Native)** | Native `.deb`, Flatpak, Snap |GTK 4.12 / Libadwaita 1.4 verification | GTK 4.12, Libadwaita 1.4, glibc 2.38 |
| | **22.04 LTS / 22.10 / 23.04** (Jammy / Kinetic / Lunar) | 2022–2023 | **Tier 2 (Sandbox)** | Flatpak & Snap | `docker.io/library/ubuntu:22.04` (Podman / Manifest) | GTK 4.6–4.10 (host) → GNOME 48 (sandbox) |
| | **20.04 LTS / 20.10 / 21.04 / 21.10** (Focal–Impish) | 2020–2021 | **Tier 2 (Sandbox)** | Flatpak & Snap | Kernel & sandbox verification | GTK 3.24 (host) → GNOME 48 (sandbox) |
| | **18.04 LTS / 18.10 / 19.04 / 19.10** (Bionic–Eoan) | 2018–2019 | **Tier 2 (Sandbox)** | Flatpak & Snap | Kernel (≥ 4.15) & sandbox verification | GTK 3.22–3.24 (host) → GNOME 48 (sandbox) |
| **2. Linux Mint** | **Mint 22 / 22.1 / 22.2 / 22.3** (Wilma / Xia / Zara) & **LMDE 7** | 2024–2026 | **Tier 1 (Native)** | Native `.deb` & Flatpak | `docker.io/linuxmintd/mint22-amd64` (Podman) | GTK 4.14+, Libadwaita 1.5+ (Ubuntu 24.04 / Debian 13 base) |
| | **Mint 21 / 21.1 / 21.2 / 21.3** (Vanessa / Vera / Victoria / Virginia) & **LMDE 6** | 2022–2024 | **Tier 2 (Sandbox)** | Flatpak (Flathub built-in) | Manifest & Flathub verification | GTK 4.6–4.10 (host) → GNOME 48 (sandbox) |
| | **Mint 20 / 20.1 / 20.2 / 20.3** (Ulyana / Ulyssa / Uma / Una) & **LMDE 5** | 2020–2022 | **Tier 2 (Sandbox)** | Flatpak | Flathub & sandbox verification | GTK 3.24 (host) → GNOME 48 (sandbox) |
| | **Mint 19 / 19.1 / 19.2 / 19.3** (Tara / Tessa / Tina / Tricia) & **LMDE 3 / LMDE 4** | 2018–2020 | **Tier 2 (Sandbox)** | Flatpak | Flathub & sandbox verification | GTK 3.22–3.24 (host) → GNOME 48 (sandbox) |
| **3. Debian** | **Debian 13** (Trixie) & **Sid** | 2025–2026 | **Tier 1 (Native)** | Native `.deb` & Source | `docker.io/library/debian:trixie` (Podman) | GTK 4.16+, Libadwaita 1.6+, glibc 2.40 |
| | **Debian 12** (Bookworm) | 2023 | **Tier 1 / Tier 2** | Flatpak (Stock) / Native `.deb` (Trixie overlay) | `docker.io/library/debian:bookworm` (Podman) | Stock: GTK 4.10, Libadwaita 1.3 (Stock users: Flatpak; Native build requires Debian 13 Trixie overlay/backports for GTK 4.12+ and Libadwaita 1.5+) |
| | **Debian 11** (Bullseye) | 2021 | **Tier 2 (Sandbox)** | Flatpak & Snap | Archive & manifest verification | GTK 3.24 (host) → GNOME 48 (sandbox) |
| | **Debian 10** (Buster) | 2019 | **Tier 2 (Sandbox)** | Flatpak & Snap | Archive & manifest verification | GTK 3.24 (host) → GNOME 48 (sandbox) |
| **4. Arch Linux** | **Arch Linux Rolling** (`2018.01`–`2026.10`) | 2018–2026 | **Tier 1 (Native)** | Native AUR / Cargo / Flatpak | `docker.io/library/archlinux:latest` (Podman) | Rolling bleeding-edge GTK 4.16+, Libadwaita 1.6+ |
| **5. Fedora** | **Fedora 39 / 40 / 41 / 42 / 43 / 44** Workstation | 2023–2026 | **Tier 1 (Native)** | Native RPM, Flatpak, Cargo | `docker.io/library/fedora:40` (Podman) | GTK 4.12–4.18+, Libadwaita 1.4–1.7+, glibc 2.38+ |
| | **Fedora 28 / 29 / 30 / 31 / 32 / 33 / 34 / 35 / 36 / 37 / 38** & **Silverblue** | 2018–2023 | **Tier 2 (Sandbox)** | Flatpak | Flathub & OSTree verification | GTK 3.22–4.10 (host) → GNOME 48 (sandbox) |
| **6. Kali Linux** | **Kali Rolling `2023.3`–`2026.3`** (`2023.3`, `2023.4`, `2024.1`–`2024.4`, `2025.1`–`2025.4`, `2026.1`–`2026.3`) | 2023–2026 | **Tier 1 (Native)** | Native `.deb`, Cargo, Flatpak | `docker.io/kalilinux/kali-rolling:latest` (Podman) | Debian Testing base: GTK 4.14–4.18+, Libadwaita 1.5–1.7+ |
| | **Kali Rolling `2018.1`–`2023.2`** (`2018.1`–`2018.4`, `2019.1`–`2019.4`, `2020.1`–`2020.4`, `2021.1`–`2021.4`, `2022.1`–`2022.4`, `2023.1`–`2023.2`) | 2018–2023 | **Tier 2 (Sandbox)** | Flatpak | Sandbox & Debian archive verification | GTK 3.22–4.10 (host) → GNOME 48 (sandbox) |
| **7. Manjaro** | **Manjaro `23.0`–`26.0`** (`23.0 Uranos`, `23.1 Vulcan`, `24.0 Wynsdey`, `24.1 Xahea`, `24.2 Yonada`, `25.0 Zetar`, `26.0`) | 2023–2026 | **Tier 1 (Native)** | Native Pacman / AUR / Flatpak | `docker.io/manjarolinux/base:latest` (Podman) | Arch rolling base: GTK 4.14+, Libadwaita 1.5+ |
| | **Manjaro `18.0`–`22.1`** (`18.0 Illyria`, `18.1 Juhraya`, `19.0 Kyria`, `20.0 Lysia`, `20.1 Mikah`, `20.2 Nibia`, `21.0 Ornara`, `21.1 Pahvo`, `21.2 Qonos`, `21.3 Ruah`, `22.0 Sikaris`, `22.1 Talos`) | 2018–2023 | **Tier 1 (Rolling Upgrade) / Tier 2** | Pacman Upgrade or Flatpak | Arch/Manjaro rolling & Flatpak verification | Rolling upgrade to Tier 1 or GNOME 48 (sandbox) |
| **8. openSUSE** | **Tumbleweed** (`2018`–`2026` Rolling) & **Leap 16.0** | 2018–2026 | **Tier 1 (Native)** | Native Zypper RPM & Flatpak | `docker.io/opensuse/tumbleweed:latest` (Podman) | Rolling bleeding-edge GTK 4.16+, Libadwaita 1.6+ |
| | **Leap 15.0 / 15.1 / 15.2 / 15.3 / 15.4 / 15.5 / 15.6** & **MicroOS / Aeon** | 2018–2025 | **Tier 2 (Sandbox)** | Flatpak | Flathub & transactional-update verification | SLE 15 glibc 2.26–2.38 → GNOME 48 (sandbox) |
| **9. elementary OS**| **elementary OS 8.0 / 8.1** (Circe) | 2024–2026 | **Tier 1 (Native)** | Native `.deb` & Flatpak | Ubuntu 24.04 LTS container verification | GTK 4.14+, Libadwaita 1.5+, PipeWire |
| | **elementary OS 5.0** (Juno), **5.1** (Hera), **6.0** (Odin), **6.1** (Jólnir), **7.0 / 7.1** (Horus) | 2018–2024 | **Tier 2 (Sandbox)** | Flatpak (AppCenter / Sideload) | Ubuntu 18.04 / 20.04 / 22.04 base verification | GTK 3.22–4.6 (host) → GNOME 48 (sandbox) |
| **10. Zorin OS** | **Zorin OS 18** | 2025–2026 | **Tier 1 (Native)** | Native `.deb` & Flatpak | Ubuntu 24.04 LTS container verification | GTK 4.14+, Libadwaita 1.5+ |
| | **Zorin OS 17 / 17.1 / 17.2 / 17.3** | 2023–2025 | **Tier 2 (Sandbox)** | Flatpak (Flathub pre-installed) | Ubuntu 22.04 base verification | GTK 4.6 (host) → GNOME 48 (sandbox) |
| | **Zorin OS 16 / 16.1 / 16.2 / 16.3** | 2021–2023 | **Tier 2 (Sandbox)** | Flatpak & Snap | Ubuntu 20.04 base verification | GTK 3.24 (host) → GNOME 48 (sandbox) |
| | **Zorin OS 15 / 15.1 / 15.2 / 15.3** | 2019–2020 | **Tier 2 (Sandbox)** | Flatpak & Snap | Ubuntu 18.04 base verification | GTK 3.22 (host) → GNOME 48 (sandbox) |
| **11. Linux Lite** | **Linux Lite 7.0 / 7.2 / 7.4** (Galena) | 2024–2026 | **Tier 1 (Native)** | Native `.deb` & Flatpak | Ubuntu 24.04 LTS container verification | GTK 4.14+, Libadwaita 1.5+ |
| | **Linux Lite 4.0 / 4.2 / 4.4 / 4.6 / 4.8** (Diamond), **5.0 / 5.2 / 5.4 / 5.6 / 5.8** (Emerald), **6.0 / 6.2 / 6.4 / 6.6** (Fluorite) | 2018–2024 | **Tier 2 (Sandbox)** | Flatpak & Snap | Ubuntu 18.04 / 20.04 / 22.04 base verification | GTK 3.22–4.6 (host) → GNOME 48 (sandbox) |
| **12. SteamOS** | **SteamOS 3.0 / 3.1 / 3.2 / 3.3 / 3.4 / 3.5 / 3.6 / 3.7** (Holo) & **2.195** (Brewmaster) | 2018–2026 | **Tier 2 (Sandbox)** | Flatpak (`flatpak --user` on Steam Deck LCD Aerith & OLED Sephiroth Zen 2 APU) | Arch Linux / Flatpak sandbox verification | Read-only `/usr` rootfs, PipeWire, GNOME 48 (sandbox) |
| **Gentoo Linux** | **Gentoo Rolling** | 2018–2026 | **Tier 1 (Native)** | Native Portage / Binhost | `docker.io/gentoo/stage3:latest` (Podman) | Rolling bleeding-edge GTK 4.16+, Libadwaita 1.6+ |

---

## 3. Hardware Optimization Guide: Universal vs. AVX2/FMA

Local speech transcription performance with `whisper.cpp` relies heavily on vector SIMD arithmetic. OpenDictate provides two distinct build variants for x86_64:

### 3.1 The Variants Explained

1. **Universal Variant (`universal`):**
   - **Flags:** `-DGGML_AVX2=OFF -DGGML_FMA=OFF`
   - **Target Audience:** Legacy CPUs (pre-2013), Intel Core 2 Duo, Pentium, Celeron, early Core i3/i5/i7, or low-cost virtual machine hypervisors lacking AVX2 passthrough.
   - **Advantage:** Prevents fatal crashes with `SIGILL` (`Illegal instruction`). Guaranteed to boot and run on every 64-bit x86 processor.

2. **AVX2 Variant (`avx2`):**
   - **Flags:** `-DGGML_AVX2=ON -DGGML_FMA=ON`
   - **Target Audience:** Modern processors (Intel Core 4th Gen Haswell 2013+, AMD Ryzen / Zen 2017+).
   - **Advantage:** Delivers 2× to 3× faster speech transcription throughput and significantly lower CPU utilization during local AI inference.

### 3.2 Identifying Your CPU Capabilities

Run the following command in a terminal to check whether your system supports AVX2 and FMA:

```bash
grep -E '(avx2.*fma|fma.*avx2)' /proc/cpuinfo >/dev/null && echo "Supported: AVX2+FMA" || echo "Not Supported: Use Universal variant"
```

Alternatively, inspect CPU flags using `lscpu`:

```bash
lscpu | grep -i Flags | grep -E 'avx2'
```

- If `avx2` and `fma` are present: Install the **AVX2 variant** (`opendictate_2.0.0_avx2_amd64.deb`).
- If either flag is absent: Install the **Universal variant** (`opendictate_2.0.0_universal_amd64.deb`).

---

## 4. Step-by-Step Build & Installation Guides

### 4.1 Flatpak Packaging (Flathub Standard)

The Flatpak manifest is defined at `packaging/flatpak/io.github.opendictate.OpenDictate.yml`.

#### Prerequisites
Install `flatpak` and `flatpak-builder`:

```bash
# Ubuntu / Debian
sudo apt-get install -y flatpak flatpak-builder

# Fedora
sudo dnf install -y flatpak flatpak-builder

# Arch Linux
sudo pacman -S --needed flatpak flatpak-builder
```

Install the GNOME 48 runtime and Freedesktop SDK extensions:

```bash
flatpak remote-add --if-not-exists flathub https://dl.flathub.org/repo/flathub.repo
flatpak install -y flathub \
    org.gnome.Platform//48 \
    org.gnome.Sdk//48 \
    org.freedesktop.Sdk.Extension.rust-stable//24.08 \
    org.freedesktop.Sdk.Extension.llvm18//24.08
```

#### Building & Installing Locally
```bash
# Build and install for current user
flatpak-builder --user --install --force-clean build-flatpak packaging/flatpak/io.github.opendictate.OpenDictate.yml

# Launch OpenDictate Flatpak
flatpak run io.github.opendictate.OpenDictate
```

#### Creating a Distributable `.flatpak` Bundle
```bash
# Export to a local OSTree repository
flatpak-builder --repo=repo --force-clean build-flatpak packaging/flatpak/io.github.opendictate.OpenDictate.yml

# Create single-file bundle
flatpak build-bundle repo opendictate.flatpak io.github.opendictate.OpenDictate
```

---

### 4.2 Canonical Snap (`snapcraft`)

The Snapcraft recipe is defined at `snap/snapcraft.yaml` with strict confinement and `core24` base.

#### Prerequisites
Install Snapcraft and configure LXD:

```bash
sudo snap install snapcraft --classic
sudo snap install lxd
sudo lxd init --auto
```

#### Building the Snap
```bash
# Execute build from project root
snapcraft --use-lxd
```

#### Installing & Connecting Plugs
```bash
# Install local snap package
sudo snap install --dangerous opendictate_2.0.0_amd64.snap

# Verify and connect audio recording interface
sudo snap connect opendictate:audio-record
sudo snap connect opendictate:audio-playback
```

---

### 4.3 Native Debian Packages (`build_rust_deb.sh`)

OpenDictate provides an automated dual-variant packaging tool: `build_rust_deb.sh`.

#### Prerequisites (Debian / Ubuntu)
```bash
sudo apt-get update
sudo apt-get install -y \
    build-essential \
    cmake \
    clang \
    pkg-config \
    libssl-dev \
    libasound2-dev \
    libgtk-4-dev \
    libadwaita-1-dev \
    libdbus-1-dev
```

> [!NOTE]
> **Debian 12 (Bookworm) Native Build Note:**
> Debian 12 standard repositories provide GTK 4.10 and Libadwaita 1.3, which do not meet OpenDictate's requirements for GTK 4.12+ and Libadwaita 1.5+. End users on stock Debian 12 should install via **Flatpak** (see Section 4.1). For native compilation on Debian 12, pin or overlay Debian 13 (Trixie) repositories or backports for `libgtk-4-dev` and `libadwaita-1-dev`:
> ```bash
> # Add Debian 13 (Trixie) overlay for GTK 4.12+ and Libadwaita 1.5+
> echo 'deb http://deb.debian.org/debian trixie main' | sudo tee /etc/apt/sources.list.d/trixie.list
> sudo apt-get update
> sudo apt-get install -y libgtk-4-dev libadwaita-1-dev
> ```

#### Building Packages
```bash
# 1. Build Universal package (baseline x86_64, no AVX2/FMA required)
bash build_rust_deb.sh --variant universal

# 2. Build AVX2 package (Haswell+/Zen+ accelerated)
bash build_rust_deb.sh --variant avx2

# 3. Build both packages in batch mode
bash build_rust_deb.sh --variant all
```

Output files in `dist/`:
- `dist/opendictate_2.0.0_universal_amd64.deb`
- `dist/opendictate_2.0.0_avx2_amd64.deb`
- `dist/opendictate_2.0.0_amd64.deb` (backwards-compatible symlink to universal)

#### Inspecting Package Integrity
```bash
# Verify file contents and permissions
dpkg -c dist/opendictate_2.0.0_universal_amd64.deb

# Verify Debian package control fields
dpkg -I dist/opendictate_2.0.0_universal_amd64.deb
```

#### Installing on Target Machine
```bash
# Install chosen variant
sudo dpkg -i dist/opendictate_2.0.0_universal_amd64.deb

# Resolve any missing host dependencies
sudo apt-get install -f -y
```

---

### 4.4 Standalone AppImage (`build_appimage.sh`)

The AppImage packaging script is located at `packaging/appimage/build_appimage.sh`.

#### Building the AppImage
```bash
# Build AppDir directory hierarchy and SquashFS bundle
bash packaging/appimage/build_appimage.sh

# Or stage AppDir without bundling
bash packaging/appimage/build_appimage.sh --appdir-only
```

#### Testing the Staged AppDir Directly
```bash
./build/AppDir/AppRun --version
./build/AppDir/AppRun --help
```

> [!NOTE]
> The default AppDir packaging bundles OpenDictate's standalone binary, desktop integration, icons, and schemas, relying on host system GTK4 and Libadwaita libraries. For legacy systems without host GTK4, use the Flatpak bundle or bundle system libraries using `linuxdeploy-plugin-gtk`.

---

### 4.5 Native Compilation from Source (Cargo)

For distributions without prebuilt binaries (e.g. Arch Linux, Fedora, openSUSE):

#### 1. Install System Development Libraries
- **Fedora 39/40+:**
  ```bash
  sudo dnf install -y gcc gcc-c++ cmake clang pkgconfig openssl-devel alsa-lib-devel gtk4-devel libadwaita-devel dbus-devel
  ```
- **Arch Linux:**
  ```bash
  sudo pacman -S --needed base-devel cmake clang pkgconf openssl alsa-lib gtk4 libadwaita dbus
  ```
- **openSUSE Tumbleweed:**
  ```bash
  sudo zypper install -y gcc gcc-c++ cmake clang pkg-config libopenssl-devel alsa-devel gtk4-devel libadwaita-devel dbus-1-devel
  ```

#### 2. Compile via Cargo

Whisper's embedded C-FFI runtime is configured via `GGML_` environment variables:

```bash
# Universal compilation (generic baseline x86_64, zero AVX/AVX2/FMA instructions)
export GGML_NATIVE=OFF
export GGML_AVX=OFF
export GGML_AVX2=OFF
export GGML_AVX512=OFF
export GGML_FMA=OFF
cargo build --release

# Accelerated compilation (modern Haswell+/Zen+ CPUs with AVX2 and FMA)
export GGML_NATIVE=OFF
export GGML_AVX=ON
export GGML_AVX2=ON
export GGML_AVX512=OFF
export GGML_FMA=ON
cargo build --release
```

Binary is output to `target/release/opendictate`.

---

## 5. Automated Container Matrix Testing

OpenDictate includes an automated container validation test runner: `scripts/test_distro_matrix.sh`. It verifies native compilation (`cargo check`) and `pkg-config` dependency resolution (`gtk4`, `libadwaita-1`, `alsa`, `dbus-1`) across all 12 core Linux distributions (Ubuntu, Linux Mint, Debian, Arch Linux, Fedora, Kali Linux, Manjaro, openSUSE, elementary OS, Zorin OS, Linux Lite, and SteamOS) as well as Gentoo Linux using **Podman** (rootless) or Docker.

### 5.1 Supported Distribution Targets

| Distro Flag | Target Release | Container Base Image | Package Manager | Verified Packages |
| :--- | :--- | :--- | :--- | :--- |
| `ubuntu-26.04` | Ubuntu 26.04.1 LTS (Resolute) | `docker.io/library/ubuntu:resolute` | `apt-get` | GTK 4.22+, Libadwaita 1.9+, ALSA, DBus |
| `ubuntu-24.04` | Ubuntu 24.04 LTS (Noble) | `docker.io/library/ubuntu:24.04` | `apt-get` | GTK 4.14+, Libadwaita 1.5+, ALSA, DBus |
| `debian-13` | Debian 13 (Trixie) | `docker.io/library/debian:trixie` | `apt-get` | GTK 4.16+, Libadwaita 1.6+, ALSA, DBus |
| `debian-12` | Debian 12 (Bookworm) | `docker.io/library/debian:bookworm` | `apt-get` | GTK 4.12+, Libadwaita 1.5+, ALSA, DBus (via Trixie overlay) |
| `mint-22` | Linux Mint 22 (Wilma) | `docker.io/linuxmintd/mint22-amd64` | `apt-get` | GTK 4.14+, Libadwaita 1.5+, ALSA, DBus |
| `arch` | Arch Linux (Rolling) | `docker.io/library/archlinux:latest` | `pacman` | Rolling GTK 4.16+, Libadwaita 1.6+, ALSA, DBus |
| `fedora` | Fedora 40 Workstation | `docker.io/library/fedora:40` | `dnf` | GTK 4.14+, Libadwaita 1.5+, ALSA, DBus |
| `kali` | Kali Linux Rolling (`2026.x`) | `docker.io/kalilinux/kali-rolling:latest` | `apt-get` | GTK 4.16+, Libadwaita 1.6+, ALSA, DBus |
| `manjaro` | Manjaro Linux (`24.x–26.x`) | `docker.io/manjarolinux/base:latest` | `pacman` | Rolling GTK 4.16+, Libadwaita 1.6+, ALSA, DBus |
| `opensuse` | openSUSE Tumbleweed (Rolling) | `docker.io/opensuse/tumbleweed:latest` | `zypper` | Rolling GTK 4.16+, Libadwaita 1.6+, ALSA, DBus |
| `elementary-8` | elementary OS 8 (Circe) | `docker.io/library/ubuntu:24.04` | `apt-get` | GTK 4.14+, Libadwaita 1.5+, ALSA, DBus |
| `zorin-18` | Zorin OS 18 | `docker.io/library/ubuntu:24.04` | `apt-get` | GTK 4.14+, Libadwaita 1.5+, ALSA, DBus |
| `linux-lite-7` | Linux Lite 7.x (Galena) | `docker.io/library/ubuntu:24.04` | `apt-get` | GTK 4.14+, Libadwaita 1.5+, ALSA, DBus |
| `steamos-3` | SteamOS 3.x (Holo) | `docker.io/library/archlinux:latest` | `pacman` | Arch base + Flatpak GNOME 46 runtime |
| `gentoo` | Gentoo Linux (Stage 3) | `docker.io/gentoo/stage3:latest` | `emerge` | Rolling GTK 4.16+, Libadwaita 1.6+ (Binhost) |
| `all` | All 14 standard targets | Batch sequential execution | Various | Comprehensive 12-distro family pass |

### 5.2 Gentoo Binhost Acceleration
Gentoo Linux compiles packages from source by default, which normally takes hours for large toolkits like GTK4 and Libadwaita. The automated test runner configures Gentoo's official binary host repository:
```
# /etc/portage/binrepos.conf/gentoo.conf
[binhost]
priority = 9999
sync-uri = https://distfiles.gentoo.org/releases/amd64/binpackages/23.0/x86-64/
```
By utilizing `emerge --getbinpkg`, binary packages are resolved and fetched directly, allowing CI verification of Gentoo to complete in approximately 2–3 minutes.
*(Note: Gentoo is excluded from the default `--distro all` batch due to initial Portage tree synchronization overhead, and can be executed on-demand via `--distro gentoo`.)*

### 5.3 Running Matrix Tests

```bash
# Dry-run: Validate matrix specifications and preview container commands
bash scripts/test_distro_matrix.sh --dry-run

# Test Ubuntu, Linux Mint, elementary OS, Zorin OS, and Linux Lite releases
bash scripts/test_distro_matrix.sh --distro ubuntu-26.04
bash scripts/test_distro_matrix.sh --distro ubuntu-24.04
bash scripts/test_distro_matrix.sh --distro mint-22
bash scripts/test_distro_matrix.sh --distro elementary-8
bash scripts/test_distro_matrix.sh --distro zorin-18
bash scripts/test_distro_matrix.sh --distro linux-lite-7

# Test Debian and Kali Linux releases
bash scripts/test_distro_matrix.sh --distro debian-13
bash scripts/test_distro_matrix.sh --distro debian-12
bash scripts/test_distro_matrix.sh --distro kali

# Test Arch, Manjaro, SteamOS, Fedora, openSUSE, and Gentoo
bash scripts/test_distro_matrix.sh --distro arch
bash scripts/test_distro_matrix.sh --distro manjaro
bash scripts/test_distro_matrix.sh --distro steamos-3
bash scripts/test_distro_matrix.sh --distro fedora
bash scripts/test_distro_matrix.sh --distro opensuse
bash scripts/test_distro_matrix.sh --distro gentoo

# Test all standard distributions sequentially
bash scripts/test_distro_matrix.sh --distro all
```

Container logs are stored in `build_reports/<distro>.log`.

> [!NOTE]
> OpenDictate's DBus status notifier crate (`ksni`) links against `libdbus-sys`. Container testing validates that `dbus-1.pc` (`libdbus-1-dev`, `dbus-devel`, or `sys-apps/dbus`) is installed alongside GTK4 and Libadwaita across all distribution families.

---

## 6. Troubleshooting & Permissions

### 6.1 Microphone Permissions & Audio Device Discovery

#### Flatpak Sandboxing
Flatpak isolates audio access through the PulseAudio socket. If no microphone devices appear in OpenDictate's Audio Settings:
1. Ensure the audio socket permission is granted:
   ```bash
   flatpak override --user --socket=pulseaudio io.github.opendictate.OpenDictate
   ```
2. Check your PipeWire / PulseAudio recording source:
   ```bash
   wpctl status          # For PipeWire
   pactl info            # For PulseAudio
   ```

#### Snap Confinement
Snaps run under strict confinement and do not have microphone access by default until the user or installer connects the plug:
```bash
snap connections opendictate
sudo snap connect opendictate:audio-record
```

#### Raw ALSA Fallback
If running without PipeWire or PulseAudio (e.g., minimal server or custom desktop), ensure your user belongs to the `audio` group:
```bash
sudo usermod -aG audio $USER
```

---

### 6.2 Audio Sampling Rates & Dynamic Resampling

OpenDictate is engineered with an embedded DSP resampler powered by [`rubato`](https://crates.io/crates/rubato). While Whisper models require exactly 16,000 Hz 16-bit mono linear PCM audio:
1. **Dynamic Hardware Rate Negotiation:** The audio capture engine interrogates the hardware's supported sampling rates (e.g. 48,000 Hz, 44,100 Hz, 96,000 Hz, or 8,000 Hz) and streams at the device's native rate to eliminate hardware-level buffer underruns.
2. **Band-Limited Sinc Resampling:** The raw input stream is downmixed to mono and resampled to 16 kHz using band-limited sinc interpolation with anti-aliasing filtering.
3. **Troubleshooting Rate Mismatches / Distorted Audio:**
   - If an exotic USB interface or virtual audio cable fails to negotiate, check ALSA device capabilities:
     ```bash
     arecord -l
     arecord --dump-hw-params -D hw:0,0 /dev/null
     ```
   - If audio levels are clamped or muted, inspect the real-time VU meter in the Settings Audio tab or Mini-Bar.
   - For virtual PipeWire streams, verify default node rates:
     ```bash
     pw-top
     ```

---

### 6.3 Global Hotkey Handling (Wayland vs. X11)

#### Under X11
Global hotkeys (such as `Ctrl+Shift+Space`) are captured globally using the X11 keyboard grab APIs.

#### Under Wayland
Wayland's security architecture restricts unfocused windows from capturing global keystrokes. OpenDictate provides two solutions:
1. **Desktop Portal Integration:** OpenDictate requests global shortcuts via the XDG GlobalShortcuts portal (`org.freedesktop.portal.Desktop`).
2. **System Custom Shortcut (Recommended for all Wayland Compositors):**
   You can bind any key combination (e.g. `Ctrl+Alt+D` or `Super+D`) in your desktop settings to invoke OpenDictate's toggle command:
   ```bash
   opendictate --toggle
   ```
   Or to bring up the floating Mini-Bar:
   ```bash
   opendictate --minibar
   ```

---

### 6.4 Missing Icons or Schemas in Desktop Menu

If installing manually from source or extracting an archive and icons or desktop menu entries do not update:
```bash
# Update desktop menu database
update-desktop-database ~/.local/share/applications /usr/share/applications

# Update GTK icon cache
gtk-update-icon-cache -f -t ~/.local/share/icons/hicolor /usr/share/icons/hicolor
```

---

## 7. Internationalization (i18n) & Localized Desktops

OpenDictate includes an embedded compile-time internationalization catalog designed for desktop Linux environments.

### 7.1 28-Language Worldwide Coverage
The translation catalog covers 28 major languages spoken across desktop Linux userbases in Europe, the Americas, Asia, and the Middle East:

| Language Code | Language Name | Native Endonym | Text Direction |
| :--- | :--- | :--- | :--- |
| `en` | English | English | Left-to-Right |
| `es` | Spanish | Español | Left-to-Right |
| `de` | German | Deutsch | Left-to-Right |
| `fr` | French | Français | Left-to-Right |
| `zh-CN` | Simplified Chinese | 简体中文 | Left-to-Right |
| `ru` | Russian | Русский | Left-to-Right |
| `pt-BR` | Portuguese (Brazil) | Português (Brasil) | Left-to-Right |
| `ja` | Japanese | 日本語 | Left-to-Right |
| `it` | Italian | Italiano | Left-to-Right |
| `pl` | Polish | Polski | Left-to-Right |
| `nl` | Dutch | Nederlands | Left-to-Right |
| `tr` | Turkish | Türkçe | Left-to-Right |
| `ko` | Korean | 한국어 | Left-to-Right |
| `uk` | Ukrainian | Українська | Left-to-Right |
| `hi` | Hindi | हिन्दी | Left-to-Right |
| `ar` | Arabic | العربية | Right-to-Left (RTL) |
| `id` | Indonesian | Bahasa Indonesia | Left-to-Right |
| `vi` | Vietnamese | Tiếng Việt | Left-to-Right |
| `zh-TW` | Traditional Chinese | 繁體中文 | Left-to-Right |
| `cs` | Czech | Čeština | Left-to-Right |
| `sv` | Swedish | Svenska | Left-to-Right |
| `hu` | Hungarian | Magyar | Left-to-Right |
| `ro` | Romanian | Română | Left-to-Right |
| `el` | Greek | Ελληνικά | Left-to-Right |
| `da` | Danish | Dansk | Left-to-Right |
| `fi` | Finnish | Suomi | Left-to-Right |
| `nb` | Norwegian Bokmål | Norsk bokmål | Left-to-Right |
| `bn` | Bengali | বাংলা | Left-to-Right |

### 7.2 Compile-Time Catalog & Dynamic Retranslation
- **Compile-Time Catalog:** The translation service (`src/services/i18n.rs`) embeds the translation dictionary directly into the binary at compile time without requiring external `.mo` runtime files.
- **Dynamic In-Place Retranslation:** Selecting a language in the header menu popover immediately updates UI strings in-place across the Main Window, Settings view, header popover, and floating MiniBar without requiring an application restart.
- **Arabic Right-to-Left (RTL) Support:** OpenDictate applies GTK4 RTL widget text direction when Arabic (`ar`) is selected.
- **Configuration Persistence:** The chosen language code is stored under the `ui_language` key in `~/.config/opendictate/config.json` and automatically synchronized whenever changed.

### 7.3 In-Popover Dictation History & SQLite Storage
- **Database Location:** `~/.local/share/opendictate/history.db` (created automatically on first launch).
- **Embedded Database Engine:** OpenDictate embeds `rusqlite` (bundled SQLite 3), eliminating external SQL database daemon dependencies.
- **In-Popover History Viewer:** Accessed via the Main Menu popover with smooth sliding navigation (`gtk4::Stack`). Features:
  - Truncated summary previews with timestamp.
  - Hover tooltips displaying the complete, unabridged transcript.
  - One-click copy button to place transcript text onto the system clipboard (`arboard`/GDK clipboard).
  - Individual record deletion button with immediate SQLite cleanup and empty-state synchronization.

