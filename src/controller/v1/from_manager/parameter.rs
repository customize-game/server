use actix_web::{web, get, post, put, delete, HttpResponse, Responder};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct DataEntry {
    id: Option<u32>,
    text: String,
}

// パラメータ取得API
#[get("/api/v1/manager/parameters/{parameter_id}")]
pub async fn get_one(web::Path(parameter_id): web::Path<u32>) -> impl Responder {
    let parameter_id: Option<u32> = Some(parameter_id);
    let response_body = "get_v1_parameter_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: parameter_id,
            text: String::from(response_body),
        }
    );
}

// パラメータ一覧取得API
#[get("/api/v1/manager/parameters")]
pub async fn get_list() -> impl Responder {
    let response_body = "get_v1_parameters_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: Some(999),
            text: String::from(response_body),
        }
    );
}

// パラメータ登録API
#[post("/api/v1/manager/parameters")]
pub async fn register() -> impl Responder {
    let response_body = "register_v1_parameter_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: Some(999),
            text: String::from(response_body),
        }
    );
}

// パラメータ更新API
#[put("/api/v1/manager/parameters/{parameter_id}")]
pub async fn update(web::Path(parameter_id): web::Path<u32>) -> impl Responder {
    let parameter_id: Option<u32> = Some(parameter_id);
    let response_body = "update_v1_parameter_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: parameter_id,
            text: String::from(response_body),
        }
    );
}

// パラメータ削除API
#[delete("/api/v1/manager/parameters/{parameter_id}")]
pub async fn delete(web::Path(parameter_id): web::Path<u32>) -> impl Responder {
    let parameter_id: Option<u32> = Some(parameter_id);
    let response_body = "delete_v1_parameter_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: parameter_id,
            text: String::from(response_body),
        }
    );
}