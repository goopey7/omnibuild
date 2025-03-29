#[derive(Clone, Debug)]
pub enum CppStandard {
    Cpp11,
    Cpp14,
    Cpp17,
    Cpp20,
    Cpp23,
}

impl Default for CppStandard {
    fn default() -> Self {
        CppStandard::Cpp20
    }
}

impl mlua::FromLua for CppStandard {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        let error = Err(mlua::Error::FromLuaConversionError {
            from: "u32",
            to: "CppStandard".to_owned(),
            message: None,
        });

        match value.as_integer() {
            Some(standard) => match standard {
                11 => Ok(CppStandard::Cpp11),
                14 => Ok(CppStandard::Cpp14),
                17 => Ok(CppStandard::Cpp17),
                20 => Ok(CppStandard::Cpp20),
                23 => Ok(CppStandard::Cpp23),
                _ => error,
            },
            None => error,
        }
    }
}

impl Into<u8> for CppStandard
{
    fn into(self) -> u8 {
        match self
        {
            Self::Cpp11 => 11,
            Self::Cpp14 => 14,
            Self::Cpp17 => 17,
            Self::Cpp20 => 20,
            Self::Cpp23 => 23,
        }
    }
}

#[derive(Clone, Debug)]
pub enum CppWarning {
    All,
    Extra,
    Pedantic,
    Conversion,
    Shadow,
    OldStyleCast,
    FloatEqual,
    ExtraSemi,
    NonVirtualDtor,
    OverloadedVirtual,
    StrictNullSentinel,
    ZeroAsNullPointerConstant,
}

impl mlua::FromLua for CppWarning {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        let error = Err(mlua::Error::FromLuaConversionError {
            from: "string",
            to: "CppWarning".to_owned(),
            message: None,
        });
        match value.as_str() {
            Some(str) => match &str as &str {
                "All" => Ok(CppWarning::All),
                "Extra" => Ok(CppWarning::Extra),
                "Pedantic" => Ok(CppWarning::Pedantic),
                "Conversion" => Ok(CppWarning::Conversion),
                "Shadow" => Ok(CppWarning::Shadow),
                "OldStyleCast" => Ok(CppWarning::OldStyleCast),
                "FloatEqual" => Ok(CppWarning::FloatEqual),
                "ExtraSemi" => Ok(CppWarning::ExtraSemi),
                "NonVirtualDtor" => Ok(CppWarning::NonVirtualDtor),
                "OverloadedVirtual" => Ok(CppWarning::OverloadedVirtual),
                "StrictNullSentinel" => Ok(CppWarning::StrictNullSentinel),
                "ZeroAsNullPointerConstant" => Ok(CppWarning::ZeroAsNullPointerConstant),
                _ => error,
            },
            None => error,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Optimization {
    None,
    Size,
    Speed,
    MaxSpeed,
    MaxSize,
}

impl mlua::FromLua for Optimization {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        let error = Err(mlua::Error::FromLuaConversionError {
            from: "string",
            to: "Optimization".to_owned(),
            message: None,
        });

        match value.as_str() {
            Some(str) => match &str as &str {
                "None" => Ok(Optimization::None),
                "Size" => Ok(Optimization::Size),
                "Speed" => Ok(Optimization::Speed),
                "MaxSpeed" => Ok(Optimization::MaxSpeed),
                "MaxSize" => Ok(Optimization::MaxSize),
                &_ => error,
            },
            None => error,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BuildConfig {
    pub name: String,
    pub debug_info: bool,
    pub cpp_standard: CppStandard,
    pub warnings_as_errors: bool,
    pub warnings: Vec<CppWarning>,
    pub optimization: Optimization,
}

impl mlua::FromLua for BuildConfig {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> Result<Self, mlua::Error> {
        match value.as_table() {
            Some(value) => Ok(BuildConfig {
                name: value.get("name")?,
                cpp_standard: value.get("cpp_standard")?,
                warnings_as_errors: value.get("warnings_as_errors")?,
                debug_info: value.get("debug_info")?,
                warnings: value.get("warnings")?,
                optimization: value.get("optimization")?,
            }),
            None => Err(mlua::Error::FromLuaConversionError {
                from: "table",
                to: "TargetConfigurationConfig".to_string(),
                message: None,
            }),
        }
    }
}
