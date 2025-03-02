use crate::lua_configuration::build_target_config::BuildTargetConfig;
use crate::Cli;
use crate::lua_configuration::project_config::ProjectConfig;

pub fn load_project(args: &Cli, lua: &mlua::Lua) -> Result<ProjectConfig, mlua::Error> {
    let project_file_read = std::fs::read_to_string(&args.project_directory.join("project.lua"))
        .expect("no project.lua found!");

    lua.load(project_file_read).exec()?;
    let globals = lua.globals();
    let project_name = globals.get::<String>("project_name")?;
    let project_version = globals.get::<String>("project_version")?;

    Ok(ProjectConfig {
        project_name,
        project_version,
    })
}

pub fn load_build_target(args: &Cli, lua: &mlua::Lua) -> Result<BuildTargetConfig, mlua::Error> {
    let config_path = &args
        .project_directory
        .join("cfg")
        .join("build_targets")
        .join(format!("{}.lua", &args.build_target));

    let build_target_file_read = std::fs::read_to_string(config_path)
        .expect(format!("could not find build target config! {}", config_path.to_str().unwrap()).as_str());

    lua.load(build_target_file_read).exec()?;
    let globals = lua.globals();
    let module_directories = globals.get::<Vec<std::path::PathBuf>>("module_directories")?;

    Ok(BuildTargetConfig { module_directories })
}

