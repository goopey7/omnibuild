use macros::add_lua_functions;

pub fn init_globals(lua: &mlua::Lua, build_target: &str) -> Result<(), mlua::Error> {
    let ob_table = lua.create_table()?;
    ob_table.set("target", build_target)?;

    lua.globals().set("ob", &ob_table)?;
    add_lua_functions!();
    Ok(())
}
