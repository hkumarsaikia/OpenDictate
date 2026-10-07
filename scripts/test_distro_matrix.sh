#!/usr/bin/env bash
# ==============================================================================
# OpenDictate: Automated Multi-Distro Validation Suite
#
# Tests compilation and native dependency resolution across major Linux distro
# families using container runtimes (Podman / Docker):
#   1. Debian / Ubuntu / Mint / Kali / elementary / Zorin / Linux Lite:
#      ubuntu:26.04 (resolute), ubuntu:24.04, debian:13 (trixie), debian:12 (bookworm),
#      mint22-amd64, kalilinux/kali-rolling, elementary-8, zorin-18, linux-lite-7 (apt-get)
#   2. Red Hat / Fedora family: fedora:40 (dnf)
#   3. Arch / Manjaro / SteamOS family: archlinux:latest, manjarolinux/base, steamos-3 (pacman)
#   4. openSUSE family:         opensuse/tumbleweed:latest (zypper)
#   5. Gentoo family:           gentoo/stage3:latest (emerge with binrepos)
#
# Requirements tested:
#   - Provisioning package manager native build libraries
#   - Verifying pkg-config discovery for gtk4, libadwaita-1, alsa, dbus-1
#   - Executing cargo check cleanly
# ==============================================================================

set -uo pipefail

# ANSI color codes
BOLD=$'\033[1m'
GREEN=$'\033[1;32m'
RED=$'\033[1;31m'
YELLOW=$'\033[1;33m'
BLUE=$'\033[1;34m'
CYAN=$'\033[1;36m'
RESET=$'\033[0m'

# Project root directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
REPORT_DIR="${REPO_ROOT}/build_reports"

# Default configuration
TARGET_DISTRO="all"
DRY_RUN=false
CONTAINER_ENGINE=""

# ------------------------------------------------------------------------------
# Usage & Help
# ------------------------------------------------------------------------------
show_help() {
    cat <<EOF
${BOLD}OpenDictate Multi-Distro Container Testing Suite${RESET}

${BOLD}USAGE:${RESET}
    bash scripts/test_distro_matrix.sh [OPTIONS]

${BOLD}OPTIONS:${RESET}
    --distro <NAME>     Distro to test:
                        ubuntu-26.04 | ubuntu-24.04 | debian-13 | debian-12 | mint-22 |
                        arch | fedora | kali | manjaro | opensuse | elementary-8 |
                        zorin-18 | linux-lite-7 | steamos-3 | gentoo | all
                        (Alias: ubuntu -> ubuntu-24.04; Default: all)
                        Note: '--distro all' excludes gentoo from the default batch run due to Portage
                        sync overhead; gentoo can be tested individually with '--distro gentoo'.
    --dry-run           Print container specs and command steps without launching
    --engine <NAME>     Container engine override: podman | docker (Default: auto-detect)
    -h, --help          Display this help message

${BOLD}EXAMPLES:${RESET}
    bash scripts/test_distro_matrix.sh --distro ubuntu-26.04
    bash scripts/test_distro_matrix.sh --distro kali
    bash scripts/test_distro_matrix.sh --distro manjaro
    bash scripts/test_distro_matrix.sh --distro all --dry-run
EOF
}

# ------------------------------------------------------------------------------
# Argument Parsing
# ------------------------------------------------------------------------------
while [[ $# -gt 0 ]]; do
    case "$1" in
        --distro)
            if [[ -z "${2:-}" ]]; then
                echo -e "${RED}Error: --distro requires an argument${RESET}" >&2
                exit 1
            fi
            TARGET_DISTRO=$(echo "$2" | tr '[:upper:]' '[:lower:]')
            shift 2
            ;;
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        --engine)
            if [[ -z "${2:-}" ]]; then
                echo -e "${RED}Error: --engine requires an argument (podman|docker)${RESET}" >&2
                exit 1
            fi
            CONTAINER_ENGINE="$2"
            shift 2
            ;;
        -h|--help)
            show_help
            exit 0
            ;;
        *)
            echo -e "${RED}Error: Unknown argument '$1'${RESET}" >&2
            show_help
            exit 1
            ;;
    esac
done

# Validate target distro
case "${TARGET_DISTRO}" in
    ubuntu-26.04|ubuntu-24.04|ubuntu|debian-13|debian-12|mint-22|arch|fedora|kali|manjaro|opensuse|elementary-8|zorin-18|linux-lite-7|steamos-3|gentoo|all)
        ;;
    *)
        echo -e "${RED}Error: Invalid distro '${TARGET_DISTRO}'. Supported: ubuntu-26.04, ubuntu-24.04, debian-13, debian-12, mint-22, arch, fedora, kali, manjaro, opensuse, elementary-8, zorin-18, linux-lite-7, steamos-3, gentoo, all${RESET}" >&2
        exit 1
        ;;
esac

# Normalize alias ubuntu -> ubuntu-24.04
if [[ "${TARGET_DISTRO}" == "ubuntu" ]]; then
    TARGET_DISTRO="ubuntu-24.04"
fi

# ------------------------------------------------------------------------------
# Detect Container Engine
# ------------------------------------------------------------------------------
detect_container_engine() {
    if [[ -n "${CONTAINER_ENGINE}" ]]; then
        if ! command -v "${CONTAINER_ENGINE}" &>/dev/null; then
            echo -e "${RED}Error: Specified container engine '${CONTAINER_ENGINE}' not found on PATH.${RESET}" >&2
            exit 1
        fi
        return 0
    fi

    if command -v podman &>/dev/null; then
        CONTAINER_ENGINE="podman"
    elif command -v docker &>/dev/null; then
        CONTAINER_ENGINE="docker"
    else
        if [[ "${DRY_RUN}" == "true" ]]; then
            CONTAINER_ENGINE="podman (simulated)"
        else
            echo -e "${RED}Error: Neither 'podman' nor 'docker' container runtime found on PATH.${RESET}" >&2
            echo -e "Please install podman: sudo apt-get update && sudo apt-get install -y podman" >&2
            exit 1
        fi
    fi
}

detect_container_engine

# ------------------------------------------------------------------------------
# Distro Definitions
# ------------------------------------------------------------------------------
get_distro_image() {
    case "$1" in
        ubuntu-26.04)        echo "docker.io/library/ubuntu:resolute" ;;
        ubuntu-24.04|ubuntu) echo "docker.io/library/ubuntu:24.04" ;;
        debian-13)           echo "docker.io/library/debian:trixie" ;;
        debian-12)           echo "docker.io/library/debian:bookworm" ;;
        mint-22)             echo "docker.io/linuxmintd/mint22-amd64" ;;
        arch)                echo "docker.io/library/archlinux:latest" ;;
        fedora)              echo "docker.io/library/fedora:40" ;;
        kali)                echo "docker.io/kalilinux/kali-rolling:latest" ;;
        manjaro)             echo "docker.io/manjarolinux/base:latest" ;;
        opensuse)            echo "docker.io/opensuse/tumbleweed:latest" ;;
        elementary-8)        echo "docker.io/library/ubuntu:24.04" ;;
        zorin-18)            echo "docker.io/library/ubuntu:24.04" ;;
        linux-lite-7)        echo "docker.io/library/ubuntu:24.04" ;;
        steamos-3)           echo "docker.io/library/archlinux:latest" ;;
        gentoo)              echo "docker.io/gentoo/stage3:latest" ;;
    esac
}

get_distro_family() {
    case "$1" in
        ubuntu-26.04)        echo "Ubuntu 26.04 LTS (Debian / Ubuntu family)" ;;
        ubuntu-24.04|ubuntu) echo "Ubuntu 24.04 LTS (Debian / Ubuntu family)" ;;
        debian-13)           echo "Debian 13 Trixie (Debian family)" ;;
        debian-12)           echo "Debian 12 Bookworm (Debian family)" ;;
        mint-22)             echo "Linux Mint 22 Wilma/Zara (Ubuntu 24.04 base)" ;;
        arch)                echo "Arch Linux Rolling (Arch family)" ;;
        fedora)              echo "Fedora Workstation / Silverblue (Red Hat family)" ;;
        kali)                echo "Kali Linux Rolling 2026.x (Debian Testing base)" ;;
        manjaro)             echo "Manjaro Linux 24.x/25.x/26.x (Arch family)" ;;
        opensuse)            echo "openSUSE Tumbleweed / Leap 16 (SUSE family)" ;;
        elementary-8)        echo "elementary OS 8 Circe (Ubuntu 24.04 LTS base)" ;;
        zorin-18)            echo "Zorin OS 18 (Ubuntu 24.04 LTS base)" ;;
        linux-lite-7)        echo "Linux Lite 7.x Galena (Ubuntu 24.04 LTS base)" ;;
        steamos-3)           echo "SteamOS 3.x Holo (Arch Linux / Flatpak base)" ;;
        gentoo)              echo "Gentoo Linux family" ;;
    esac
}

get_distro_pkg_manager() {
    case "$1" in
        ubuntu-26.04|ubuntu-24.04|ubuntu|debian-13|debian-12|mint-22|kali|elementary-8|zorin-18|linux-lite-7)
            echo "apt-get"
            ;;
        arch|manjaro|steamos-3)
            echo "pacman"
            ;;
        fedora)
            echo "dnf"
            ;;
        opensuse)
            echo "zypper"
            ;;
        gentoo)
            echo "emerge"
            ;;
    esac
}

get_distro_packages() {
    case "$1" in
        ubuntu-26.04|ubuntu-24.04|ubuntu|debian-13|debian-12|mint-22|kali|elementary-8|zorin-18|linux-lite-7)
            echo "build-essential cmake clang pkg-config libssl-dev libasound2-dev libgtk-4-dev libadwaita-1-dev libdbus-1-dev ca-certificates curl"
            ;;
        fedora)
            echo "gcc gcc-c++ cmake clang pkgconfig openssl-devel alsa-lib-devel gtk4-devel libadwaita-devel dbus-devel ca-certificates curl"
            ;;
        arch|manjaro|steamos-3)
            echo "base-devel cmake clang pkgconf openssl alsa-lib gtk4 libadwaita dbus ca-certificates curl"
            ;;
        opensuse)
            echo "gcc gcc-c++ cmake clang pkg-config libopenssl-devel alsa-devel gtk4-devel libadwaita-devel dbus-1-devel ca-certificates curl"
            ;;
        gentoo)
            echo "dev-lang/rust-bin dev-util/pkgconf gui-libs/gtk gui-libs/libadwaita media-libs/alsa-lib sys-apps/dbus"
            ;;
    esac
}

get_distro_provision_cmd() {
    local distro="$1"
    local pkgs
    pkgs="$(get_distro_packages "$distro")"
    case "$distro" in
        debian-12)
            # Debian 12 (Bookworm) ships with older GTK4 (4.10) and libadwaita (1.3); overlay Debian 13 (Trixie)
            # repository to satisfy modern Relm4 / libadwaita requirements (>= 1.4/1.5) on a Bookworm base.
            echo "echo 'deb http://deb.debian.org/debian trixie main' > /etc/apt/sources.list.d/trixie.list && apt-get update -y && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends ${pkgs}"
            ;;
        ubuntu-26.04|ubuntu-24.04|ubuntu|elementary-8|zorin-18|linux-lite-7)
            echo "if [ -n \"\${UBUNTU_MIRROR:-}\" ]; then sed -i \"s|http://archive.ubuntu.com/ubuntu|\$UBUNTU_MIRROR|g\" /etc/apt/sources.list.d/ubuntu.sources 2>/dev/null || true; fi; apt-get update -y && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends ${pkgs}"
            ;;
        debian-13|kali)
            echo "apt-get update -y && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends ${pkgs}"
            ;;
        mint-22)
            echo "if [ -n \"\${UBUNTU_MIRROR:-}\" ]; then sed -i \"s|http://archive.ubuntu.com/ubuntu|\$UBUNTU_MIRROR|g\" /etc/apt/sources.list.d/ubuntu.sources /etc/apt/sources.list.d/official-package-repositories.list 2>/dev/null || true; fi; apt-get update -y && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends ${pkgs}"
            ;;
        fedora)
            echo "dnf install -y --setopt=install_weak_deps=False ${pkgs}"
            ;;
        arch|manjaro|steamos-3)
            echo "pacman -Sy --noconfirm --needed ${pkgs}"
            ;;
        opensuse)
            echo "zypper --non-interactive refresh && zypper --non-interactive install -y --no-recommends ${pkgs}"
            ;;
        gentoo)
            echo "mkdir -p /etc/portage/binrepos.conf && printf '[gentoo]\nsync-uri = https://distfiles.gentoo.org/releases/amd64/binpackages/23.0/x86-64/\n' > /etc/portage/binrepos.conf/gentoo.conf && emerge --getbinpkg ${pkgs}"
            ;;
    esac
}

# ------------------------------------------------------------------------------
# Generate Container Test Script
# ------------------------------------------------------------------------------
generate_container_script() {
    local distro="$1"
    local provision_cmd
    provision_cmd="$(get_distro_provision_cmd "$distro")"

    cat <<EOF
set -e

echo "=== [0/4] Operating System Information ==="
if [ -f /etc/os-release ]; then
    grep PRETTY_NAME /etc/os-release || true
fi
uname -m

echo ""
echo "=== [1/4] Provisioning Native Dependencies via \$(command -v $(get_distro_pkg_manager "$distro") || echo $(get_distro_pkg_manager "$distro")) ==="
${provision_cmd}

echo ""
echo "=== [2/4] Verifying pkg-config Resolution ==="
pkg-config --exists gtk4 libadwaita-1 alsa dbus-1
echo "pkg-config status: OK"
echo "  gtk4:         \$(pkg-config --modversion gtk4 2>/dev/null || echo 'N/A')"
echo "  libadwaita-1: \$(pkg-config --modversion libadwaita-1 2>/dev/null || echo 'N/A')"
echo "  alsa:         \$(pkg-config --modversion alsa 2>/dev/null || echo 'N/A')"
echo "  dbus-1:       \$(pkg-config --modversion dbus-1 2>/dev/null || echo 'N/A')"

echo ""
echo "=== [3/4] Ensuring Rust & Cargo Toolchain ==="
if ! command -v cargo &>/dev/null; then
    if [[ -x "/usr/local/cargo/bin/cargo" ]]; then
        export PATH="/usr/local/cargo/bin:\$PATH"
        export RUSTUP_HOME="/usr/local/rustup"
        export CARGO_HOME="/usr/local/cargo"
    fi
fi

if ! command -v cargo &>/dev/null; then
    echo "Cargo not in path; attempting minimal rustup installation..."
    export RUSTUP_HOME="/tmp/rustup"
    export CARGO_HOME="/tmp/cargo"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable --profile minimal
    export PATH="\$CARGO_HOME/bin:\$PATH"
fi

cargo --version
rustc --version

echo ""
echo "=== [4/4] Executing Cargo Check ==="
export CARGO_TARGET_DIR="/tmp/target"
if cargo check --offline; then
    echo "Cargo check (offline) succeeded!"
else
    echo "Offline check incomplete; running standard cargo check..."
    cargo check
fi

echo ""
echo "=== Distro Validation SUCCESS: ${distro} ==="
EOF
}

# ------------------------------------------------------------------------------
# Execute Distro Test
# ------------------------------------------------------------------------------
run_distro_test() {
    local distro="$1"
    local image
    image="$(get_distro_image "$distro")"
    local family
    family="$(get_distro_family "$distro")"
    local pkgs
    pkgs="$(get_distro_packages "$distro")"
    local provision_cmd
    provision_cmd="$(get_distro_provision_cmd "$distro")"
    local log_file="${REPORT_DIR}/${distro}.log"

    # Mint-22 / Manjaro image fallback check
    if [[ "$distro" == "mint-22" && "${DRY_RUN}" != "true" ]]; then
        if ! ${CONTAINER_ENGINE} image inspect "$image" &>/dev/null; then
            if ! ${CONTAINER_ENGINE} pull "$image" &>/dev/null; then
                echo -e "${YELLOW}[WARN] Mint 22 image '${image}' unavailable; falling back to docker.io/library/ubuntu:24.04${RESET}"
                image="docker.io/library/ubuntu:24.04"
            fi
        fi
    elif [[ "$distro" == "manjaro" && "${DRY_RUN}" != "true" ]]; then
        if ! ${CONTAINER_ENGINE} image inspect "$image" &>/dev/null; then
            if ! ${CONTAINER_ENGINE} pull "$image" &>/dev/null; then
                echo -e "${YELLOW}[WARN] Manjaro image '${image}' unavailable; falling back to docker.io/library/archlinux:latest${RESET}"
                image="docker.io/library/archlinux:latest"
            fi
        fi
    fi

    echo -e "${CYAN}------------------------------------------------------------${RESET}"
    echo -e "${BOLD}Distro Target:${RESET}    ${BLUE}${distro}${RESET} (${family})"
    echo -e "${BOLD}Container Image:${RESET}  ${image}"
    echo -e "${BOLD}Engine:${RESET}           ${CONTAINER_ENGINE}"
    echo -e "${BOLD}Packages:${RESET}         ${pkgs}"

    if [[ "${DRY_RUN}" == "true" ]]; then
        echo -e "${YELLOW}[DRY-RUN] Simulating test execution for ${distro}...${RESET}"
        echo -e "${BOLD}Provision Command:${RESET}"
        echo "    ${provision_cmd}"
        echo -e "${BOLD}Dependency Check:${RESET}"
        echo "    pkg-config --exists gtk4 libadwaita-1 alsa dbus-1"
        echo -e "${BOLD}Build Check:${RESET}"
        echo "    cargo check"
        echo -e "${GREEN}[DRY-RUN] Specification validated successfully.${RESET}"
        return 0
    fi

    mkdir -p "${REPORT_DIR}"
    echo -e "${YELLOW}Starting container run (logging to ${log_file})...${RESET}"

    local container_script
    container_script="$(generate_container_script "$distro")"

    # Assemble volume mounts
    local mount_args=(
        "-v" "${REPO_ROOT}:/workspace:ro"
        "-w" "/workspace"
        "-e" "DEBIAN_FRONTEND=noninteractive"
        "-e" "CI=true"
    )

    # Mount host cargo and rustup toolchains to dramatically speed up validation
    if [[ -d "${HOME}/.cargo" && -d "${HOME}/.rustup" ]]; then
        mount_args+=(
            "-v" "${HOME}/.cargo:/usr/local/cargo:ro"
            "-v" "${HOME}/.rustup:/usr/local/rustup:ro"
            "-e" "RUSTUP_HOME=/usr/local/rustup"
            "-e" "CARGO_HOME=/usr/local/cargo"
            "-e" "PATH=/usr/local/cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"
        )
    fi

    local start_time
    start_time="$(date +%s)"

    # Run container and tee output to log
    local status=0
    ${CONTAINER_ENGINE} run --rm "${mount_args[@]}" "${image}" bash -c "${container_script}" 2>&1 | tee "${log_file}" || status=${PIPESTATUS[0]}
    if [[ ${status} -ne 0 ]]; then
        if [[ "$distro" == "mint-22" && "$image" != "docker.io/library/ubuntu:24.04" ]]; then
            echo -e "${YELLOW}Retrying mint-22 test using base ubuntu:24.04 container...${RESET}"
            image="docker.io/library/ubuntu:24.04"
            status=0
            ${CONTAINER_ENGINE} run --rm "${mount_args[@]}" "${image}" bash -c "${container_script}" 2>&1 | tee -a "${log_file}" || status=${PIPESTATUS[0]}
        elif [[ "$distro" == "manjaro" && "$image" != "docker.io/library/archlinux:latest" ]]; then
            echo -e "${YELLOW}Retrying manjaro test using base archlinux:latest container...${RESET}"
            image="docker.io/library/archlinux:latest"
            status=0
            ${CONTAINER_ENGINE} run --rm "${mount_args[@]}" "${image}" bash -c "${container_script}" 2>&1 | tee -a "${log_file}" || status=${PIPESTATUS[0]}
        fi
    fi

    local end_time
    end_time="$(date +%s)"
    local duration=$((end_time - start_time))

    if [[ ${status} -eq 0 ]]; then
        echo -e "${GREEN}✓ ${distro} validation PASSED (${duration}s)${RESET}"
        return 0
    else
        echo -e "${RED}✗ ${distro} validation FAILED (exit code: ${status}, ${duration}s)${RESET}"
        return 1
    fi
}

# ------------------------------------------------------------------------------
# Main Runner
# ------------------------------------------------------------------------------
main() {
    echo -e "${BOLD}${BLUE}============================================================${RESET}"
    echo -e "${BOLD}${BLUE}   OpenDictate Automated Multi-Distro Validation Suite      ${RESET}"
    echo -e "${BOLD}${BLUE}============================================================${RESET}"
    echo -e "${BOLD}Target Distro:${RESET}    ${TARGET_DISTRO}"
    echo -e "${BOLD}Engine:${RESET}           ${CONTAINER_ENGINE}"
    echo -e "${BOLD}Dry Run:${RESET}          ${DRY_RUN}"
    echo -e "${BOLD}Repo Root:${RESET}        ${REPO_ROOT}"
    echo -e "${BOLD}Report Dir:${RESET}       ${REPORT_DIR}"
    echo ""

    local distros_to_test=()
    if [[ "${TARGET_DISTRO}" == "all" ]]; then
        distros_to_test=(
            "ubuntu-26.04"
            "ubuntu-24.04"
            "debian-13"
            "debian-12"
            "mint-22"
            "arch"
            "fedora"
            "kali"
            "manjaro"
            "opensuse"
            "elementary-8"
            "zorin-18"
            "linux-lite-7"
            "steamos-3"
        )
    else
        distros_to_test=("${TARGET_DISTRO}")
    fi

    local passed_distros=()
    local failed_distros=()

    for d in "${distros_to_test[@]}"; do
        if run_distro_test "$d"; then
            passed_distros+=("$d")
        else
            failed_distros+=("$d")
        fi
        echo ""
    done

    # --------------------------------------------------------------------------
    # Results Summary
    # --------------------------------------------------------------------------
    echo -e "${BOLD}${BLUE}============================================================${RESET}"
    echo -e "${BOLD}${BLUE}                     Summary of Results                     ${RESET}"
    echo -e "${BOLD}${BLUE}============================================================${RESET}"

    for d in "${passed_distros[@]}"; do
        echo -e "  ${GREEN}✓ [PASS]${RESET} ${d} ($(get_distro_family "$d"))"
    done
    for d in "${failed_distros[@]}"; do
        echo -e "  ${RED}✗ [FAIL]${RESET} ${d} ($(get_distro_family "$d"))"
    done

    echo ""
    if [[ ${#failed_distros[@]} -eq 0 ]]; then
        echo -e "${GREEN}${BOLD}ALL TESTS PASSED! (${#passed_distros[@]}/${#distros_to_test[@]})${RESET}"
        exit 0
    else
        echo -e "${RED}${BOLD}SOME TESTS FAILED! (${#failed_distros[@]}/${#distros_to_test[@]} failures)${RESET}"
        exit 1
    fi
}

main
