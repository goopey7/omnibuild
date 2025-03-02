mod build_modules;
mod cli;
mod load_lua_helpers;
mod lua_configuration;

use build_modules::find_and_compile_modules;
use clap::Parser;
use cli::Cli;
use load_lua_helpers::{load_build_target, load_project};

fn main() {
    let args = Cli::parse();
    let lua = mlua::Lua::new();

    let loaded_project = load_project(&args, &lua);
    let loaded_build_target_and_project = loaded_project.and_then(|project| {
        load_build_target(&args, &lua).map(|build_target| (project, build_target))
    });

    match loaded_build_target_and_project {
        Ok((project, build_target)) => {
            println!(
                "Compiling {} {} {} to {}",
                &project.project_name,
                &args.build_target,
                &args.target_configuration,
                args.build_directory.to_str().unwrap()
            );
            find_and_compile_modules(&args, &lua, &project, &build_target);
        }
        Err(error) => {
            println!("{}", error);
        }
    }
}
