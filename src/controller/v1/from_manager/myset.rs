use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use server::service;

// マイセット取得APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct GetOneResponseEntry {
    id: u32,                     // マイセットID
    name: String,                // マイセット名
    user_id: u32,                // ユーザID
    body_id: u32,                // 素体ID
    display_order: u32,          // 表示順
    version: u32,                // バージョン
    body_name: String,           // 素体名
    body_ruby: Option<String>,   // 素体ルビ
    body_flavor: Option<String>, // 素体フレーバー
    body_display_order: u32,     // 素体表示順
    body_is_deleted: bool,       // 素体削除済みかどうか
    body_version: u32,           // 素体バージョン
}
// マイセット取得API
#[get("/api/v1/manager/mysets/{myset_id}")]
pub async fn get_one(
    web::Path(myset_id): web::Path<u32>, // マイセットID - パスパラメータ
) -> impl Responder {
    // リクエスト取得
    let myset_id: Option<u32> = Some(myset_id);

    // データ取得
    let myset = service::myset::find_by_id(myset_id.unwrap());

    // レスポンス加工
    return HttpResponse::Ok().json(GetOneResponseEntry {
        id: myset.id,
        name: myset.name,
        user_id: myset.user_id.unwrap(),
        body_id: myset.body_id,
        display_order: myset.display_order,
        version: myset.version,
        body_name: myset.body_name,
        body_ruby: myset.body_ruby,
        body_flavor: myset.body_flavor,
        body_display_order: myset.body_display_order,
        body_is_deleted: myset.body_is_deleted,
        body_version: myset.body_version,
    });
}

// マイセット一覧取得APIクエリパラメータ
#[derive(Deserialize)]
pub struct GetListRequest {
    sort_by: Option<u32>, // ソート種別
    limit: Option<u32>,   // 取得数
    offset: Option<u32>,  // 取得位置
}
// マイセット一覧取得APIレスポンスのマイセット
#[derive(Serialize, Deserialize, Debug)]
struct MysetEntryOfGetListResponseEntry {
    id: u32,            // 装備ID
    name: String,       // 装備名
    user_id: u32,       // ユーザID
    body_id: u32,       // 素体ID
    display_order: u32, // 表示順
}
// マイセット一覧取得APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct GetListResponseEntry {
    total_count: u32,                              // 合計数
    mysets: Vec<MysetEntryOfGetListResponseEntry>, // マイセット一覧
}
// マイセット一覧取得API
#[get("/api/v1/manager/mysets")]
pub async fn get_list(
    query: web::Query<GetListRequest>, // クエリパラメータ
) -> impl Responder {
    // リクエスト取得
    let user_id = None;
    let sort_by = query.sort_by;
    let limit = query.limit;
    let offset = query.offset;

    // データ取得
    let mysets = service::myset::find_list(user_id, sort_by, limit, offset);

    // レスポンス加工
    return HttpResponse::Ok().json(GetListResponseEntry {
        total_count: mysets.total_count,
        mysets: mysets
            .mysets
            .iter()
            .map(|myset| MysetEntryOfGetListResponseEntry {
                id: myset.id,
                name: myset.name.to_string(),
                user_id: myset.user_id.unwrap(),
                body_id: myset.body_id,
                display_order: myset.display_order,
            })
            .collect(),
    });
}
