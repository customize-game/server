use actix_web::{ web, get, post, put, delete, App, HttpResponse, HttpRequest, HttpServer , Responder};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct DataEntry {
    id: Option<u32>,
    text: String,
}

// マイセット取得API
#[get("/api/v1/game/mysets/{mysets_id}")]
pub async fn get_v1_myset_from_game(web::Path(mysets_id): web::Path<u32>) -> impl Responder {
    let mysets_id: Option<u32> = Some(mysets_id);
    let response_body = "get_v1_myset_from_game";
    return HttpResponse::Ok().json(
        DataEntry {
            id: mysets_id,
            text: String::from(response_body),
        }
    );
}

// マイセット一覧取得API
#[get("/api/v1/game/mysets")]
pub async fn get_v1_mysets_from_game() -> impl Responder {
    let response_body = "get_v1_mysets_from_game";
    return HttpResponse::Ok().json(
        DataEntry {
            id: Some(999),
            text: String::from(response_body),
        }
    );
}

// マイセット登録API
#[post("/api/v1/game/mysets")]
pub async fn register_v1_myset_from_game() -> impl Responder {
    let response_body = "register_v1_myset_from_game";
    return HttpResponse::Ok().json(
        DataEntry {
            id: Some(999),
            text: String::from(response_body),
        }
    );
}

// マイセット更新API
#[put("/api/v1/game/mysets/{mysets_id}")]
pub async fn update_v1_myset_from_game(web::Path(mysets_id): web::Path<u32>) -> impl Responder {
    let mysets_id: Option<u32> = Some(mysets_id);
    let response_body = "update_v1_myset_from_game";
    return HttpResponse::Ok().json(
        DataEntry {
            id: mysets_id,
            text: String::from(response_body),
        }
    );
}

// マイセット削除API
#[delete("/api/v1/game/mysets/{mysets_id}")]
pub async fn delete_v1_myset_from_game(web::Path(mysets_id): web::Path<u32>) -> impl Responder {
    let mysets_id: Option<u32> = Some(mysets_id);
    let response_body = "delete_v1_myset_from_game";
    return HttpResponse::Ok().json(
        DataEntry {
            id: mysets_id,
            text: String::from(response_body),
        }
    );
}