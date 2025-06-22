mod build;
mod cli;
mod compiler;
mod lua;

use build::{build::build, collect_modules::collect_modules};
use clap::Parser;
use cli::Cli;
use compiler::gcc::Gcc;
use lua::init_globals::init_globals;

fn main() -> Result<(), mlua::Error> {
    let args = Cli::parse();
    std::env::set_current_dir(&args.project_directory)?;

    build_state!().args = args;
    build_state!().working_directory = std::env::current_dir().unwrap();

    let lua = mlua::Lua::new();
    init_globals(&lua)?;

    let init_file_read = std::fs::read_to_string("init.lua").expect("no init.lua found!");
    lua.load(init_file_read).exec()?;

    collect_modules(&lua)?;

    build::<Gcc>();

    Ok(())
}
