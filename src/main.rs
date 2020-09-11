mod routes;
mod lua;
mod config;

use actix_web::{web, App, HttpServer};

use std::env;

use self::config::AppConfig;

pub struct AppData {
    config: AppConfig
}

impl AppData {
    fn new(config: AppConfig) -> AppData {
        AppData { config }
    }
}

fn get_port(config: &AppConfig) -> String {
    env::vars_os()
        .find(|(name, _)| name.eq("PORT"))
        .map(|(_, value)| String::from(value.to_str().unwrap_or(&config.port)))
        .unwrap_or(String::from(&config.port))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::load()?;
    let port = get_port(&config);

    println!("Server run at port: {}", port);

    let app_data = web::Data::new(AppData::new(config));

    HttpServer::new(move || {
        App::new()
            .data(app_data.clone())
            .service(routes::service())
    })
        .bind(format!("0.0.0.0:{}", port))?
        .run()
        .await
}