use mlua::Lua;
use std::path::PathBuf;

use crate::lua_configuration::module_config::{ModuleConfig, ModuleType};

pub fn load_module(module_path: &PathBuf, api: &Lua) -> Result<ModuleConfig, mlua::Error> {
    let module_file_read = std::fs::read_to_string(&module_path.join("module.lua"))
        .expect(format!("{:?} is missing a module.lua file!", &module_path).as_str());
    api.load(module_file_read).exec()?;
    let globals = api.globals();

    let name = globals.get::<String>("name")?;
    let r#type = globals.get::<ModuleType>("type")?;
    let dependencies = globals.get::<Vec<String>>("dependencies")?;
    let include_dirs = globals.get::<Vec<String>>("include_dirs")?;
    Ok(ModuleConfig {
        name,
        r#type,
        dependencies,
        include_dirs,
    })
}
