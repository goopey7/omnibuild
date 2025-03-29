use std::path::PathBuf;

use crate::lua::config::{build_config::{BuildConfig, CppWarning, Optimization}, module_config::ModuleConfig, target_config::TargetConfig};

pub trait Compiler {
    fn compile(module: &ModuleConfig, target_config: &TargetConfig, build_config: &BuildConfig, file: &PathBuf);
    fn get_debug_symbols() -> &'static str;
    fn get_standard_prefix() -> &'static str;
    fn get_warning(warning: &CppWarning) -> &'static str;
    fn get_warnings_as_errors() -> &'static str;
    fn get_staticlib_flags() -> &'static str;
    fn get_dylib_flags() -> &'static str;
    fn get_exe_flags() -> &'static str;
    fn get_include_dir_prefix() -> &'static str;
    fn get_lib_prefix() -> &'static str;
    fn get_optimization(optimization: &Optimization) -> &'static str;
    fn get_definition_prefix() -> &'static str;
    fn input_src_file_prefix() -> &'static str;
    fn output_obj_file_prefix() -> &'static str;
}
