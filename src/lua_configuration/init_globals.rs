use super::{
    build_target_config::BuildTargetConfig, module_config::ModuleConfig,
    project_config::ProjectConfig, target_configuration_config::BuildConfig,
};

pub fn add_lua_function<
    F: Fn(&mlua::Lua, A) -> Result<R, mlua::Error> + mlua::MaybeSend + 'static,
    A: mlua::FromLuaMulti,
    R: mlua::IntoLuaMulti,
>(
    lua: &mlua::Lua,
    name: &str,
    f: F,
) -> Result<(), mlua::Error> {
    let lua_func = lua.create_function(f)?;
    lua.globals()
        .get::<mlua::Table>("ob")?
        .set(name, lua_func)?;
    Ok(())
}

pub fn init_globals(lua: &mlua::Lua) -> Result<(), mlua::Error> {
    let ob_table = lua.create_table()?;
    lua.globals().set("ob", ob_table)?;

    add_lua_function(
        lua,
        "set_project",
        |_: &mlua::Lua, project: ProjectConfig| -> Result<(), mlua::Error> {
            println!("added project {:?}", project);
            Ok(())
        },
    )?;

    add_lua_function(
        lua,
        "add_module",
        |_: &mlua::Lua, module: ModuleConfig| -> Result<(), mlua::Error> {
            println!("added module {:?}", module);
            Ok(())
        },
    )?;

    add_lua_function(
        lua,
        "add_config",
        |_: &mlua::Lua, target: BuildConfig| -> Result<(), mlua::Error> {
            println!("added build config {:?}", target);
            Ok(())
        },
    )?;

    add_lua_function(
        lua,
        "add_target",
        |_: &mlua::Lua, target: BuildTargetConfig| -> Result<(), mlua::Error> {
            println!("added build target {:?}", target);
            Ok(())
        },
    )?;

    add_lua_function(
        lua,
        "is_windows",
        |_: &mlua::Lua, ()| -> Result<bool, mlua::Error> { Ok(cfg!(windows)) },
    )?;

    add_lua_function(
        lua,
        "is_unix",
        |_: &mlua::Lua, ()| -> Result<bool, mlua::Error> { Ok(cfg!(unix)) },
    )?;

    add_lua_function(
        lua,
        "is_macos",
        |_: &mlua::Lua, ()| -> Result<bool, mlua::Error> { Ok(cfg!(target_os = "macos")) },
    )?;

    add_lua_function(lua, "print", |_, str: String| -> Result<(), mlua::Error> {
        println!("{}", str);
        Ok(())
    })?;
    Ok(())
}
