#[derive(Debug)]
pub struct BuildTargetConfig {
    pub name: String,
    pub module_directories: Vec<std::path::PathBuf>,
}

impl mlua::FromLua for BuildTargetConfig
{
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> Result<Self, mlua::Error> {
        match value.as_table() {
            Some(value) => Ok(BuildTargetConfig {
                name: value.get("name")?,
                module_directories: value.get("module_directories")?,
            }),
            None => Err(mlua::Error::FromLuaConversionError {
                from: "table",
                to: "BuildTargetConfig".to_string(),
                message: None,
            }),
        }
    }
}
