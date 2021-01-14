use actix_web::{web, get, put, delete, HttpResponse, Responder};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct DataEntry {
    id: Option<u32>,
    text: String,
}

// ユーザ取得API
#[get("/api/v1/manager/users/{user_id}")]
pub async fn get_one(web::Path(user_id): web::Path<u32>) -> impl Responder {
    let user_id: Option<u32> = Some(user_id);
    let response_body = "get_v1_user_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: user_id,
            text: String::from(response_body),
        }
    );
}

// ユーザ一覧取得API
#[get("/api/v1/manager/users")]
pub async fn get_list() -> impl Responder {
    let response_body = "get_v1_users_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: Some(999),
            text: String::from(response_body),
        }
    );
}

// ユーザ更新API
#[put("/api/v1/manager/users/{user_id}")]
pub async fn update(web::Path(user_id): web::Path<u32>) -> impl Responder {
    let user_id: Option<u32> = Some(user_id);
    let response_body = "update_v1_user_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: user_id,
            text: String::from(response_body),
        }
    );
}

// ユーザ削除API
#[delete("/api/v1/manager/users/{user_id}")]
pub async fn delete(web::Path(user_id): web::Path<u32>) -> impl Responder {
    let user_id: Option<u32> = Some(user_id);
    let response_body = "delete_v1_user_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: user_id,
            text: String::from(response_body),
        }
    );
}