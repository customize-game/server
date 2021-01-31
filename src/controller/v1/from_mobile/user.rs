use actix_web::{get, post, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use server::service;

// ユーザ取得APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct GetOneResponseEntry {
    exp: u32,     // 経験値
    version: u32, // バージョン
}
// ユーザ取得API
#[get("/api/v1/mobile/users/myself")]
pub async fn get_myself() -> impl Responder {
    // リクエスト取得
    let user_id: Option<u32> = Some(4); // TODO 認証情報から取得

    // データ取得
    let user = service::user::find_by_id(user_id.unwrap());

    // レスポンス加工
    return HttpResponse::Ok().json(GetOneResponseEntry {
        exp: user.exp,
        version: user.version,
    });
}

// ユーザ登録APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct RegisterResponseEntry {
    exp: u32,     // 経験値
    version: u32, // バージョン
}
// ユーザ登録API
#[post("/api/v1/mobile/users")]
pub async fn register() -> impl Responder {
    // データ登録
    let user = service::user::register();

    // レスポンス加工
    return HttpResponse::Ok().json(RegisterResponseEntry {
        exp: user.exp,
        version: user.version,
    });
}
