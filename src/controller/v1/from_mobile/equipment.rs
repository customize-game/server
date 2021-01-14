use actix_web::{web, get, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use server::service;

#[derive(Serialize, Deserialize, Debug)]
struct DataEntry {
    id: Option<u32>,
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct GetOneResponseDataEntry {
    id: u32,
    name: String,
    ruby: String,
    flavor: String,
}
// 装備取得API
#[get("/api/v1/mobile/equipments/{equipment_id}")]
pub async fn get_one(web::Path(equipment_id): web::Path<u32>) -> impl Responder {
    let equipment_id: Option<u32> = Some(equipment_id);
    let equipment = service::equipment::find_by_id(equipment_id.unwrap());
    return HttpResponse::Ok().json(
        GetOneResponseDataEntry {
            id: equipment.id,
            name: equipment.name,
            ruby: equipment.ruby,
            flavor: equipment.flavor,
        }
    );
}

// 装備一覧取得API
#[get("/api/v1/mobile/equipments")]
pub async fn get_list() -> impl Responder {
    let response_body = "get_v1_equipments_from_mobile";
    return HttpResponse::Ok().json(
        DataEntry {
            id: Some(999),
            text: String::from(response_body),
        }
    );
}
