use rlua::Lua;
use serde::Serialize;

#[derive(Serialize)]
pub struct LuaResult {
    result: String
}

pub fn exec(lua_code: &str) -> LuaResult {
    let lua = Lua::new();
    let mut result = String::new();
    lua.context(|ctx| {
        let res = ctx.load(lua_code).eval::<String>();
        if let Result::Ok(res) = res {
            result.push_str(&res);
        }
    });

    LuaResult { result }
}