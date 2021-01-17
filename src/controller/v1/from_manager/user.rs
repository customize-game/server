use actix_web::{delete, get, put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use server::service;

// ユーザ取得APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct GetOneResponseEntry {
    id: u32,          // ユーザID
    exp: u32,         // 経験値
    is_deleted: bool, // 削除済みかどうか
    version: u32,     // バージョン
}
// ユーザ取得API
#[get("/api/v1/manager/users/{user_id}")]
pub async fn get_one(
    web::Path(user_id): web::Path<u32>, // ユーザID - パスパラメータ
) -> impl Responder {
    // リクエスト取得
    let user_id: Option<u32> = Some(user_id);

    // データ取得
    let user = service::user::find_by_id(user_id.unwrap());

    // レスポンス加工
    return HttpResponse::Ok().json(GetOneResponseEntry {
        id: user.id,
        exp: user.exp,
        is_deleted: user.is_deleted,
        version: user.version,
    });
}

// ユーザ一覧取得APIクエリパラメータ
#[derive(Deserialize)]
pub struct GetListRequest {
    sort_by: Option<u32>, // ソート種別
    limit: Option<u32>,   // 取得数
    offset: Option<u32>,  // 取得位置
}
// ユーザ一覧取得APIレスポンスのユーザ
#[derive(Serialize, Deserialize, Debug)]
struct UserEntryOfGetListResponseEntry {
    id: u32,          // ユーザID
    exp: u32,         // 経験値
    is_deleted: bool, // 削除済みかどうか
}
// ユーザ一覧取得APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct GetListResponseEntry {
    total_count: u32,                            // 合計数
    users: Vec<UserEntryOfGetListResponseEntry>, // ユーザ一覧
}
// ユーザ一覧取得API
#[get("/api/v1/manager/users")]
pub async fn get_list(
    query: web::Query<GetListRequest>, // クエリパラメータ
) -> impl Responder {
    // リクエスト取得
    let sort_by = query.sort_by;
    let limit = query.limit;
    let offset = query.offset;

    // データ取得
    let users = service::user::find_list(sort_by, limit, offset);

    // レスポンス加工
    let mut response = GetListResponseEntry {
        total_count: users.total_count,
        users: Vec::new(),
    };
    for user in &users.users {
        response.users.push(UserEntryOfGetListResponseEntry {
            id: user.id,
            exp: user.exp,
            is_deleted: user.is_deleted,
        });
    }
    return HttpResponse::Ok().json(response);
}

// ユーザ更新APIリクエスト
#[derive(Deserialize)]
pub struct UpdateRequestBody {
    exp: u32,         // 経験値
    is_deleted: bool, // 削除済みかどうか
}
// 装備更新APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct UpdateResponseEntry {
    id: u32,          // ユーザID
    exp: u32,         // 経験値
    is_deleted: bool, // 削除済みかどうか
    version: u32,     // バージョン
}
// ユーザ更新API
#[put("/api/v1/manager/users/{user_id}")]
pub async fn update(
    web::Path(user_id): web::Path<u32>, // ユーザID - パスパラメータ
    request_body: web::Json<UpdateRequestBody>, // リクエストボディ
) -> impl Responder {
    // リクエスト取得
    let user_id: Option<u32> = Some(user_id);
    let exp = request_body.exp;
    let is_deleted = request_body.is_deleted;

    // データ更新
    let user = service::user::update(user_id.unwrap(), exp, is_deleted);

    // レスポンス加工
    return HttpResponse::Ok().json(UpdateResponseEntry {
        id: user.id,
        exp: user.exp,
        is_deleted: user.is_deleted,
        version: user.version,
    });
}

// ユーザ削除APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct DeleteResponseEntry {
    id: u32,          // ユーザID
    exp: u32,         // 経験値
    is_deleted: bool, // 削除済みかどうか
    version: u32,     // バージョン
}
// ユーザ削除API
#[delete("/api/v1/manager/users/{user_id}")]
pub async fn delete(
    web::Path(user_id): web::Path<u32>, // ユーザID - パスパラメータ
) -> impl Responder {
    // リクエスト取得
    let user_id: Option<u32> = Some(user_id);

    // データ削除
    let user = service::user::delete(user_id.unwrap());

    // レスポンス加工
    return HttpResponse::Ok().json(DeleteResponseEntry {
        id: user.id,
        exp: user.exp,
        is_deleted: user.is_deleted,
        version: user.version,
    });
}
