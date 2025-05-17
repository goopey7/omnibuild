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

pub fn print(str: String) {
    println!("{}", str);
}

pub fn set_project(project: super::config::project_config::ProjectConfig) {
    build_state!().project = Some(project);
}

pub fn add_module(module: super::config::module_config::ModuleConfig) {
    build_state!().modules.push(module);
}

pub fn add_config(config: super::config::build_config::BuildConfig) {
    build_state!().configs.push(config);
}

pub fn add_target(target: super::config::target_config::TargetConfig) {
    build_state!().targets.push(target);
}

pub fn cmd(cmd: Vec<String>) {
    if cmd.is_empty() {
        return;
    }
    std::process::Command::new(cmd.first().unwrap())
        .args(&cmd[1..])
        .status()
        .unwrap();
}

