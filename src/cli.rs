/// Omnibuild is a dead simple build system for large scale C++ projects
#[derive(clap::Parser)]
#[command(version, about)]
pub struct Cli {
    /// Directory containing project.lua
    #[arg(short, long, default_value = ".")]
    pub directory: std::path::PathBuf,
}
