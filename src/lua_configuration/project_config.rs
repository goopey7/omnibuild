pub struct ModuleDirectory {
    pub path: std::path::PathBuf,
    pub targets: Vec<String>,
}

impl mlua::FromLua for ModuleDirectory {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::Table(table) => {
                if !table.contains_key("path").unwrap_or(false) {
                    return Err(mlua::Error::FromLuaConversionError {
                        from: "table",
                        to: "ModuleDirectory".to_string(),
                        message: Some(
                            "Invalid module directory in module_directories. `path` field not set"
                                .to_string(),
                        ),
                    });
                }

                Ok(Self {
                    path: table.get("path").expect("there must be a path provided"),
                    targets: table.get("targets").unwrap_or(
                        lua.globals()
                            .get("build_targets")
                            .expect("project's build_targets not configured!"),
                    ),
                })
            }
            _ => Err(mlua::Error::FromLuaConversionError {
                from: "non-table",
                to: "ModuleDirectory".to_string(),
                message: Some("Expected a table".to_string()),
            }),
        }
    }
}

pub struct ProjectConfig {
    pub project_name: String,
    pub project_version: String,
    pub build_targets: Vec<String>,
    pub module_directories: Vec<ModuleDirectory>,
}
