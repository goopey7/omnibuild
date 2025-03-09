use mlua::{AsChunk, Chunk, Table};

pub struct LuaApi {
    lua: mlua::Lua,
}

impl LuaApi {
    pub fn load<'a>(&self, chunk: impl AsChunk<'a>) -> Chunk<'a> {
        self.lua.load(chunk)
    }

    pub fn globals(&self) -> Table
    {
        self.lua.globals()
    }
}
