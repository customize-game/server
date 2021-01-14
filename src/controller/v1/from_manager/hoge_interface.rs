use actix_web::{ web, get, post, put, delete, App, HttpResponse, HttpRequest, HttpServer , Responder};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct DataEntry {
    id: Option<u32>,
    text: String,
}

// hogeインタフェース取得API
#[get("/api/v1/manager/hoge-interfaces/{hoge_interface_id}")]
pub async fn get_v1_hoge_interface_from_manager(web::Path(hoge_interface_id): web::Path<u32>) -> impl Responder {
    let hoge_interface_id: Option<u32> = Some(hoge_interface_id);
    let response_body = "get_v1_hoge_interface_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: hoge_interface_id,
            text: String::from(response_body),
        }
    );
}

// hogeインタフェース一覧取得API
#[get("/api/v1/manager/hoge-interfaces")]
pub async fn get_v1_hoge_interfaces_from_manager() -> impl Responder {
    let response_body = "get_v1_hoge_interfaces_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: Some(999),
            text: String::from(response_body),
        }
    );
}

// hogeインタフェース登録API
#[post("/api/v1/manager/hoge-interfaces")]
pub async fn register_v1_hoge_interface_from_manager() -> impl Responder {
    let response_body = "register_v1_hoge_interface_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: Some(999),
            text: String::from(response_body),
        }
    );
}

// hogeインタフェース更新API
#[put("/api/v1/manager/hoge-interfaces/{hoge_interface_id}")]
pub async fn update_v1_hoge_interface_from_manager(web::Path(hoge_interface_id): web::Path<u32>) -> impl Responder {
    let hoge_interface_id: Option<u32> = Some(hoge_interface_id);
    let response_body = "update_v1_hoge_interface_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: hoge_interface_id,
            text: String::from(response_body),
        }
    );
}

// hogeインタフェース削除API
#[delete("/api/v1/manager/hoge-interfaces/{hoge_interface_id}")]
pub async fn delete_v1_hoge_interface_from_manager(web::Path(hoge_interface_id): web::Path<u32>) -> impl Responder {
    let hoge_interface_id: Option<u32> = Some(hoge_interface_id);
    let response_body = "delete_v1_hoge_interface_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: hoge_interface_id,
            text: String::from(response_body),
        }
    );
}