use actix_web::{web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use actix_web::dev::HttpServiceFactory;

#[derive(Deserialize, Clone)]
pub struct VkConfig {
    #[serde(rename = "confirmKey")]
    confirm_key: String
}

#[derive(Deserialize)]
struct VkStruct {
    #[serde(rename = "type")]
    type_request: String,
    group_id: String,
    secret: String
}

async fn index(vk_obj: web::Json<VkStruct>, data: web::Data<crate::AppData>) -> impl Responder {
    HttpResponse::Ok().body(&data.config.vk.confirm_key)
}

pub fn service() -> impl HttpServiceFactory {
    web::scope("bot/vk")
        .route("", web::post().to(index))
}