use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use server::service;

// マイセット取得APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct GetOneResponseEntry {
    id: u32,      // マイセットID
    name: String, // マイセット名
    body_id: u32, // 素体ID
    version: u32, // バージョン
}
// マイセット取得API
#[get("/api/v1/game/mysets/{myset_id}")]
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
        body_id: myset.body_id,
        version: myset.version,
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
    id: u32,      // 装備ID
    name: String, // 装備名
    body_id: u32, // 素体ID
}
// マイセット一覧取得APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct GetListResponseEntry {
    total_count: u32,                              // 合計数
    mysets: Vec<MysetEntryOfGetListResponseEntry>, // マイセット一覧
}
// マイセット一覧取得API
#[get("/api/v1/game/mysets")]
pub async fn get_list(
    query: web::Query<GetListRequest>, // クエリパラメータ
) -> impl Responder {
    // リクエスト取得
    let user_id = Some(3); // TODO 認証情報から取得
    let sort_by = query.sort_by;
    let limit = query.limit;
    let offset = query.offset;

    // データ取得
    let mysets = service::myset::find_list(user_id, sort_by, limit, offset);

    // レスポンス加工
    let mut response = GetListResponseEntry {
        total_count: mysets.total_count,
        mysets: Vec::new(),
    };
    for myset in &mysets.mysets {
        response.mysets.push(MysetEntryOfGetListResponseEntry {
            id: myset.id,
            name: myset.name.to_string(),
            body_id: myset.body_id,
        });
    }
    return HttpResponse::Ok().json(response);
}

// マイセット登録APIリクエスト
#[derive(Deserialize)]
pub struct RegisterRequestBody {
    name: String, // マイセット名
    body_id: u32, // 素体ID
}
// マイセット登録APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct RegisterResponseEntry {
    id: u32,      // マイセットID
    name: String, // マイセット名
    body_id: u32, // 素体ID
}
// マイセット登録API
#[post("/api/v1/game/mysets")]
pub async fn register(
    request_body: web::Json<RegisterRequestBody>, // リクエストボディ
) -> impl Responder {
    // リクエスト取得
    let user_id = 3; // TODO 認証情報から取得
    let name = request_body.name.to_string();
    let body_id = request_body.body_id;

    // データ登録
    let myset = service::myset::register(name, user_id, body_id);

    // レスポンス加工
    return HttpResponse::Ok().json(RegisterResponseEntry {
        id: myset.id,
        name: myset.name,
        body_id: myset.body_id,
    });
}

// マイセット更新APIリクエスト
#[derive(Deserialize)]
pub struct UpdateRequestBody {
    name: String, // マイセット名
    body_id: u32, // 素体ID
}
// マイセット更新APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct UpdateResponseEntry {
    id: u32,      // マイセットID
    name: String, // マイセット名
    body_id: u32, // 素体ID
    version: u32, // バージョン
}
// マイセット更新API
#[put("/api/v1/game/mysets/{myset_id}")]
pub async fn update(
    web::Path(myset_id): web::Path<u32>, // マイセットID - パスパラメータ
    request_body: web::Json<UpdateRequestBody>, // リクエストボディ
) -> impl Responder {
    // リクエスト取得
    let myset_id: Option<u32> = Some(myset_id);
    let name = request_body.name.to_string();
    let body_id = request_body.body_id;

    // データ更新
    let myset = service::myset::update(myset_id.unwrap(), name, body_id);

    // レスポンス加工
    return HttpResponse::Ok().json(UpdateResponseEntry {
        id: myset.id,
        name: myset.name,
        body_id: myset.body_id,
        version: myset.version,
    });
}

// マイセット削除APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct DeleteResponseEntry {
    id: u32,      // マイセットID
    name: String, // マイセット名
    body_id: u32, // 素体ID
    version: u32, // バージョン
}
// マイセット削除API
#[delete("/api/v1/game/mysets/{myset_id}")]
pub async fn delete(
    web::Path(myset_id): web::Path<u32>, // マイセットID - パスパラメータ
) -> impl Responder {
    // リクエスト取得
    let myset_id: Option<u32> = Some(myset_id);

    // データ削除
    let myset = service::myset::delete(myset_id.unwrap());

    // レスポンス加工
    return HttpResponse::Ok().json(DeleteResponseEntry {
        id: myset.id,
        name: myset.name,
        body_id: myset.body_id,
        version: myset.version,
    });
}
