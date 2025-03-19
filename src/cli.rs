/// Omnibuild is a dead simple build system for large scale C++ projects
#[derive(clap::Parser)]
#[command(version, about)]
pub struct Cli {
    /// Directory containing project.lua
    #[arg(name = "project-directory", short = 'i', long, default_value = ".")]
    pub project_directory: std::path::PathBuf,

    /// Output directory for build binaries
    #[arg(name = "build-directory", short = 'o', long, default_value = "./build")]
    pub build_directory: std::path::PathBuf,

    /// Build target to compile
    #[arg(name = "build-target", short = 't', long)]
    pub build_target: String,

    /// build config to compile (Debug, Release)
    #[arg(name = "config", short = 'c', long)]
    pub build_config: String,

    /// build file to output (make, vs)
    #[arg(name = "file", short = 'f', long)]
    pub build_file: String,
}
