extern crate server;

use actix_web::{get, HttpResponse, Responder};
use serde::{Serialize, Deserialize};

use server::utils; 

#[derive(Serialize, Deserialize, Debug)]
struct DataEntry {
    id: Option<u32>,
    text: String,
}

// index
#[get("/")]
pub async fn index() -> impl Responder {
    let response_body = "Customize Game!";
    utils::establish_connection();
    return HttpResponse::Ok().json(
        DataEntry {
            id: Some(999),
            text: String::from(response_body),
        }
    );
}