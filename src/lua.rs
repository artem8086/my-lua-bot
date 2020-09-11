use rlua::Lua;

pub fn exec(lua_code: &str) -> Result<String, String> {
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

    if error {
        Err(result)
    } else {
        Ok(result)
    }
}