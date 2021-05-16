use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use server::service;

// 装備取得APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct GetOneResponseEntry {
    id: i32,                // 装備ID
    name: String,           // 装備名
    ruby: Option<String>,   // ルビ
    flavor: Option<String>, // フレーバーテキスト
    add_socket_count: i32,  // 装備時に増えるソケット数
    display_order: i32,     // 表示順
    version: i32,           // バージョン
}
// 装備取得API
// ex.)
//   curl -X GET -H "Content-Type: application/json" -v http://localhost:5000/api/v1/manager/equipments/2 | jq
#[get("/api/v1/manager/equipments/{equipment_id}")]
pub async fn get_one(
    web::Path(equipment_id): web::Path<i32>, // 装備ID - パスパラメータ
) -> impl Responder {
    
    let user_id = None;

    // データ取得
    let equipment = service::equipment::find_by_id(
        equipment_id,
        user_id,
    ).unwrap();

    // レスポンス加工
    return HttpResponse::Ok().json(GetOneResponseEntry {
        id: equipment.id,
        name: equipment.name,
        ruby: equipment.ruby,
        flavor: equipment.flavor,
        add_socket_count: equipment.add_socket_count,
        display_order: equipment.display_order,
        version: equipment.version,
    });
}

// 装備一覧取得APIクエリパラメータ
#[derive(Deserialize)]
pub struct GetListRequest {
    sort_by: Option<String>, // ソート種別
    limit: Option<i32>,      // 取得数
    offset: Option<i32>,     // 取得位置
}
// 装備一覧取得APIレスポンスの装備
#[derive(Serialize, Deserialize, Debug)]
struct EquipmentEntryOfGetListResponseEntry {
    id: i32,                // 装備ID
    name: String,           // 装備名
    ruby: Option<String>,   // ルビ
    flavor: Option<String>, // フレーバーテキスト
    add_socket_count: i32,  // 装備時に増えるソケット数
    display_order: i32,     // 表示順
}
// 装備一覧取得APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct GetListResponseEntry {
    total_count: usize,                                    // 合計数
    equipments: Vec<EquipmentEntryOfGetListResponseEntry>, // 装備一覧
}
// 装備一覧取得API
// ex.)
//   curl -X GET -H "Content-Type: application/json" -v http://localhost:5000/api/v1/manager/equipments | jq
#[get("/api/v1/manager/equipments")]
pub async fn get_list(
    query: web::Query<GetListRequest>, // クエリパラメータ
) -> impl Responder {
    // リクエスト取得
    let only_having = Some(true);
    let sort_by = query.sort_by.clone();
    let limit = query.limit;
    let offset = query.offset;

    // データ取得
    let equipments = service::equipment::find_list(
        None, 
        only_having, 
        sort_by, 
        limit, 
        offset
    ).unwrap();

    // レスポンス加工
    return HttpResponse::Ok().json(GetListResponseEntry {
        total_count: equipments.total_count,
        equipments: equipments
            .equipments
            .iter()
            .map(|equipment| EquipmentEntryOfGetListResponseEntry {
                id: equipment.id,
                name: equipment.name.to_string(),
                ruby: equipment.ruby.clone(),
                flavor: equipment.flavor.clone(),
                add_socket_count: equipment.add_socket_count,
                display_order: equipment.display_order,
            })
            .collect(),
    });
}

// 装備登録APIリクエスト
#[derive(Deserialize)]
pub struct RegisterRequestBody {
    name: String,           // 装備名
    ruby: Option<String>,   // ルビ
    flavor: Option<String>, // フレーバーテキスト
    add_socket_count: i32,  // 装備時に増えるソケット数
    display_order: i32,     // 表示順
}
// 装備登録APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct RegisterResponseEntry {
    register_count: usize, // 登録件数
}
// 装備登録API
// ex.)
//   curl -X POST -H "Content-Type: application/json" -v http://localhost:5000/api/v1/manager/equipments --data '{"name":"ビームサーベル" ,"ruby":"びーむさーべる" , "flavor":"強い!", "display_order":3, "add_socket_count":3}' | jq
#[post("/api/v1/manager/equipments")]
pub async fn register(
    request_body: web::Json<RegisterRequestBody>, // リクエストボディ
) -> impl Responder {
    // リクエスト取得
    let name = request_body.name.clone();
    let ruby = request_body.ruby.clone();
    let flavor = request_body.flavor.clone();
    let add_socket_count = request_body.add_socket_count;
    let display_order = request_body.display_order;

    // データ登録
    let register_count = service::equipment::register(
        name, 
        ruby, 
        flavor, 
        add_socket_count, 
        display_order
    ).unwrap();

    // レスポンス加工
    return HttpResponse::Ok().json(RegisterResponseEntry {
        register_count: register_count,
    });
}

// 装備更新APIリクエスト
#[derive(Deserialize)]
pub struct UpdateRequestBody {
    name: String,           // 装備名
    ruby: Option<String>,   // ルビ
    flavor: Option<String>, // フレーバーテキスト
    add_socket_count: i32,  // 装備時に増えるソケット数
    display_order: i32,     // 表示順
    version: i32,           // バージョン
}
// 装備更新APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct UpdateResponseEntry {
    update_count: usize, // 更新件数
}
// 装備更新API
// ex.)
//   curl -X PUT -H "Content-Type: application/json" -v http://localhost:5000/api/v1/manager/equipments/3 --data '{"name":"ビームサーベル2" ,"ruby":"びーむさーべる2" , "flavor":"強い!!!!", "display_order":33, "add_socket_count":23, "version":0}' | jq
#[put("/api/v1/manager/equipments/{equipment_id}")]
pub async fn update(
    web::Path(equipment_id): web::Path<i32>, // 装備ID - パスパラメータ
    request_body: web::Json<UpdateRequestBody>, // リクエストボディ
) -> impl Responder {
    // リクエスト取得
    let name = request_body.name.clone();
    let ruby = request_body.ruby.clone();
    let flavor = request_body.flavor.clone();
    let add_socket_count = request_body.add_socket_count;
    let display_order = request_body.display_order;
    let version = request_body.version;

    // データ更新
    let update_count = service::equipment::update(
        equipment_id,
        name,
        ruby,
        flavor,
        add_socket_count,
        display_order,
        version,
    ).unwrap();

    // レスポンス加工
    return HttpResponse::Ok().json(UpdateResponseEntry {
        update_count: update_count,
    });
}

// 装備削除APIリクエスト
#[derive(Deserialize)]
pub struct DeleteRequestBody {
    version: i32, // バージョン
}
// 装備削除APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct DeleteResponseEntry {
    delete_count: usize, // 削除件数
}
// 装備削除API
// ex.)
//   curl -X DELETE -H "Content-Type: application/json" -v http://localhost:5000/api/v1/manager/equipments/2 --data '{"version":0}' | jq
#[delete("/api/v1/manager/equipments/{equipment_id}")]
pub async fn delete(
    web::Path(equipment_id): web::Path<i32>,    // 装備ID - パスパラメータ
    request_body: web::Json<DeleteRequestBody>, // リクエストボディ
) -> impl Responder {
    // リクエスト取得
    let version = request_body.version;

    // データ削除
    let delete_count = service::equipment::delete(
        equipment_id,
        version
    ).unwrap();

    // レスポンス加工
    return HttpResponse::Ok().json(DeleteResponseEntry {
        delete_count: delete_count,
    });
}
