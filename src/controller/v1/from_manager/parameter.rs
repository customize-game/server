use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use server::service;

// パラメータ取得APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct GetOneResponseEntry {
    id: i32,            // パラメータID
    name: String,       // パラメータ名
    display_order: i32, // 表示順
    version: i32,       // バージョン
}
// パラメータ取得API
// ex.)
//    curl -X GET -v http://localhost:5000/api/v1/manager/parameters/1 | jq
#[get("/api/v1/manager/parameters/{parameter_id}")]
pub async fn get_one(
    web::Path(parameter_id): web::Path<i32>, // パラメータID - パスパラメータ
) -> impl Responder {

    // データ取得
    let parameter = service::parameter::find_by_id(parameter_id).unwrap();

    // レスポンス加工
    return HttpResponse::Ok().json(GetOneResponseEntry {
        id: parameter.id,
        name: parameter.name,
        display_order: parameter.display_order,
        version: parameter.version,
    });
}

// パラメータ一覧取得APIクエリパラメータ
#[derive(Deserialize)]
pub struct GetListRequest {
    sort_by: Option<i32>, // ソート種別
    limit: Option<i32>,   // 取得数
    offset: Option<i32>,  // 取得位置
}
// パラメータ一覧取得APIレスポンスのパラメータ
#[derive(Serialize, Deserialize, Debug)]
struct ParameterEntryOfGetListResponseEntry {
    id: i32,            // パラメータID
    name: String,       // パラメータ名
    display_order: i32, // 表示順
}
// パラメータ一覧取得APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct GetListResponseEntry {
    total_count: usize,                                      // 合計数
    parameters: Vec<ParameterEntryOfGetListResponseEntry>, // パラメータ一覧
}
// パラメータ一覧取得API
// ex.)
//   curl -X GET -v http://localhost:5000/api/v1/manager/parameters | jq
#[get("/api/v1/manager/parameters")]
pub async fn get_list(
    query: web::Query<GetListRequest>, // クエリパラメータ
) -> impl Responder {
    // リクエスト取得
    let sort_by = query.sort_by;
    let limit = query.limit;
    let offset = query.offset;

    // データ取得
    let parameters = service::parameter::find_list(sort_by, limit, offset).unwrap();

    // レスポンス加工
    return HttpResponse::Ok().json(GetListResponseEntry {
        total_count: parameters.total_count,
        parameters: parameters
            .parameters
            .iter()
            .map(|parameter| ParameterEntryOfGetListResponseEntry {
                id: parameter.id,
                name: parameter.name.to_string(),
                display_order: parameter.display_order,
            })
            .collect(),
    });
}

// パラメータ登録APIリクエスト
#[derive(Deserialize)]
pub struct RegisterRequestBody {
    name: String,       // パラメータ名
    display_order: i32, // 表示順
}
// パラメータ登録APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct RegisterResponseEntry {
    register_count: usize, // 登録件数
}
// パラメータ登録API
// ex.)
//   curl -X POST -H "Content-Type: application/json" -v http://localhost:5000/api/v1/manager/parameters --data '{ "name":"攻撃力" , "display_order":2 }' | jq
#[post("/api/v1/manager/parameters")]
pub async fn register(
    request_body: web::Json<RegisterRequestBody>, // リクエストボディ
) -> impl Responder {
    // リクエスト取得
    let name = request_body.name.to_string();
    let display_order = request_body.display_order;

    // データ登録
    let register_count = service::parameter::register(name, display_order).unwrap();

    // レスポンス加工
    return HttpResponse::Ok().json(RegisterResponseEntry {
        register_count: register_count,
    });
}

// パラメータ更新APIリクエスト
#[derive(Deserialize)]
pub struct UpdateRequestBody {
    name: String,       // パラメータ名
    display_order: i32, // 表示順
    version: i32,       // バージョン
}
// パラメータ更新APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct UpdateResponseEntry {
    update_count: usize, // 更新件数
}
// パラメータ更新API
// ex.)
//   curl -X PUT -H "Content-Type: application/json" -v http://localhost:5000/api/v1/manager/parameters/3 --data '{ "name":"防御力" , "display_order":29, "version":0 }' | jq
#[put("/api/v1/manager/parameters/{parameter_id}")]
pub async fn update(
    web::Path(parameter_id): web::Path<i32>, // パラメータID - パスパラメータ
    request_body: web::Json<UpdateRequestBody>, // リクエストボディ
) -> impl Responder {
    // リクエスト取得
    let name = request_body.name.to_string();
    let display_order = request_body.display_order;
    let version = request_body.version;

    // データ更新
    let update_count = service::parameter::update(
        parameter_id,
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
// パラメータ削除APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct DeleteResponseEntry {
    delete_count: usize, // 削除件数
}
// パラメータ削除API
// ex.)
//   curl -X DELETE -H "Content-Type: application/json" -v http://localhost:5000/api/v1/manager/parameters/3 --data '{ "version":1 }' | jq
#[delete("/api/v1/manager/parameters/{parameter_id}")]
pub async fn delete(
    web::Path(parameter_id): web::Path<i32>, // パラメータID - パスパラメータ
    request_body: web::Json<DeleteRequestBody>,   // リクエストボディ
) -> impl Responder {
    // リクエスト取得
    let version = request_body.version;

    // データ削除
    let delete_count = service::parameter::delete(parameter_id, version).unwrap();

    // レスポンス加工
    return HttpResponse::Ok().json(DeleteResponseEntry {
        delete_count: delete_count,
    });
}
