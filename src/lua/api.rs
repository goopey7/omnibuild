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

pub fn set_project(lua: &mlua::Lua, project: super::config::project_config::ProjectConfig) {
    println!("added project {:?}", project);
}

pub fn add_module(lua: &mlua::Lua, module: super::config::module_config::ModuleConfig) {
    println!("added module {:?}", module);
}

pub fn add_config(lua: &mlua::Lua, target: super::config::target_configuration_config::BuildConfig) {
    println!("added build config {:?}", target);
}

pub fn add_target(lua: &mlua::Lua, target: super::config::build_target_config::BuildTargetConfig) {
    println!("added build target {:?}", target);
}

pub fn build(lua: &mlua::Lua, path: String) {}
