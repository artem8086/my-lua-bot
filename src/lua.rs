use rlua::Lua;
use serde::Serialize;

#[derive(Serialize)]
pub struct LuaResult  {
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>
}

impl LuaResult {
    pub fn have_error(&self) -> bool {
        self.result.is_none()
    }
}

pub fn exec(lua_code: &str) -> LuaResult {
    let lua = Lua::new();
    let mut result = String::new();
    let mut error = false;
    lua.context(|ctx| {
        let res = ctx.load(&format!("tostring({})", lua_code))
            .set_name("lua-bot")
            .and_then(|chunk| chunk.eval::<String>());
        match res {
            Ok(res) => { result.push_str(&res); }
            Err(err) => {
                error = true;
                result.push_str(&err.to_string());
            }
        }
    });

    if !error {
        LuaResult { result: Some(result), error: None }
    } else {
        LuaResult { result: None, error: Some(result) }
    }
}