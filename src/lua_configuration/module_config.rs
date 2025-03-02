enum ModuleType {
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

pub struct ModuleConfig {
    module_name: Option<String>,
    module_type: ModuleType,
    dependencies: Vec<String>,
}
