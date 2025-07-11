use std::path::{Path, PathBuf};

use crate::build::build_state::BUILD_STATE;
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

fn gather_cpp_files(dir: &Path, files: &mut Vec<PathBuf>, ignore_dirs: &Vec<String>) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let mut should_ignore = false;
                for ignore_dir in ignore_dirs {
                    let ignore_name = Path::new(ignore_dir)
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or(ignore_dir);

                    if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                        if dir_name == ignore_name {
                            should_ignore = true;
                            break;
                        }
                    }
                }
                if should_ignore {
                    continue;
                }
                gather_cpp_files(&path, files, ignore_dirs);
            } else if path.extension().map_or(false, |ext| ext == "cpp") {
                files.push(path);
            }
        }
    }
}

pub fn build<T: Compiler>() {
    run_assertions();

    let build_state = BUILD_STATE
        .try_read()
        .expect("failed to get read on build_state");

    if let Some(project) = &build_state.project {
        println!(
            "Building {} {}",
            project.project_name, project.project_version
        );
    }

    let target = build_state
        .targets
        .iter()
        .find(|target_config| target_config.name == build_state.args.build_target);
    let target = target.expect("provided target not found!");

    let build_config = build_state
        .configs
        .iter()
        .find(|build_config| build_config.name == build_state.args.build_config);
    let build_config = build_config.expect("provided build configuration not found!");

    let mut compile_commands = vec![];
    build_state.modules.iter().for_each(|module| {
        if module.path.is_none() {
            return;
        }

        let mut files = vec![];

        gather_cpp_files(
            Path::new(&module.path.as_ref().unwrap().clone()),
            &mut files,
            &module.ignore_dirs,
        );

        files.iter().for_each(|file| {
            let compile_command = T::compile(module, &target, &build_config, file);
            compile_commands.push(compile_command);
        });
        let object_files: Vec<PathBuf> = files
            .iter()
            .map(|f| {
                let mut o = f.clone();
                o.set_extension("o");
                target.output_dir.join(o)
            })
            .collect();
        T::link_module(module, target, object_files);
    });
    let compile_commands_json = serde_json::to_string_pretty(&compile_commands)
        .expect("unable to serialize compile commands to json!");
    std::fs::write("compile_commands.json", compile_commands_json)
        .expect("failed to write compile_commands.json!");
}
