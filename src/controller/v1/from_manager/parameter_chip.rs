use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use server::service;

// パラメータチップ取得APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct GetOneResponseEntry {
    id: u32,            // パラメータチップID
    name: String,       // パラメータチップ名
    display_order: u32, // 表示順
    is_deleted: bool,   // 削除済みかどうか
    version: u32,       // バージョン
}
// パラメータチップ取得API
#[get("/api/v1/manager/parameter-chips/{parameter_chip_id}")]
pub async fn get_one(
    web::Path(parameter_chip_id): web::Path<u32>, // パラメータチップID - パスパラメータ
) -> impl Responder {
    // リクエスト取得
    let parameter_chip_id: Option<u32> = Some(parameter_chip_id);

    // データ取得
    let parameter_chip = service::parameter_chip::find_by_id(parameter_chip_id.unwrap());

    // レスポンス加工
    return HttpResponse::Ok().json(GetOneResponseEntry {
        id: parameter_chip.id,
        name: parameter_chip.name,
        display_order: parameter_chip.display_order,
        is_deleted: parameter_chip.is_deleted,
        version: parameter_chip.version,
    });
}

// パラメータチップ一覧取得APIクエリパラメータ
#[derive(Deserialize)]
pub struct GetListRequest {
    only_having: Option<bool>, // 取得済みのみ取得するかどうか
    sort_by: Option<u32>,      // ソート種別
    limit: Option<u32>,        // 取得数
    offset: Option<u32>,       // 取得位置
}
// パラメータチップ一覧取得APIレスポンスのパラメータチップ
#[derive(Serialize, Deserialize, Debug)]
struct ParameterChipEntryOfGetListResponseEntry {
    id: u32,              // パラメータチップID
    name: String,         // パラメータチップ名
    having: Option<bool>, // 取得済みかどうか
    display_order: u32,   // 表示順
}
// パラメータチップ一覧取得APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct GetListResponseEntry {
    total_count: u32,                                               // 合計数
    parameter_chips: Vec<ParameterChipEntryOfGetListResponseEntry>, // パラメータチップ一覧
}
// パラメータチップ一覧取得API
#[get("/api/v1/manager/parameter-chips")]
pub async fn get_list(
    query: web::Query<GetListRequest>, // クエリパラメータ
) -> impl Responder {
    // リクエスト取得
    let user_id = None; // TODO 認証情報から取得
    let only_having = query.only_having;
    let sort_by = query.sort_by;
    let limit = query.limit;
    let offset = query.offset;

    // データ取得
    let parameter_chips =
        service::parameter_chip::find_list(user_id, only_having, sort_by, limit, offset);

    // レスポンス加工
    let mut response = GetListResponseEntry {
        total_count: parameter_chips.total_count,
        parameter_chips: Vec::new(),
    };
    for parameter_chip in &parameter_chips.parameter_chips {
        response
            .parameter_chips
            .push(ParameterChipEntryOfGetListResponseEntry {
                id: parameter_chip.id,
                name: parameter_chip.name.to_string(),
                having: Some(false),
                display_order: parameter_chip.display_order,
            });
    }
    return HttpResponse::Ok().json(response);
}

// パラメータチップ登録APIリクエスト
#[derive(Deserialize)]
pub struct RegisterRequestBody {
    name: String,       // パラメータチップ名
    display_order: u32, // 表示順
}
// パラメータチップ登録APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct RegisterResponseEntry {
    id: u32,            // パラメータチップID
    name: String,       // パラメータチップ名
    display_order: u32, // 表示順
    is_deleted: bool,   // 削除済みかどうか
    version: u32,       // バージョン
}
// パラメータチップ登録API
#[post("/api/v1/manager/parameter-chips")]
pub async fn register(
    request_body: web::Json<RegisterRequestBody>, // リクエストボディ
) -> impl Responder {
    // リクエスト取得
    let name = request_body.name.to_string();
    let display_order = request_body.display_order;

    // データ登録
    let parameter_chip = service::parameter_chip::register(name, display_order);

    // レスポンス加工
    return HttpResponse::Ok().json(RegisterResponseEntry {
        id: parameter_chip.id,
        name: parameter_chip.name,
        display_order: parameter_chip.display_order,
        is_deleted: parameter_chip.is_deleted,
        version: parameter_chip.version,
    });
}

// パラメータチップ更新APIリクエスト
#[derive(Deserialize)]
pub struct UpdateRequestBody {
    name: String,       // パラメータチップ名
    display_order: u32, // 表示順
}
// パラメータチップ更新APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct UpdateResponseEntry {
    id: u32,            // パラメータチップID
    name: String,       // パラメータチップ名
    display_order: u32, // 表示順
    is_deleted: bool,   // 削除済みかどうか
    version: u32,       // バージョン
}
// パラメータチップ更新API
#[put("/api/v1/manager/parameter-chips/{parameter_chip_id}")]
pub async fn update(
    web::Path(parameter_chip_id): web::Path<u32>, // パラメータチップID - パスパラメータ
    request_body: web::Json<UpdateRequestBody>,   // リクエストボディ
) -> impl Responder {
    // リクエスト取得
    let parameter_chip_id: Option<u32> = Some(parameter_chip_id);
    let name = request_body.name.to_string();
    let display_order = request_body.display_order;

    // データ更新
    let parameter_chip =
        service::parameter_chip::update(parameter_chip_id.unwrap(), name, display_order);

    // レスポンス加工
    return HttpResponse::Ok().json(UpdateResponseEntry {
        id: parameter_chip.id,
        name: parameter_chip.name,
        display_order: parameter_chip.display_order,
        is_deleted: parameter_chip.is_deleted,
        version: parameter_chip.version,
    });
}

// パラメータチップ削除APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct DeleteResponseEntry {
    id: u32,            // パラメータチップID
    name: String,       // パラメータチップ名
    display_order: u32, // 表示順
    is_deleted: bool,   // 削除済みかどうか
    version: u32,       // バージョン
}
// パラメータチップ削除API
#[delete("/api/v1/manager/parameter-chips/{parameter_chip_id}")]
pub async fn delete(
    web::Path(parameter_chip_id): web::Path<u32>, // パラメータチップID - パスパラメータ
) -> impl Responder {
    // リクエスト取得
    let parameter_chip_id: Option<u32> = Some(parameter_chip_id);

    // データ削除
    let parameter_chip = service::parameter_chip::delete(parameter_chip_id.unwrap());

    // レスポンス加工
    return HttpResponse::Ok().json(DeleteResponseEntry {
        id: parameter_chip.id,
        name: parameter_chip.name,
        display_order: parameter_chip.display_order,
        is_deleted: parameter_chip.is_deleted,
        version: parameter_chip.version,
    });
}
