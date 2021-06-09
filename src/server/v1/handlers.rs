use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

// アクセサの型をinfrastrucresで定義されたものをimport
use crate::domains::users::UserId;
use crate::infrastructures::infrastructures::DbServer;
use crate::server::v1::responses::*;
use crate::usecases;

// ユーザ取得API
#[get("/v1/users/{id}")]
pub async fn get_myself(db: web::Data<DbServer>, path_params: web::Path<u32>) -> impl Responder {
    let user_id = UserId::new(path_params.into_inner().into());
    match usecases::usecases::find_by_id(db.user_repository(), user_id) {
        Ok(user) => HttpResponse::Ok().json(UserDTO::new(&user)),
        Err(_) => HttpResponse::InternalServerError().json(""),
    }
}

// ユーザ登録API
#[post("/api/v1/mobile/users")]
pub async fn register() -> impl Responder {
     format!("hello from /api/v1/mobile/users")
}
