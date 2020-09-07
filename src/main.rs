use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

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
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
        .bind("127.0.0.1:8088")?
        .run()
        .await
}