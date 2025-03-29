use std::{
    path::PathBuf,
    process::Command,
};

use super::compiler::Compiler;
use crate::{
    build::build_state::BUILD_STATE,
    lua::config::{
        build_config::{BuildConfig, CppWarning, Optimization},
        module_config::ModuleConfig,
        target_config::TargetConfig,
    },
};

pub struct Gcc {}

impl Compiler for Gcc {
    fn compile(
        module: &ModuleConfig,
        target_config: &TargetConfig,
        build_config: &BuildConfig,
        file: &PathBuf,
    ) {
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
            let dep_path = module_dep.path.clone().unwrap();
            let dep_path = dep_path.to_str().unwrap();
            module_dep.include_dirs.iter().for_each(|dir| {
                cmd.arg(format!(
                    "{}{}/{}",
                    Gcc::get_include_dir_prefix(),
                    dep_path,
                    dir
                ));
            });
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

        cmd.arg(Gcc::input_src_file_prefix());
        cmd.arg(format!(
            "{}",
            file.display()
        ));

        let mut o_file = file.clone();
        o_file.set_extension("o");

        let out_path = format!("{}/{}", target_config.output_dir.display(), o_file.parent().unwrap().display());
        Command::new("mkdir")
            .arg("-p")
            .arg(out_path)
            .status()
            .expect("mkdir failed!");

        cmd.arg(Gcc::output_obj_file_prefix());
        cmd.arg(format!(
            "{}/{}",
            target_config.output_dir.display(),
            o_file.display()
        ));

        cmd.status().unwrap();
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

    fn get_exe_flags() -> &'static str {
        ""
    }

    fn get_dylib_flags() -> &'static str {
        "-shared -fPIC"
    }

    fn get_staticlib_flags() -> &'static str {
        ""
    }

    fn get_standard_prefix() -> &'static str {
        "-std=c++"
    }

    fn get_include_dir_prefix() -> &'static str {
        "-I"
    }

    fn get_lib_prefix() -> &'static str {
        ""
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
