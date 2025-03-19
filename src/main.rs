mod build;
mod cli;
mod lua;

use build::{build::configure, collect_modules::collect_modules};
use clap::Parser;
use cli::Cli;
use lua::init_globals::init_globals;

fn main() -> Result<(), mlua::Error> {
    let args = Cli::parse();

    std::env::set_current_dir(&args.project_directory)?;

    let lua = mlua::Lua::new();
    init_globals(&lua)?;

    let init_file_read = std::fs::read_to_string("init.lua").expect("no init.lua found!");
    lua.load(init_file_read).exec()?;

    collect_modules(&lua, &args)?;

    configure(&lua, &args);

    Ok(())
}
