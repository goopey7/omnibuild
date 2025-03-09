pub struct TargetConfigurationConfig {
    name: String,
    optimization_level: u8,
    warnings_as_errors: bool,
}

impl mlua::FromLua for TargetConfigurationConfig {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> Result<Self, mlua::Error> {
        match value.as_table() {
            Some(value) => Ok(TargetConfigurationConfig {
                name: value.get("name")?,
                optimization_level: value.get("optimization_level")?,
                warnings_as_errors: value.get("warnings_as_errors")?,
            }),
            None => Err(mlua::Error::FromLuaConversionError {
                from: "table",
                to: "TargetConfigurationConfig".to_string(),
                message: None,
            }),
        }
    }
}
