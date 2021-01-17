use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use server::service;

// パラメータ取得APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct GetOneResponseEntry {
    id: u32,            // パラメータID
    name: String,       // パラメータ名
    display_order: u32, // 表示順
    is_deleted: bool,   // 削除済みかどうか
    version: u32,       // バージョン
}
// パラメータ取得API
#[get("/api/v1/manager/parameters/{parameter_id}")]
pub async fn get_one(
    web::Path(parameter_id): web::Path<u32>, // パラメータID - パスパラメータ
) -> impl Responder {
    // リクエスト取得
    let parameter_id: Option<u32> = Some(parameter_id);

    // データ取得
    let parameter = service::parameter::find_by_id(parameter_id.unwrap());

    // レスポンス加工
    return HttpResponse::Ok().json(GetOneResponseEntry {
        id: parameter.id,
        name: parameter.name,
        display_order: parameter.display_order,
        is_deleted: parameter.is_deleted,
        version: parameter.version,
    });
}

// パラメータ一覧取得APIクエリパラメータ
#[derive(Deserialize)]
pub struct GetListRequest {
    sort_by: Option<u32>, // ソート種別
    limit: Option<u32>,   // 取得数
    offset: Option<u32>,  // 取得位置
}
// パラメータ一覧取得APIレスポンスのパラメータ
#[derive(Serialize, Deserialize, Debug)]
struct ParameterEntryOfGetListResponseEntry {
    id: u32,            // パラメータID
    name: String,       // パラメータ名
    display_order: u32, // 表示順
    is_deleted: bool,   // 削除済みかどうか
}
// パラメータ一覧取得APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct GetListResponseEntry {
    total_count: u32,                                      // 合計数
    parameters: Vec<ParameterEntryOfGetListResponseEntry>, // パラメータ一覧
}
// パラメータ一覧取得API
#[get("/api/v1/manager/parameters")]
pub async fn get_list(
    query: web::Query<GetListRequest>, // クエリパラメータ
) -> impl Responder {
    // リクエスト取得
    let sort_by = query.sort_by;
    let limit = query.limit;
    let offset = query.offset;

    // データ取得
    let parameters = service::parameter::find_list(sort_by, limit, offset);

    // レスポンス加工
    let mut response = GetListResponseEntry {
        total_count: parameters.total_count,
        parameters: Vec::new(),
    };
    for parameter in &parameters.parameters {
        response
            .parameters
            .push(ParameterEntryOfGetListResponseEntry {
                id: parameter.id,
                name: parameter.name.to_string(),
                display_order: parameter.display_order,
                is_deleted: parameter.is_deleted,
            });
    }
    return HttpResponse::Ok().json(response);
}

// パラメータ登録APIリクエスト
#[derive(Deserialize)]
pub struct RegisterRequestBody {
    name: String,       // パラメータ名
    display_order: u32, // 表示順
}
// パラメータ登録APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct RegisterResponseEntry {
    id: u32,            // パラメータID
    name: String,       // パラメータ名
    display_order: u32, // 表示順
}
// パラメータ登録API
#[post("/api/v1/manager/parameters")]
pub async fn register(
    request_body: web::Json<RegisterRequestBody>, // リクエストボディ
) -> impl Responder {
    // リクエスト取得
    let name = request_body.name.to_string();
    let display_order = request_body.display_order;

    // データ登録
    let parameter = service::parameter::register(name, display_order);

    // レスポンス加工
    return HttpResponse::Ok().json(RegisterResponseEntry {
        id: parameter.id,
        name: parameter.name,
        display_order: parameter.display_order,
    });
}

// パラメータ更新APIリクエスト
#[derive(Deserialize)]
pub struct UpdateRequestBody {
    name: String,       // パラメータ名
    display_order: u32, // 表示順
    is_deleted: bool,   // 削除済みかどうか
}
// パラメータ更新APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct UpdateResponseEntry {
    id: u32,            // パラメータID
    name: String,       // パラメータ名
    display_order: u32, // 表示順
    is_deleted: bool,   // 削除済みかどうか
    version: u32,       // バージョン
}
// パラメータ更新API
#[put("/api/v1/manager/parameters/{parameter_id}")]
pub async fn update(
    web::Path(parameter_id): web::Path<u32>, // パラメータID - パスパラメータ
    request_body: web::Json<UpdateRequestBody>, // リクエストボディ
) -> impl Responder {
    // リクエスト取得
    let parameter_id: Option<u32> = Some(parameter_id);
    let name = request_body.name.to_string();
    let display_order = request_body.display_order;
    let is_deleted = request_body.is_deleted;

    // データ更新
    let parameter =
        service::parameter::update(parameter_id.unwrap(), name, display_order, is_deleted);
    // レスポンス加工
    return HttpResponse::Ok().json(UpdateResponseEntry {
        id: parameter.id,
        name: parameter.name,
        display_order: parameter.display_order,
        is_deleted: parameter.is_deleted,
        version: parameter.version,
    });
}

// パラメータ削除APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct DeleteResponseEntry {
    id: u32,            // パラメータID
    name: String,       // パラメータ名
    display_order: u32, // 表示順
    is_deleted: bool,   // 削除済みかどうか
    version: u32,       // バージョン
}
// パラメータ削除API
#[delete("/api/v1/manager/parameters/{parameter_id}")]
pub async fn delete(web::Path(parameter_id): web::Path<u32>) -> impl Responder {
    // リクエスト取得
    let parameter_id: Option<u32> = Some(parameter_id);

    // データ削除
    let parameter = service::parameter::delete(parameter_id.unwrap());

    // レスポンス加工
    return HttpResponse::Ok().json(DeleteResponseEntry {
        id: parameter.id,
        name: parameter.name,
        display_order: parameter.display_order,
        is_deleted: parameter.is_deleted,
        version: parameter.version,
    });
}
