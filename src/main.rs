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

fn main() {
    let args = Cli::parse();
    let lua = mlua::Lua::new();
    let loaded_project = load_project(&args.directory, &lua).unwrap();

    println!("Compiling {}...", &loaded_project.project_name);

    // find modules
    for module_search_directory in loaded_project.module_directories {
        let module_list = std::fs::read_dir(&args.directory.join(&module_search_directory.path))
            .expect(format!("directory not found: {}", &module_search_directory.path).as_str());

        for module_path in module_list {
            match module_path {
                Ok(entry) => {
                    match entry.file_type() {
                        Ok(file_type) => {
                            if !file_type.is_dir() {
                                continue;
                            }
                        }
                        Err(_) => {
                            continue;
                        }
                    }

                    let file_name = entry.file_name();
                    let file_name_str = file_name
                        .to_str()
                        .expect("could not convert file_name to str");

                    println!("{}/", file_name_str);

                    let module_file_read = std::fs::read_to_string(format!(
                        "{}/{}/{}/module.lua",
                        &args.directory.to_str().unwrap(),
                        &module_search_directory.path,
                        file_name_str,
                    ))
                    .expect(
                        format!(
                            "{}/{}/{} is missing a module.lua file!",
                            &args.directory.to_str().unwrap(),
                            &module_search_directory.path,
                            &file_name_str
                        )
                        .as_str(),
                    );
                }
                Err(_) => {
                    continue;
                }
            };
        }
    }
}
