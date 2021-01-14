use actix_web::{ web, get, post, put, delete, App, HttpResponse, HttpRequest, HttpServer , Responder};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct DataEntry {
    id: Option<u32>,
    text: String,
}

// 装備取得API
#[get("/api/v1/game/equipments/{equipment_id}")]
pub async fn get_v1_equipment_from_game(web::Path(equipment_id): web::Path<u32>) -> impl Responder {
    let equipment_id: Option<u32> = Some(equipment_id);
    let response_body = "get_v1_equipment_from_game";
    return HttpResponse::Ok().json(
        DataEntry {
            id: equipment_id,
            text: String::from(response_body),
        }
    );
}

// 装備一覧取得API
#[get("/api/v1/game/equipments")]
pub async fn get_v1_equipments_from_game() -> impl Responder {
    let response_body = "get_v1_equipments_from_game";
    return HttpResponse::Ok().json(
        DataEntry {
            id: Some(999),
            text: String::from(response_body),
        }
    );
}