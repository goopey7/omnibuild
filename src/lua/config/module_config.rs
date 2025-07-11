use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleType {
    Dylib,
    Lib,
    Exe,
}

impl mlua::FromLua for ModuleType {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::String(s) => match &*(s.to_str()?) {
                "dylib" => Ok(ModuleType::Dylib),
                "lib" => Ok(ModuleType::Lib),
                "exe" => Ok(ModuleType::Exe),
                other => Err(mlua::Error::FromLuaConversionError {
                    from: "string",
                    to: "ModuleType".to_string(),
                    message: Some(format!("Invalid module_type: {}", other)),
                }),
            },
            _ => Err(mlua::Error::FromLuaConversionError {
                from: "non-string",
                to: "ModuleType".to_string(),
                message: Some("Expected a string".to_string()),
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ModuleConfig {
    pub name: String,
    pub r#type: Option<ModuleType>,
    pub dependencies: Vec<String>,
    pub include_dirs: Vec<String>,
    pub ignore_dirs: Vec<String>,
    pub path: Option<PathBuf>,
}

impl mlua::FromLua for ModuleConfig {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> Result<Self, mlua::Error> {
        match value.as_table() {
            Some(value) => Ok(ModuleConfig {
                name: value.get("name")?,
                r#type: value.get("type").unwrap_or(None),
                dependencies: value.get("dependencies").unwrap_or(Vec::new()),
                include_dirs: value.get("include_dirs").unwrap_or(Vec::new()),
                ignore_dirs: value.get("ignore_dirs").unwrap_or(Vec::new()),
                path: value.get("path").unwrap_or(None),
            }),
            None => Err(mlua::Error::FromLuaConversionError {
                from: "table",
                to: "ModuleConfig".to_string(),
                message: None,
            }),
        }
    }
}
