use std::sync::RwLock;

use crate::lua::config::{build_target_config::BuildTargetConfig, module_config::ModuleConfig, project_config::ProjectConfig, target_configuration_config::BuildConfig};
use lazy_static::lazy_static;

#[derive(Default, Clone)]
pub struct BuildState
{
    pub project: Option<ProjectConfig>,
    pub modules: Vec<ModuleConfig>,
    pub configs: Vec<BuildConfig>,
    pub targets: Vec<BuildTargetConfig>,
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
