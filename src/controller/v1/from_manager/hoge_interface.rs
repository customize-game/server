use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use server::service;

// hogeインタフェース取得APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct GetOneResponseEntry {
    id: u32,            // hogeインタフェースID
    name: String,       // hogeインタフェース名
    display_order: u32, // 表示順
    is_deleted: bool,   // 削除済みかどうか
    version: u32,       // バージョン
}
// hogeインタフェース取得API
#[get("/api/v1/manager/hoge-interfaces/{hoge_interface_id}")]
pub async fn get_one(
    web::Path(hoge_interface_id): web::Path<u32>, // hogeインタフェースID - パスパラメータ
) -> impl Responder {
    // リクエスト取得
    let hoge_interface_id: Option<u32> = Some(hoge_interface_id);

    // データ取得
    let hoge_interface = service::hoge_interface::find_by_id(hoge_interface_id.unwrap());

    // レスポンス加工
    return HttpResponse::Ok().json(GetOneResponseEntry {
        id: hoge_interface.id,
        name: hoge_interface.name,
        display_order: hoge_interface.display_order,
        is_deleted: hoge_interface.is_deleted,
        version: hoge_interface.version,
    });
}

// hogeインタフェース一覧取得APIクエリパラメータ
#[derive(Deserialize)]
pub struct GetListRequest {
    sort_by: Option<u32>, // ソート種別
    limit: Option<u32>,   // 取得数
    offset: Option<u32>,  // 取得位置
}
// hogeインタフェース一覧取得APIレスポンスのhogeインタフェース
#[derive(Serialize, Deserialize, Debug)]
struct HogeInterfaceEntryOfGetListResponseEntry {
    id: u32,            // hogeインタフェースID
    name: String,       // hogeインタフェース名
    display_order: u32, // 表示順
    is_deleted: bool,   // 削除済みかどうか
}
// hogeインタフェース一覧取得APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct GetListResponseEntry {
    total_count: u32,                                               // 合計数
    hoge_interfaces: Vec<HogeInterfaceEntryOfGetListResponseEntry>, // hogeインタフェース一覧
}
// hogeインタフェース一覧取得API
#[get("/api/v1/manager/hoge-interfaces")]
pub async fn get_list(
    query: web::Query<GetListRequest>, // クエリパラメータ
) -> impl Responder {
    // リクエスト取得
    let sort_by = query.sort_by;
    let limit = query.limit;
    let offset = query.offset;

    // データ取得
    let hoge_interfaces = service::hoge_interface::find_list(sort_by, limit, offset);

    // レスポンス加工
    return HttpResponse::Ok().json(GetListResponseEntry {
        total_count: hoge_interfaces.total_count,
        hoge_interfaces: hoge_interfaces
            .hoge_interfaces
            .iter()
            .map(|hoge_interface| HogeInterfaceEntryOfGetListResponseEntry {
                id: hoge_interface.id,
                name: hoge_interface.name.to_string(),
                display_order: hoge_interface.display_order,
                is_deleted: hoge_interface.is_deleted,
            })
            .collect(),
    });
}

// hogeインタフェース登録APIリクエスト
#[derive(Deserialize)]
pub struct RegisterRequestBody {
    name: String,       // hogeインタフェース名
    display_order: u32, // 表示順
}
// hogeインタフェース登録APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct RegisterResponseEntry {
    id: u32,            // hogeインタフェースID
    name: String,       // hogeインタフェース名
    display_order: u32, // 表示順
}
// hogeインタフェース登録API
#[post("/api/v1/manager/hoge-interfaces")]
pub async fn register(
    request_body: web::Json<RegisterRequestBody>, // リクエストボディ
) -> impl Responder {
    // リクエスト取得
    let name = request_body.name.to_string();
    let display_order = request_body.display_order;

    // データ登録
    let hoge_interface = service::hoge_interface::register(name, display_order);

    // レスポンス加工
    return HttpResponse::Ok().json(RegisterResponseEntry {
        id: hoge_interface.id,
        name: hoge_interface.name,
        display_order: hoge_interface.display_order,
    });
}

// hogeインタフェース更新APIリクエスト
#[derive(Deserialize)]
pub struct UpdateRequestBody {
    name: String,       // hogeインタフェース名
    display_order: u32, // 表示順
    is_deleted: bool,   // 削除済みかどうか
}
// 装備更新APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct UpdateResponseEntry {
    id: u32,            // hogeインタフェースID
    name: String,       // hogeインタフェース名
    display_order: u32, // 表示順
    is_deleted: bool,   // 削除済みかどうか
    version: u32,       // バージョン
}
// hogeインタフェース更新API
#[put("/api/v1/manager/hoge-interfaces/{hoge_interface_id}")]
pub async fn update(
    web::Path(hoge_interface_id): web::Path<u32>, // hogeインタフェースID - パスパラメータ
    request_body: web::Json<UpdateRequestBody>,   // リクエストボディ
) -> impl Responder {
    // リクエスト取得
    let hoge_interface_id: Option<u32> = Some(hoge_interface_id);
    let name = request_body.name.to_string();
    let display_order = request_body.display_order;
    let is_deleted = request_body.is_deleted;

    // データ更新
    let hoge_interface = service::hoge_interface::update(
        hoge_interface_id.unwrap(),
        name,
        display_order,
        is_deleted,
    );

    // レスポンス加工
    return HttpResponse::Ok().json(UpdateResponseEntry {
        id: hoge_interface.id,
        name: hoge_interface.name,
        display_order: hoge_interface.display_order,
        is_deleted: hoge_interface.is_deleted,
        version: hoge_interface.version,
    });
}

// hogeインタフェース削除APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct DeleteResponseEntry {
    id: u32,            // hogeインタフェースID
    name: String,       // hogeインタフェース名
    display_order: u32, // 表示順
    is_deleted: bool,   // 削除済みかどうか
    version: u32,       // バージョン
}
// hogeインタフェース削除API
#[delete("/api/v1/manager/hoge-interfaces/{hoge_interface_id}")]
pub async fn delete(
    web::Path(hoge_interface_id): web::Path<u32>, // hogeインタフェースID - パスパラメータ
) -> impl Responder {
    // リクエスト取得
    let hoge_interface_id: Option<u32> = Some(hoge_interface_id);

    // データ削除
    let hoge_interface = service::hoge_interface::delete(hoge_interface_id.unwrap());

    // レスポンス加工
    return HttpResponse::Ok().json(DeleteResponseEntry {
        id: hoge_interface.id,
        name: hoge_interface.name,
        display_order: hoge_interface.display_order,
        is_deleted: hoge_interface.is_deleted,
        version: hoge_interface.version,
    });
}
