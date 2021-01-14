use actix_web::{get, post, HttpResponse, Responder};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct DataEntry {
    id: Option<u32>,
    text: String,
}

// ユーザ取得API
#[get("/api/v1/game/users/myself")]
pub async fn get_myself() -> impl Responder {
    let response_body = "get_v1_user_from_game";
    return HttpResponse::Ok().json(
        DataEntry {
            id: Some(999),
            text: String::from(response_body),
        }
    );
}

// ユーザ登録API
#[post("/api/v1/game/users")]
pub async fn register() -> impl Responder {
    let response_body = "register_v1_user_from_game";
    return HttpResponse::Ok().json(
        DataEntry {
            id: Some(999),
            text: String::from(response_body),
        }
    );
}