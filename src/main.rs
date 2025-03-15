mod cli;
mod lua;

use lua::init_globals::init_globals;
use clap::Parser;
use cli::Cli;

fn main() -> Result<(), mlua::Error> {
    let args = Cli::parse();
    std::env::set_current_dir(&args.project_directory)?;
    let lua = mlua::Lua::new();
    init_globals(&lua)?;

    let project_file_read = std::fs::read_to_string("init.lua").expect("no init.lua found!");
    lua.load(project_file_read).exec()?;

    Ok(())
}
