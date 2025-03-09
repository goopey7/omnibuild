pub struct ProjectConfig {
    pub project_name: String,
    pub project_version: String,
}

impl mlua::FromLua for ProjectConfig {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> Result<Self, mlua::Error> {
        match value.as_table() {
            Some(value) => Ok(ProjectConfig {
                project_name: value.get("project_name")?,
                project_version: value.get("project_version")?,
            }),
            None => Err(mlua::Error::FromLuaConversionError {
                from: "table",
                to: "ProjectConfig".to_string(),
                message: None,
            }),
        }
    }
}
