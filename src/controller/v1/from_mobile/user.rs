use actix_web::{ web, get, post, put, delete, App, HttpResponse, HttpRequest, HttpServer , Responder};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct DataEntry {
    id: Option<u32>,
    text: String,
}

// ユーザ取得API
#[get("/api/v1/mobile/users/myself")]
pub async fn get_v1_user_from_mobile() -> impl Responder {
    let response_body = "get_v1_user_from_mobile";
    return HttpResponse::Ok().json(
        DataEntry {
            id: Some(999),
            text: String::from(response_body),
        }
    );
}

// ユーザ登録API
#[post("/api/v1/mobile/users")]
pub async fn register_v1_user_from_mobile() -> impl Responder {
    let response_body = "register_v1_user_from_mobile";
    return HttpResponse::Ok().json(
        DataEntry {
            id: Some(999),
            text: String::from(response_body),
        }
    );
}