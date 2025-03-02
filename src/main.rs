mod cli;
mod lua_configuration;

use clap::Parser;
use cli::Cli;
use lua_configuration::{build_target_config::BuildTargetConfig, project_config::ProjectConfig};
use std::path::{Path, PathBuf};

fn load_project(args: &Cli, lua: &mlua::Lua) -> Result<ProjectConfig, mlua::Error> {
    let project_file_read = std::fs::read_to_string(&args.project_directory.join("project.lua"))
        .expect("no project.lua found!");

    lua.load(project_file_read).exec()?;
    let globals = lua.globals();
    let project_name = globals.get::<String>("project_name")?;
    let project_version = globals.get::<String>("project_version")?;

    Ok(ProjectConfig {
        project_name,
        project_version,
    })
}

fn load_build_target(args: &Cli, lua: &mlua::Lua) -> Result<BuildTargetConfig, mlua::Error> {
    let config_path = &args
        .project_directory
        .join("cfg")
        .join("build_targets")
        .join(format!("{}.lua", &args.build_target));

    let build_target_file_read = std::fs::read_to_string(config_path)
        .expect(format!("could not find build target config! {}", config_path.to_str().unwrap()).as_str());

    lua.load(build_target_file_read).exec()?;
    let globals = lua.globals();
    let module_directories = globals.get::<Vec<PathBuf>>("module_directories")?;

    Ok(BuildTargetConfig { module_directories })
}

fn process_module(
    args: &Cli,
    module_search_directory: &std::path::PathBuf,
    module_path: Result<std::fs::DirEntry, std::io::Error>,
    build_target: &BuildTargetConfig,
) {
    if module_path.is_err() {
        return;
    }
    let module_path = module_path.unwrap();

    if module_path.file_type().is_err() {
        return;
    }

    if !module_path.file_type().unwrap().is_dir() {
        return;
    }

    let module_directory_name = module_path
        .file_name()
        .into_string()
        .expect("could not convert file_name to str");

    let full_module_relative_path = args
        .project_directory
        .join("src")
        .join(&module_search_directory)
        .join(&module_directory_name);

    println!("{:?}", &full_module_relative_path);

    let module_file_read = std::fs::read_to_string(&full_module_relative_path.join("module.lua"))
        .expect(
            format!(
                "{:?} is missing a module.lua file!",
                &full_module_relative_path
            )
            .as_str(),
        );

    // TODO load module.lua
}

fn find_and_compile_modules(
    args: &Cli,
    lua: &mlua::Lua,
    loaded_project: &ProjectConfig,
    build_target: &BuildTargetConfig,
) {
    for module_search_directory in build_target.module_directories.as_slice() {
        let module_list = std::fs::read_dir(
            Path::new("src")
                .join(&args.project_directory)
                .join(&module_search_directory),
        )
        .expect(format!("directory not found: {:?}", &module_search_directory).as_str());

        for module_path in module_list {
            process_module(args, module_search_directory, module_path, build_target);
        }
    }
}

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
