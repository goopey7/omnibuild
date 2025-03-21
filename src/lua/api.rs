use crate::build_state;

pub fn is_windows() -> bool {
    cfg!(windows)
}

pub fn is_unix() -> bool {
    cfg!(unix)
}

pub fn is_macos() -> bool {
    cfg!(target_os = "macos")
}

pub fn print(_lua: &mlua::Lua, str: String) {
    println!("{}", str);
}

pub fn set_project(_lua: &mlua::Lua, project: super::config::project_config::ProjectConfig) {
    build_state!().project = Some(project);
    println!("added project {:?}", build_state!().project);
}

pub fn add_module(_lua: &mlua::Lua, module: super::config::module_config::ModuleConfig) {
    build_state!().modules.push(module);
    println!("added module {:?}", build_state!().modules.last());
}

pub fn add_config(
    _lua: &mlua::Lua,
    config: super::config::target_configuration_config::BuildConfig,
) {
    build_state!().configs.push(config);
    println!("added build config {:?}", build_state!().configs.last());
}

pub fn add_target(_lua: &mlua::Lua, target: super::config::build_target_config::BuildTargetConfig) {
    build_state!().targets.push(target);
    println!("added build target {:?}", build_state!().targets.last());
}
