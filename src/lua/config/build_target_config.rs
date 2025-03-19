use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct BuildTargetConfig {
    pub name: String,
    pub module_directories: Vec<std::path::PathBuf>,
    pub definitions: HashMap<String, String>,
}

impl mlua::FromLua for BuildTargetConfig
{
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> Result<Self, mlua::Error> {
        match value.as_table() {
            Some(value) => {
                Ok(BuildTargetConfig {
                name: value.get("name")?,
                module_directories: value.get("module_directories")?,
                definitions: value.get("definitions").unwrap_or(HashMap::default()),
            })},
            None => Err(mlua::Error::FromLuaConversionError {
                from: "table",
                to: "BuildTargetConfig".to_string(),
                message: None,
            }),
        }
    }
}
