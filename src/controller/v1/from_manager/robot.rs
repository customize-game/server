use actix_web::{web, get, post, put, delete, HttpResponse, Responder};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct DataEntry {
    id: Option<u32>,
    text: String,
}

// ロボット取得API
#[get("/api/v1/manager/robots/{body_id}")]
pub async fn get_one(web::Path(body_id): web::Path<u32>) -> impl Responder {
    let body_id: Option<u32> = Some(body_id);
    let response_body = "get_v1_robot_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: body_id,
            text: String::from(response_body),
        }
    );
}

// ロボット一覧取得API
#[get("/api/v1/manager/robots")]
pub async fn get_list() -> impl Responder {
    let response_body = "get_v1_robots_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: Some(999),
            text: String::from(response_body),
        }
    );
}

// ロボット登録API
#[post("/api/v1/manager/robots")]
pub async fn register() -> impl Responder {
    let response_body = "register_v1_robot_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: Some(999),
            text: String::from(response_body),
        }
    );
}

// ロボット更新API
#[put("/api/v1/manager/robots/{body_id}")]
pub async fn update(web::Path(body_id): web::Path<u32>) -> impl Responder {
    let body_id: Option<u32> = Some(body_id);
    let response_body = "update_v1_robot_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: body_id,
            text: String::from(response_body),
        }
    );
}

// ロボット削除API
#[delete("/api/v1/manager/robots/{body_id}")]
pub async fn delete(web::Path(body_id): web::Path<u32>) -> impl Responder {
    let body_id: Option<u32> = Some(body_id);
    let response_body = "delete_v1_robot_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: body_id,
            text: String::from(response_body),
        }
    );
}