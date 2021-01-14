use actix_web::{web, get, HttpResponse, Responder};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct DataEntry {
    id: Option<u32>,
    text: String,
}

// パラメータチップ取得API
#[get("/api/v1/game/parameter-chips/{parameter_chip_id}")]
pub async fn get_one(web::Path(parameter_chip_id): web::Path<u32>) -> impl Responder {
    let parameter_chip_id: Option<u32> = Some(parameter_chip_id);
    let response_body = "get_v1_parameter_chip_from_game";
    return HttpResponse::Ok().json(
        DataEntry {
            id: parameter_chip_id,
            text: String::from(response_body),
        }
    );
}

// パラメータチップ一覧取得API
#[get("/api/v1/game/parameter-chips")]
pub async fn get_list() -> impl Responder {
    let response_body = "get_v1_parameter_chips_from_game";
    return HttpResponse::Ok().json(
        DataEntry {
            id: Some(999),
            text: String::from(response_body),
        }
    );
}
