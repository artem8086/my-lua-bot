use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

use std::env;

mod lua;

#[derive(Deserialize)]
struct LuaCode {
    code: String
}

async fn index(lua: web::Json<LuaCode>) -> impl Responder {
    HttpResponse::Ok().json(lua::exec(&lua.code))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::vars()
        .find(|(name, _)| name.eq("PORT"))
        .unwrap_or((String::from("PORT"), String::from("8088"))).1;

    println!("Server run at port: {}", port);

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
        .bind(format!("127.0.0.1:{}", port))?
        .run()
        .await
}