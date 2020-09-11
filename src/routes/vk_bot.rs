use actix_web::{web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use actix_web::dev::HttpServiceFactory;
use actix_web::web::Data;

#[derive(Deserialize, Clone)]
pub struct VkConfig {
    #[serde(rename = "confirmKey")]
    confirm_key: String
}

#[derive(Deserialize)]
struct VkStruct {
    #[serde(rename = "type")]
    type_request: Option<String>,
    group_id: Option<String>,
    secret: Option<String>
}

async fn index(vk_obj: web::Json<VkStruct>, data: web::Data<crate::AppData>) -> impl Responder {
    HttpResponse::Ok()
        .header("Content-Type", "text/plain")
        .body(&data.config.vk.confirm_key)
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/vk")
            .route(web::post().to(index))
    );
}