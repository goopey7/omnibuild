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
}

pub fn add_module(_lua: &mlua::Lua, module: super::config::module_config::ModuleConfig) {
    build_state!().modules.push(module);
}

pub fn add_config(
    _lua: &mlua::Lua,
    config: super::config::build_config::BuildConfig,
) {
    build_state!().configs.push(config);
}

pub fn add_target(_lua: &mlua::Lua, target: super::config::target_config::TargetConfig) {
    build_state!().targets.push(target);
}
