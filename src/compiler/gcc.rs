use std::{
    collections::HashSet,
    fs::{File, OpenOptions},
    hash::Hasher,
    io::{BufReader, Read, Write},
    path::{Path, PathBuf},
    process::Command,
};

use regex::Regex;
use twox_hash::XxHash3_64;

use super::{
    clangd::CompileCommand,
    compiler::{stringify_command, Compiler},
};
use crate::{
    build::build_state::{BuildState, BUILD_STATE},
    compiler::inc_build_cache::{IncBuildDependency, IncBuildFile},
    lua::config::{
        build_config::{BuildConfig, CppWarning, Optimization},
        module_config::{ModuleConfig, ModuleType},
        target_config::TargetConfig,
    },
};

fn hash_file_xxh3(file_path: &str) -> Result<u64, std::io::Error> {
    let mut file = BufReader::new(File::open(file_path)?);
    let mut hasher = XxHash3_64::with_seed(0);

    let mut buffer = [0; 8192];
    while let Ok(bytes_read) = file.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        hasher.write(&buffer[..bytes_read]);
    }

    Ok(hasher.finish())
}

fn parse_gcc_output(output: &str) -> Option<Vec<PathBuf>> {
    let output = output
        .replace("\\", "")
        .replace("\n", " ")
        .trim()
        .to_string();

    let re = Regex::new(r"([^\s]+):\s*(.+)").unwrap();

    if let Some(caps) = re.captures(&output) {
        let dependencies = caps.get(2).map_or("", |m| m.as_str());
        let mut dependencies: Vec<&str> = dependencies.split_whitespace().collect();
        dependencies.remove(0);
        return Some(
            dependencies
                .iter()
                .map(|path| {
                    let mut p = PathBuf::new();
                    p.push(path);
                    p
                })
                .collect(),
        );
    }
    None
}

pub struct Gcc {}

impl Compiler for Gcc {
    fn compile(
        module: &ModuleConfig,
        target_config: &TargetConfig,
        build_config: &BuildConfig,
        file: &PathBuf,
    ) -> CompileCommand {
        let build_state = BUILD_STATE
            .try_read()
            .expect("failed to get read on build_state");

        let mut cmd = Command::new("g++");

        let standard: u8 = build_config.cpp_standard.clone().into();
        cmd.arg(format!("{}{}", Gcc::get_standard_prefix(), standard));

        cmd.arg(Gcc::get_optimization(&build_config.optimization));

        build_config.warnings.iter().for_each(|warning| {
            cmd.arg(Gcc::get_warning(&warning));
        });

        if build_config.warnings_as_errors {
            cmd.arg(Gcc::get_warnings_as_errors());
        }

        target_config.definitions.iter().for_each(|def| {
            cmd.arg(format!(
                "{}{}={}",
                Gcc::get_definition_prefix(),
                def.0,
                def.1
            ));
        });

        module.dependencies.iter().for_each(|dep| {
            let module_dep = build_state
                .modules
                .iter()
                .find(|module| module.name == *dep);
            let module_dep = module_dep.expect("could not find module dependency!");
            if let Some(dep_path) = module_dep.path.clone() {
                let dep_path = dep_path.to_str().unwrap();
                module_dep.include_dirs.iter().for_each(|dir| {
                    cmd.arg(format!(
                        "{}{}/{}",
                        Gcc::get_include_dir_prefix(),
                        dep_path,
                        dir
                    ));
                });
            } else {
                module_dep.include_dirs.iter().for_each(|dir| {
                    cmd.arg(format!(
                        "{}{}/{}",
                        Gcc::get_include_dir_prefix(),
                        format!(".packages/{}", &module_dep.name),
                        dir
                    ));
                });
            }
        });

        let module_path = module.path.clone().unwrap();
        let module_path = module_path.to_str().unwrap();

        module.include_dirs.iter().for_each(|dir| {
            cmd.arg(format!(
                "{}{}/{}",
                Gcc::get_include_dir_prefix(),
                module_path,
                dir
            ));
        });

        if build_config.debug_info {
            cmd.arg(Gcc::get_debug_symbols());
        }

        if let Some(r#type) = &module.r#type {
            match r#type {
                ModuleType::Dylib => {
                    cmd.arg("-fPIC");
                }
                _ => {}
            }
        }

        cmd.arg(Gcc::input_src_file_prefix());
        cmd.arg(format!("{}", file.display()));

        let mut o_file = file.clone();
        o_file.set_extension("o");

        let out_path = format!(
            "{}/{}",
            target_config.output_dir.display(),
            o_file.parent().unwrap().display()
        );
        Command::new("mkdir")
            .arg("-p")
            .arg(&out_path)
            .status()
            .expect("mkdir failed!");

        cmd.arg(Gcc::output_obj_file_prefix());
        cmd.arg(format!(
            "{}/{}",
            target_config.output_dir.display(),
            o_file.display()
        ));

        let mut should_compile = true;

        let new_cache_state = get_inc_build_file(module, &file);
        if let Some(file) = get_inc_build_cache_state(&file, &target_config.output_dir) {
            let dependencies_match = file
                .dependencies
                .iter()
                .eq(new_cache_state.dependencies.iter());
            should_compile = !dependencies_match || file.hash != new_cache_state.hash;
        }

        if should_compile {
            cmd.status().unwrap();
        }
        update_inc_build_cache(&module, &file, &target_config.output_dir);

        let directory = std::env::current_dir().unwrap();
        CompileCommand {
            directory: directory.to_string_lossy().to_string(),
            command: stringify_command(&cmd),
            file: directory.join(file).to_string_lossy().to_string(),
        }
    }

    fn link_module(
        module: &ModuleConfig,
        target_config: &TargetConfig,
        object_files: Vec<PathBuf>,
    ) {
        let build_state = BUILD_STATE
            .try_read()
            .expect("failed to get read on build_state");

        let output_dir = target_config.output_dir.clone();
        if module.r#type.is_none() {
            return;
        }

        let r#type = module.r#type.as_ref().unwrap();
        let output_path = match r#type {
            ModuleType::Lib => output_dir.join(format!("lib{}.a", module.name)),
            ModuleType::Dylib => output_dir.join(format!("lib{}.so", module.name)),
            ModuleType::Exe => output_dir.join(&module.name),
        };

        fn collect_transitive_deps(
            name: &str,
            build_state: &BuildState,
            visited: &mut HashSet<String>,
            static_libs: &mut Vec<PathBuf>,
            dynamic_libs: &mut Vec<String>,
            output_dir: &Path,
        ) {
            if !visited.insert(name.to_string()) {
                return;
            }

            let dep = build_state
                .modules
                .iter()
                .find(|m| m.name == name)
                .expect("missing dep");

            for dep_name in &dep.dependencies {
                collect_transitive_deps(
                    dep_name,
                    build_state,
                    visited,
                    static_libs,
                    dynamic_libs,
                    output_dir,
                );
            }

            if let Some(r#type) = &dep.r#type {
                match r#type {
                    ModuleType::Lib => {
                        static_libs.push(output_dir.join(format!("lib{}.a", dep.name)));
                    }
                    ModuleType::Dylib => {
                        dynamic_libs.push(dep.name.clone());
                    }
                    _ => {}
                }
            }
        }

        match r#type {
            ModuleType::Lib => {
                let mut cmd = Command::new("ar");
                cmd.arg("rcs");
                cmd.arg(&output_path);
                for obj in &object_files {
                    cmd.arg(obj);
                }

                cmd.status().expect("Failed to link static library");
            }

            ModuleType::Dylib => {
                let mut cmd = Command::new("g++");
                cmd.arg("-shared").arg("-fPIC");
                cmd.args(&object_files);

                let mut visited = HashSet::new();
                let mut static_libs = Vec::new();
                let mut dynamic_libs = Vec::new();

                for dep_name in &module.dependencies {
                    collect_transitive_deps(
                        dep_name,
                        &build_state,
                        &mut visited,
                        &mut static_libs,
                        &mut dynamic_libs,
                        &output_dir,
                    );
                }

                for lib in &static_libs {
                    cmd.arg(lib);
                }

                if !dynamic_libs.is_empty() {
                    cmd.arg(format!("-L{}", output_dir.display()));
                    for lib in &dynamic_libs {
                        cmd.arg(format!("-l{}", lib));
                    }
                    cmd.arg(format!("-Wl,-rpath,{}", output_dir.display()));
                }

                cmd.arg("-o").arg(&output_path);
                cmd.status().expect("Failed to link shared library");
            }

            ModuleType::Exe => {
                let mut cmd = Command::new("g++");
                cmd.args(&object_files);

                let mut visited = HashSet::new();
                let mut static_libs = Vec::new();
                let mut dynamic_libs = Vec::new();

                for dep_name in &module.dependencies {
                    collect_transitive_deps(
                        dep_name,
                        &build_state,
                        &mut visited,
                        &mut static_libs,
                        &mut dynamic_libs,
                        &output_dir,
                    );
                }

                for lib in &static_libs {
                    cmd.arg(lib);
                }

                if !dynamic_libs.is_empty() {
                    cmd.arg(format!("-L{}", output_dir.display()));
                    for lib in &dynamic_libs {
                        cmd.arg(format!("-l{}", lib));
                    }
                    cmd.arg(format!("-Wl,-rpath,{}", output_dir.display()));
                }

                cmd.arg("-o").arg(&output_path);
                cmd.status().expect("Failed to link executable");
            }
        }

        println!("Output written to: {}", output_path.display());
    }

    fn get_warning(warning: &CppWarning) -> &'static str {
        match warning {
            CppWarning::All => "-Wall",
            CppWarning::Extra => "-Wextra",
            CppWarning::Pedantic => "-Wpedantic",
            CppWarning::Conversion => "-Wconversion",
            CppWarning::Shadow => "-Wshadow",
            CppWarning::OldStyleCast => "-Wold-style-cast",
            CppWarning::FloatEqual => "-Wfloat-equal",
            CppWarning::ExtraSemi => "-Wextra-semi",
            CppWarning::NonVirtualDtor => "-Wnon-virtual-dtor",
            CppWarning::OverloadedVirtual => "-Woverloaded-virtual",
            CppWarning::StrictNullSentinel => "-Wstrict-null-sentinel",
            CppWarning::ZeroAsNullPointerConstant => "-Wzero-as-null-pointer-constant",
        }
    }

    fn get_optimization(optimization: &Optimization) -> &'static str {
        match optimization {
            Optimization::None => "-O0",
            Optimization::Size => "-Os",
            Optimization::Speed => "-O2",
            Optimization::MaxSpeed => "-O3",
            Optimization::MaxSize => "-Oz",
        }
    }

    fn get_warnings_as_errors() -> &'static str {
        "-Werror"
    }

    fn get_debug_symbols() -> &'static str {
        "-g"
    }

    fn get_standard_prefix() -> &'static str {
        "-std=c++"
    }

    fn get_include_dir_prefix() -> &'static str {
        "-I"
    }

    fn get_definition_prefix() -> &'static str {
        "-D"
    }

    fn output_obj_file_prefix() -> &'static str {
        "-o"
    }

    fn input_src_file_prefix() -> &'static str {
        "-c"
    }
}

fn get_inc_build_cache_state(o_file: &PathBuf, output_dir: &PathBuf) -> Option<IncBuildFile> {
    let path = output_dir.join("cache.json");

    let cache_exists = std::fs::exists(&path).expect("failed to check path");
    let mut cache_file = if cache_exists {
        OpenOptions::new().read(true).open(&path).unwrap()
    } else {
        File::create_new(&path).unwrap()
    };

    let mut json_str = String::default();
    cache_file.read_to_string(&mut json_str).unwrap();

    let json = serde_json::from_str::<Vec<IncBuildFile>>(&json_str).unwrap_or(vec![]);

    let json = json
        .iter()
        .find(|found_file| found_file.file == o_file.to_str().unwrap());
    match json {
        Some(json) => Some(json.clone()),
        None => None,
    }
}

fn get_inc_build_file(module: &ModuleConfig, file: &PathBuf) -> IncBuildFile {
    let build_state = BUILD_STATE
        .try_read()
        .expect("failed to get read on build_state");

    let mut cmd = Command::new("g++");
    cmd.arg("-MM").arg(file);

    module.dependencies.iter().for_each(|dep| {
        let module_dep = build_state
            .modules
            .iter()
            .find(|module| module.name == *dep);
        let module_dep = module_dep.expect("could not find module dependency!");
        if let Some(dep_path) = module_dep.path.clone() {
            let dep_path = dep_path.to_str().unwrap();
            module_dep.include_dirs.iter().for_each(|dir| {
                cmd.arg(format!(
                    "{}{}/{}",
                    Gcc::get_include_dir_prefix(),
                    dep_path,
                    dir
                ));
            });
        } else {
            module_dep.include_dirs.iter().for_each(|dir| {
                cmd.arg(format!(
                    "{}{}/{}",
                    Gcc::get_include_dir_prefix(),
                    format!(".packages/{}", &module_dep.name),
                    dir
                ));
            });
        }
    });

    let output = cmd.output().unwrap();
    let output = std::str::from_utf8(&output.stdout).unwrap();
    let deps = parse_gcc_output(output).unwrap_or(vec![]);
    let deps = deps
        .iter()
        .map(|dep| IncBuildDependency {
            file: dep.display().to_string(),
            hash: hash_file_xxh3(&dep.display().to_string()).unwrap(),
        })
        .collect::<Vec<_>>();

    let hash = hash_file_xxh3(&file.display().to_string()).unwrap();

    IncBuildFile {
        file: file.to_str().unwrap().to_owned(),
        hash,
        dependencies: deps,
    }
}

fn update_inc_build_cache(module: &ModuleConfig, file: &PathBuf, output_dir: &PathBuf) {
    let build_state = BUILD_STATE
        .try_read()
        .expect("failed to get read on build_state");

    let mut cmd = Command::new("g++");
    cmd.arg("-MM").arg(file);

    module.dependencies.iter().for_each(|dep| {
        let module_dep = build_state
            .modules
            .iter()
            .find(|module| module.name == *dep);
        let module_dep = module_dep.expect("could not find module dependency!");
        if let Some(dep_path) = module_dep.path.clone() {
            let dep_path = dep_path.to_str().unwrap();
            module_dep.include_dirs.iter().for_each(|dir| {
                cmd.arg(format!(
                    "{}{}/{}",
                    Gcc::get_include_dir_prefix(),
                    dep_path,
                    dir
                ));
            });
        } else {
            module_dep.include_dirs.iter().for_each(|dir| {
                cmd.arg(format!(
                    "{}{}/{}",
                    Gcc::get_include_dir_prefix(),
                    format!(".packages/{}", &module_dep.name),
                    dir
                ));
            });
        }
    });

    let output = cmd.output().unwrap();
    let output = std::str::from_utf8(&output.stdout).unwrap();
    let deps = parse_gcc_output(output).unwrap_or(vec![]);
    let deps = deps
        .iter()
        .map(|dep| IncBuildDependency {
            file: dep.display().to_string(),
            hash: hash_file_xxh3(&dep.display().to_string()).unwrap(),
        })
        .collect::<Vec<_>>();

    let hash = hash_file_xxh3(&file.display().to_string()).unwrap();
    let path = output_dir.join("cache.json");

    let cache_exists = std::fs::exists(&path).expect("failed to check path");
    let mut cache_file = if cache_exists {
        OpenOptions::new().read(true).open(&path).unwrap()
    } else {
        File::create_new(&path).unwrap()
    };

    let mut json_str = String::default();
    cache_file.read_to_string(&mut json_str).unwrap();

    let mut cache_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&path)
        .unwrap();

    let build_file = IncBuildFile {
        file: file.to_str().unwrap().to_owned(),
        hash,
        dependencies: deps,
    };

    let mut json = serde_json::from_str::<Vec<IncBuildFile>>(&json_str).unwrap_or(vec![]);

    if let Some(found_file) = json
        .iter_mut()
        .find(|found_file| found_file.file == build_file.file)
    {
        found_file.hash = build_file.hash;
        found_file.dependencies = build_file.dependencies;
    } else {
        json.push(build_file);
    }

    cache_file
        .write_all(serde_json::to_string_pretty(&json).unwrap().as_bytes())
        .unwrap();
}
