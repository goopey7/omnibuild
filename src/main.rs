mod cli;
mod lua_configuration;

use clap::Parser;
use cli::Cli;
use lua_configuration::project_config::{ModuleDirectory, ProjectConfig};

fn load_project(
    project_directory: &std::path::PathBuf,
    lua: &mlua::Lua,
) -> Result<ProjectConfig, mlua::Error> {
    let project_file_read = std::fs::read_to_string(project_directory.join("project.lua"))
        .expect("no project.lua found!");

    lua.load(project_file_read).exec()?;
    let globals = lua.globals();
    let project_name = globals.get::<String>("project_name")?;
    let project_version = globals.get::<String>("project_version")?;
    let build_targets = globals.get::<Vec<String>>("build_targets")?;
    let module_directories = globals.get::<Vec<ModuleDirectory>>("module_directories")?;

    Ok(ProjectConfig {
        project_name,
        project_version,
        build_targets,
        module_directories,
    })
}

fn process_module(
    args: &Cli,
    module_path: Result<std::fs::DirEntry, std::io::Error>,
    module_search_directory: &ModuleDirectory,
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
        .join(&module_search_directory.path)
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

fn find_and_compile_modules(args: &Cli, loaded_project: &ProjectConfig) {
    for module_search_directory in loaded_project.module_directories.as_slice() {
        let module_list = std::fs::read_dir(
            &args.project_directory.join(&module_search_directory.path),
        )
        .expect(format!("directory not found: {:?}", &module_search_directory.path).as_str());

        for module_path in module_list {
            process_module(args, module_path, module_search_directory);
        }
    }
}

fn main() {
    let args = Cli::parse();
    let lua = mlua::Lua::new();
    let loaded_project = load_project(&args.project_directory, &lua);

    match loaded_project {
        Ok(project) => {
            println!(
                "Compiling {} to {}",
                &project.project_name,
                args.build_directory.to_str().unwrap()
            );
            find_and_compile_modules(&args, &project);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
