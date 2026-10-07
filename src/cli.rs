//! Command-line argument parsing for OpenDictate.

/// Command-line argument flags supported by OpenDictate.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CliArgs {
    /// Launch directly showing the minibar floating island.
    pub minibar: bool,
    /// Launch directly showing the main dashboard window.
    pub dashboard: bool,
    /// Toggle dictation recording on the running instance or launch.
    pub toggle: bool,
    /// Print version information and exit.
    pub version: bool,
    /// Print help information and exit.
    pub help: bool,
}

impl CliArgs {
    /// Parses CLI flags from any iterator of string references.
    pub fn parse_from<I, T>(args: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: AsRef<str>,
    {
        let mut cli = Self::default();
        for arg in args {
            match arg.as_ref() {
                "--minibar" | "-m" => cli.minibar = true,
                "--dashboard" | "-d" => cli.dashboard = true,
                "--toggle" | "-t" => cli.toggle = true,
                "--version" | "-v" => cli.version = true,
                "--help" | "-h" => cli.help = true,
                _ => {}
            }
        }
        cli
    }

    /// Parses CLI flags from process arguments (`std::env::args()`).
    pub fn parse() -> Self {
        Self::parse_from(std::env::args().skip(1))
    }
}
