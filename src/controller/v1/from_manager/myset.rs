use actix_web::{web, get, HttpResponse, Responder};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct DataEntry {
    id: Option<u32>,
    text: String,
}

// マイセット取得API
#[get("/api/v1/manager/mysets/{mysets_id}")]
pub async fn get_one(web::Path(mysets_id): web::Path<u32>) -> impl Responder {
    let mysets_id: Option<u32> = Some(mysets_id);
    let response_body = "get_v1_myset_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: mysets_id,
            text: String::from(response_body),
        }
    );
}

// マイセット一覧取得API
#[get("/api/v1/manager/mysets")]
pub async fn get_list() -> impl Responder {
    let response_body = "get_v1_mysets_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: Some(999),
            text: String::from(response_body),
        }
    );
}