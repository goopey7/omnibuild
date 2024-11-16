mod lua_configuration;

fn main() {
    let project_directory = "../goop2/engine";

    // look for project.lua file
    let project_file_read =
        std::fs::read_to_string(String::from(project_directory) + "/project.lua")
            .expect("no project.lua found!");

    let lua = mlua::Lua::new();
    let globals = lua.globals();

    lua.load(project_file_read).exec().unwrap();

    println!(
        "Compiling {}",
        globals
            .get::<String>("project_name")
            .unwrap_or("project".to_string())
    );

    // find modules
    let module_search_directories = globals
        .get::<Vec<lua_configuration::project_config::ModuleDirectory>>("module_directories")
        .unwrap();

    for module_search_directory in module_search_directories {
        let module_list = std::fs::read_dir(
            String::from(project_directory) + "/" + &module_search_directory.path,
        )
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
                        &project_directory, &module_search_directory.path, file_name_str,
                    ))
                    .expect(
                        format!(
                            "{}/{}/{} is missing a module.lua file!",
                            &project_directory, &module_search_directory.path, &file_name_str
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
