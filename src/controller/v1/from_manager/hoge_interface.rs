use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use server::service;

// hogeインタフェース取得APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct GetOneResponseEntry {
    id: i32,            // hogeインタフェースID
    name: String,       // hogeインタフェース名
    display_order: i32, // 表示順
    version: i32,       // バージョン
}
// hogeインタフェース取得API
// ex.)
//    curl -X GET -v http://localhost:5000/api/v1/manager/hoge-interfaces/1 | jq
#[get("/api/v1/manager/hoge-interfaces/{hoge_interface_id}")]
pub async fn get_one(
    web::Path(hoge_interface_id): web::Path<i32>, // hogeインタフェースID - パスパラメータ
) -> impl Responder {
    
    // データ取得
    let hoge_interface = service::hoge_interface::find_by_id(hoge_interface_id).unwrap();

    // レスポンス加工
    return HttpResponse::Ok().json(GetOneResponseEntry {
        id: hoge_interface.id,
        name: hoge_interface.name,
        display_order: hoge_interface.display_order,
        version: hoge_interface.version,
    });
}

// hogeインタフェース一覧取得APIクエリパラメータ
#[derive(Deserialize)]
pub struct GetListRequest {
    sort_by: Option<i32>, // ソート種別
    limit: Option<i32>,   // 取得数
    offset: Option<i32>,  // 取得位置
}
// hogeインタフェース一覧取得APIレスポンスのhogeインタフェース
#[derive(Serialize, Deserialize, Debug)]
struct HogeInterfaceEntryOfGetListResponseEntry {
    id: i32,            // hogeインタフェースID
    name: String,       // hogeインタフェース名
    display_order: i32, // 表示順
}
// hogeインタフェース一覧取得APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct GetListResponseEntry {
    total_count: usize,                                             // 合計数
    hoge_interfaces: Vec<HogeInterfaceEntryOfGetListResponseEntry>, // hogeインタフェース一覧
}
// hogeインタフェース一覧取得API
// ex.)
//   curl -X GET -v http://localhost:5000/api/v1/manager/hoge-interfaces | jq
#[get("/api/v1/manager/hoge-interfaces")]
pub async fn get_list(
    query: web::Query<GetListRequest>, // クエリパラメータ
) -> impl Responder {
    // リクエスト取得
    let sort_by = query.sort_by;
    let limit = query.limit;
    let offset = query.offset;

    // データ取得
    let hoge_interfaces = service::hoge_interface::find_list(sort_by, limit, offset).unwrap();

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
            })
            .collect(),
    });
}

// hogeインタフェース登録APIリクエスト
#[derive(Deserialize)]
pub struct RegisterRequestBody {
    name: String,       // hogeインタフェース名
    display_order: i32, // 表示順
}
// hogeインタフェース登録APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct RegisterResponseEntry {
    register_count: usize, // 登録件数
}
// hogeインタフェース登録API
// ex.)
//   curl -X POST -H "Content-Type: application/json" -v http://localhost:5000/api/v1/manager/hoge-interfaces --data '{ "name":"右手" , "display_order":345 }' | jq
#[post("/api/v1/manager/hoge-interfaces")]
pub async fn register(
    request_body: web::Json<RegisterRequestBody>, // リクエストボディ
) -> impl Responder {
    // リクエスト取得
    let name = request_body.name.to_string();
    let display_order = request_body.display_order;

    // データ登録
    let register_count = service::hoge_interface::register(name, display_order).unwrap();

    // レスポンス加工
    return HttpResponse::Ok().json(RegisterResponseEntry {
        register_count: register_count,
    });
}

// hogeインタフェース更新APIリクエスト
#[derive(Deserialize)]
pub struct UpdateRequestBody {
    name: String,       // hogeインタフェース名
    display_order: i32, // 表示順
    version: i32,       // バージョン
}
// 装備更新APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct UpdateResponseEntry {
    update_count: usize, // 更新件数
}
// hogeインタフェース更新API
// ex.)
//   curl -X PUT -H "Content-Type: application/json" -v http://localhost:5000/api/v1/manager/hoge-interfaces/3 --data '{ "name":"左手" , "display_order":2, "version":0 }' | jq
#[put("/api/v1/manager/hoge-interfaces/{hoge_interface_id}")]
pub async fn update(
    web::Path(hoge_interface_id): web::Path<i32>, // hogeインタフェースID - パスパラメータ
    request_body: web::Json<UpdateRequestBody>,   // リクエストボディ
) -> impl Responder {
    // リクエスト取得
    let name = request_body.name.to_string();
    let display_order = request_body.display_order;
    let version = request_body.version;

    // データ更新
    let update_count = service::hoge_interface::update(
        hoge_interface_id,
        name,
        display_order,
        version ,
    ).unwrap();

    // レスポンス加工
    return HttpResponse::Ok().json(UpdateResponseEntry {
        update_count: update_count,
    });
}

// hogeインタフェース削除APIリクエスト
#[derive(Deserialize)]
pub struct DeleteRequestBody {
    version: i32, // バージョン
}
// hogeインタフェース削除APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct DeleteResponseEntry {
    delete_count: usize, // 削除件数
}
// hogeインタフェース削除API
// ex.)
//   curl -X DELETE -H "Content-Type: application/json" -v http://localhost:5000/api/v1/manager/hoge-interfaces/3 --data '{ "version":1 }' | jq
#[delete("/api/v1/manager/hoge-interfaces/{hoge_interface_id}")]
pub async fn delete(
    web::Path(hoge_interface_id): web::Path<i32>, // hogeインタフェースID - パスパラメータ
    request_body: web::Json<DeleteRequestBody>,   // リクエストボディ
) -> impl Responder {
    // リクエスト取得
    let version = request_body.version;

    // データ削除
    let delete_count = service::hoge_interface::delete(hoge_interface_id, version).unwrap();

    // レスポンス加工
    return HttpResponse::Ok().json(DeleteResponseEntry {
        delete_count: delete_count,
    });
}
