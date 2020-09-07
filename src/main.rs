use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

use std::env;
use std::ffi::OsString;

mod lua;

#[derive(Deserialize)]
struct LuaCode {
    code: String
}

async fn index(lua: web::Json<LuaCode>) -> impl Responder {
    let result = lua::exec(&lua.code);
    if result.have_error() {
        HttpResponse::BadRequest().json(result)
    } else {
        HttpResponse::Ok().json(result)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::vars_os()
        .find(|(name, _)| name.eq("PORT"))
        .map(|(_, value)| value)
        .unwrap_or(OsString::from("8088"))
        .to_str().unwrap_or("8088");

    println!("Server run at port: {}", port);

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
        .bind(format!("127.0.0.1:{}", port))?
        .run()
        .await
}