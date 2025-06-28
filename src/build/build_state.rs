use std::{path::PathBuf, sync::RwLock};

use crate::{cli::Cli, lua::config::{build_config::BuildConfig, module_config::ModuleConfig, project_config::ProjectConfig, target_config::TargetConfig}};
use lazy_static::lazy_static;

#[derive(Default, Clone)]
pub struct BuildState
{
    pub project: Option<ProjectConfig>,
    pub modules: Vec<ModuleConfig>,
    pub configs: Vec<BuildConfig>,
    pub targets: Vec<TargetConfig>,
    pub repo_url: String,
    pub args: Cli,
    pub working_directory: PathBuf,
    pub is_running_package_lua: bool,
}

lazy_static! {
    pub static ref BUILD_STATE: RwLock<BuildState> = RwLock::new(BuildState::default());
}

#[macro_export]
macro_rules! build_state {
    () => {
        $crate::build::build_state::BUILD_STATE.write().expect("failed to unwrap BUILD_STATE")
    };
}
