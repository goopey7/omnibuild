use macros::add_lua_functions;

pub fn init_globals(lua: &mlua::Lua) -> Result<(), mlua::Error> {
    let ob_table = lua.create_table()?;
    lua.globals().set("ob", &ob_table)?;
    add_lua_functions!();
    Ok(())
}
