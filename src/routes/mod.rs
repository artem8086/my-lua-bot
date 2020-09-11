pub mod vk_bot;

use actix_web::{web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use actix_web::dev::HttpServiceFactory;

#[derive(Deserialize)]
struct LuaCode {
    code: String
}

#[derive(Serialize)]
struct LuaResult  {
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

impl From<Result<String, String>> for LuaResult {
    fn from(lua_result: Result<String, String>) -> Self {
        match lua_result {
        Ok(result) => LuaResult { result: Some(result), error: None },
            Err(error) => LuaResult { result: None, error: Some(error) }
        }
    }
}

async fn index(lua: web::Json<LuaCode>) -> impl Responder {
    let result = LuaResult::from(crate::lua::exec(&lua.code));
    if result.have_error() {
        HttpResponse::BadRequest().json(result)
    } else {
        HttpResponse::Ok().json(result)
    }
}

pub fn service() -> impl HttpServiceFactory {
    web::scope("/")
        .route("", web::get().to(index))
        .service(vk_bot::service())
}