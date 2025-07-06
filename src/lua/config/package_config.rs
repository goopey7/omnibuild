use std::path::PathBuf;

use super::module_config::ModuleType;

#[derive(Debug, Clone)]
pub struct PackageConfig {
    pub name: String,
    pub r#type: Option<ModuleType>,
    pub include_dirs: Vec<String>,
    pub binary: Option<PathBuf>,
}

impl mlua::FromLua for PackageConfig {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> Result<Self, mlua::Error> {
        match value.as_table() {
            Some(value) => Ok(PackageConfig {
                name: value.get("name")?,
                r#type: value.get("type").unwrap_or(None),
                include_dirs: value.get("include_dirs").unwrap_or(Vec::new()),
                binary: value.get("binary").unwrap_or(None),
            }),
            None => Err(mlua::Error::FromLuaConversionError {
                from: "table",
                to: "PackageConfig".to_string(),
                message: None,
            }),
        }
    }
}
