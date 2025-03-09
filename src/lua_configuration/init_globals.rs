use super::{
    build_target_config::BuildTargetConfig, module_config::ModuleConfig,
    project_config::ProjectConfig, target_configuration_config::TargetConfigurationConfig,
};

pub fn init_globals(lua: &mlua::Lua) -> Result<(), mlua::Error> {
    let ob_table = lua.create_table()?;

    let set_project = lua.create_function(|_, project: ProjectConfig| {
        println!("project ahhh");
        Ok(())
    })?;
    ob_table.set("set_project", set_project);

    let add_module = lua.create_function(|_, module: ModuleConfig| {
        println!("module ahhh");
        Ok(())
    })?;
    ob_table.set("add_module", add_module);

    let add_config = lua.create_function(|_, target: TargetConfigurationConfig| {
        println!("config ahhh");
        Ok(())
    })?;
    ob_table.set("add_config", add_config);

    let add_target = lua.create_function(|_, target: BuildTargetConfig| {
        println!("target ahhh");
        Ok(())
    })?;
    ob_table.set("add_target", add_target);

    let is_windows = lua.create_function(|_, ()| {
        Ok(cfg!(windows))
    })?;
    ob_table.set("is_windows", is_windows);

    let is_unix = lua.create_function(|_, ()| {
        Ok(cfg!(unix))
    })?;
    ob_table.set("is_unix", is_unix);

    let is_macos = lua.create_function(|_, ()| {
        Ok(cfg!(target_os = "macos"))
    })?;
    ob_table.set("is_macos", is_macos);

    lua.globals().set("ob", ob_table)?;
    Ok(())
}
