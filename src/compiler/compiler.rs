use std::{path::PathBuf, process::Command};

use crate::
    lua::config::{
        build_config::{BuildConfig, CppWarning, Optimization},
        module_config::ModuleConfig,
        target_config::TargetConfig,
    }
;

use super::clangd::CompileCommand;

pub trait Compiler {
    fn compile(
        module: &ModuleConfig,
        target_config: &TargetConfig,
        build_config: &BuildConfig,
        file: &PathBuf,
    ) -> CompileCommand;
    fn link_module(module: &ModuleConfig, target_config: &TargetConfig, object_files: Vec<PathBuf>);
    fn get_debug_symbols() -> &'static str;
    fn get_standard_prefix() -> &'static str;
    fn get_warning(warning: &CppWarning) -> &'static str;
    fn get_warnings_as_errors() -> &'static str;
    fn get_include_dir_prefix() -> &'static str;
    fn get_optimization(optimization: &Optimization) -> &'static str;
    fn get_definition_prefix() -> &'static str;
    fn input_src_file_prefix() -> &'static str;
    fn output_obj_file_prefix() -> &'static str;
}

pub fn stringify_command(cmd: &Command) -> String {
    let program = cmd.get_program().to_string_lossy();
    let args = cmd.get_args().map(|arg| {
        let arg_str = arg.to_string_lossy();
        if arg_str.contains(' ') || arg_str.contains('"') {
            format!("\"{}\"", arg_str.replace('"', "\\\""))
        } else {
            arg_str.into_owned()
        }
    });

    std::iter::once(program.into_owned())
        .chain(args)
        .collect::<Vec<String>>()
        .join(" ")
}
