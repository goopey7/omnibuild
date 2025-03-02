use crate::{
    cli::Cli,
    lua_configuration::{build_target_config::BuildTargetConfig, project_config::ProjectConfig},
};
use std::path::Path;

pub fn find_and_compile_modules(
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
