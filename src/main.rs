mod build_modules;
mod cli;
mod load_lua_helpers;
mod lua_api;
mod lua_configuration;

use clap::Parser;
use cli::Cli;
use lua_configuration::init_globals::init_globals;

fn main() -> Result<(), mlua::Error> {
    let args = Cli::parse();
    let lua = mlua::Lua::new();
    init_globals(&lua)?;

    // find init.lua
    let project_file_read = std::fs::read_to_string(&args.project_directory.join("init.lua"))
        .expect("no init.lua found!");
    lua.load(project_file_read).exec()?;

    Ok(())
}
