use std::sync::Mutex;

use crate::lua::config::{build_target_config::BuildTargetConfig, module_config::ModuleConfig, project_config::ProjectConfig, target_configuration_config::BuildConfig};
use lazy_static::lazy_static;

#[derive(Default)]
pub struct BuildState
{
    pub project: Option<ProjectConfig>,
    pub modules: Vec<ModuleConfig>,
    pub configs: Vec<BuildConfig>,
    pub targets: Vec<BuildTargetConfig>,
}

lazy_static! {
    pub static ref BUILD_STATE: Mutex<BuildState> = Mutex::new(BuildState::default());
}

#[macro_export]
macro_rules! build_state {
    () => {
        $crate::build::build_state::BUILD_STATE.lock().expect("failed to unwrap BUILD_STATE")
    };
}
