mod build;
mod cli;
mod lua;

use build::build_state::BuildState;
use clap::Parser;
use cli::Cli;
use lua::init_globals::init_globals;

fn main() -> Result<(), mlua::Error> {
    let args = Cli::parse();
    std::env::set_current_dir(&args.project_directory)?;

    let lua = mlua::Lua::new();
    init_globals(&lua, &args)?;

    let project_file_read = std::fs::read_to_string("init.lua").expect("no init.lua found!");
    lua.load(project_file_read).exec()?;

    Ok(())
}
