//! Build script ensuring portable GLIBC symbol compatibility across Linux distributions.
//!
//! When compiling on newer toolchains (such as Ubuntu 25.04+/26.04 with glibc 2.43),
//! C/C++ dependencies like `whisper.cpp` map `log10f` to `log10f@GLIBC_2.43` and
//! `strtol` to `__isoc23_strtol@GLIBC_2.38`. Wrapping those two symbols to delegate
//! to `log10@GLIBC_2.2.5` and `strtol@GLIBC_2.2.5` keeps the binary's required
//! strong GLIBC baseline at `GLIBC_2.34` (Ubuntu 22.04+, Debian 12+, Fedora 36+,
//! Flatpak GNOME 46, and Snap `core24`).

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    #[cfg(all(target_os = "linux", target_env = "gnu"))]
    {
        println!("cargo:rustc-link-arg=-Wl,--wrap=log10f");
        println!("cargo:rustc-link-arg=-Wl,--wrap=__isoc23_strtol");
        println!("cargo:rustc-link-arg=-Wl,--wrap=pidfd_spawnp");
        println!("cargo:rustc-link-arg=-Wl,--wrap=pidfd_getpid");
    }
}
