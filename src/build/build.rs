use std::path::{Path, PathBuf};

use crate::build::build_state::BUILD_STATE;
use crate::cli::Cli;
use crate::compiler::compiler::Compiler;

fn run_assertions() {
    let build_state = BUILD_STATE
        .try_read()
        .expect("failed to get read on build_state");

    assert!(build_state.project.is_some(), "project must be set!");
    assert!(
        !build_state.modules.is_empty(),
        "no modules have been added!"
    );
    assert!(
        !build_state.targets.is_empty(),
        "no targets have been added!"
    );
    assert!(
        !build_state.configs.is_empty(),
        "no configs have been added!"
    );
}

fn gather_cpp_files(dir: &Path, files: &mut Vec<PathBuf>) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                gather_cpp_files(&path, files);
            } else if path.extension().map_or(false, |ext| ext == "cpp") {
                files.push(path);
            }
        }
    }
}

pub fn build<T: Compiler>(args: &Cli) {
    run_assertions();

    let build_state = BUILD_STATE
        .try_read()
        .expect("failed to get read on build_state");

    if let Some(project) = &build_state.project
    {
        println!("Building {} {}", project.project_name, project.project_version);
    }

    let target = build_state
        .targets
        .iter()
        .find(|target_config| target_config.name == args.build_target);
    let target = target.expect("provided target not found!");

    let build_config = build_state
        .configs
        .iter()
        .find(|build_config| build_config.name == args.build_config);
    let build_config = build_config.expect("provided build configuration not found!");

    let output_dir = target
        .output_dir
        .to_str()
        .expect("could not convert output path to string")
        .to_owned();
    build_state.modules.iter().for_each(|module| {
        let path = module.path.clone().expect("module path not found!");
        let path = path
            .to_str()
            .expect("could not convert module path to string")
            .to_owned();
        let path = format!("{}/{}", output_dir, path);


        let mut files = vec![];
        gather_cpp_files(Path::new(&module.path.as_ref().unwrap().clone()), &mut files);

        println!("building {} to {}", module.name, path);
        files.iter().for_each(|file| {
            T::compile(module, &target, &build_config, file);
        });
    });
}
