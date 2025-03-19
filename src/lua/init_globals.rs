use macros::add_lua_functions;
use crate::cli::Cli;

pub fn init_globals(lua: &mlua::Lua, args: &Cli) -> Result<(), mlua::Error> {
    let ob_table = lua.create_table()?;

    ob_table.set("build_directory", args.build_directory.clone())?;
    ob_table.set("target", args.build_target.clone())?;
    ob_table.set("config", args.build_config.clone())?;

    lua.globals().set("ob", &ob_table)?;
    add_lua_functions!();
    Ok(())
}
