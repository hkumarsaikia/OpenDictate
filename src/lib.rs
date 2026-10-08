//! OpenDictate: Native Linux voice dictation and AI speech-to-text application.

pub mod audio;
pub mod cli;
pub mod config;
pub mod services;
pub mod ui;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(all(target_os = "linux", target_env = "gnu"))]
mod glibc_compat {
    use std::ffi::{c_char, c_int, c_long, c_void};

    unsafe extern "C" {
        fn log10(x: f64) -> f64;
        fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
        fn __errno_location() -> *mut c_int;
    }

    const ENOSYS: c_int = 38;

    /// Redirects `log10f@GLIBC_2.43` calls from C/C++ dependencies to `log10@GLIBC_2.2.5`.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn __wrap_log10f(x: f32) -> f32 {
        unsafe { log10(f64::from(x)) as f32 }
    }

    /// Redirects `__isoc23_strtol@GLIBC_2.38` calls from C/C++ dependencies to `strtol@GLIBC_2.2.5`.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn __wrap___isoc23_strtol(
        nptr: *const c_char,
        endptr: *mut *mut c_char,
        base: c_int,
    ) -> c_long {
        unsafe { strtol(nptr, endptr, base) }
    }

    /// Redirects `pidfd_getpid@GLIBC_2.39` so `std::process::Command` falls back to `posix_spawnp@GLIBC_2.15`.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn __wrap_pidfd_getpid(_fd: c_int) -> c_int {
        unsafe {
            *__errno_location() = ENOSYS;
        }
        -1
    }

    /// Redirects `pidfd_spawnp@GLIBC_2.39` so `std::process::Command` falls back to `posix_spawnp@GLIBC_2.15`.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn __wrap_pidfd_spawnp(
        _pidfd: *mut c_int,
        _file: *const c_char,
        _file_actions: *const c_void,
        _attrp: *const c_void,
        _argv: *const *mut c_char,
        _envp: *const *mut c_char,
    ) -> c_int {
        ENOSYS
    }
}

