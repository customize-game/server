use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use server::service;

// 装備取得APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct GetOneResponseEntry {
    id: u32,               // 装備ID
    name: String,          // 装備名
    ruby: String,          // ルビ
    flavor: String,        // フレーバーテキスト
    add_socket_count: u32, // 装備時に増えるソケット数
    display_order: u32,    // 表示順
    is_deleted: bool,      // 削除済みかどうか
    version: u32,          // バージョン
}
// 装備取得API
#[get("/api/v1/manager/equipments/{equipment_id}")]
pub async fn get_one(
    web::Path(equipment_id): web::Path<u32>, // 装備ID - パスパラメータ
) -> impl Responder {
    // リクエスト取得
    let equipment_id: Option<u32> = Some(equipment_id);

    // データ取得
    let equipment = service::equipment::find_by_id(equipment_id.unwrap());

    // レスポンス加工
    return HttpResponse::Ok().json(GetOneResponseEntry {
        id: equipment.id,
        name: equipment.name,
        ruby: equipment.ruby,
        flavor: equipment.flavor,
        add_socket_count: equipment.add_socket_count,
        display_order: equipment.display_order,
        is_deleted: equipment.is_deleted,
        version: equipment.version,
    });
}

// 装備一覧取得APIクエリパラメータ
#[derive(Deserialize)]
pub struct GetListRequest {
    sort_by: Option<u32>, // ソート種別
    limit: Option<u32>,   // 取得数
    offset: Option<u32>,  // 取得位置
}
// 装備一覧取得APIレスポンスの装備
#[derive(Serialize, Deserialize, Debug)]
struct EquipmentEntryOfGetListResponseEntry {
    id: u32,               // 装備ID
    name: String,          // 装備名
    ruby: String,          // ルビ
    flavor: String,        // フレーバーテキスト
    add_socket_count: u32, // 装備時に増えるソケット数
    display_order: u32,    // 表示順
    is_deleted: bool,      // 削除済みかどうか
}
// 装備一覧取得APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct GetListResponseEntry {
    total_count: u32,                                      // 合計数
    equipments: Vec<EquipmentEntryOfGetListResponseEntry>, // 装備一覧
}
// 装備一覧取得API
#[get("/api/v1/manager/equipments")]
pub async fn get_list(
    query: web::Query<GetListRequest>, // クエリパラメータ
) -> impl Responder {
    // リクエスト取得
    let only_having = Some(true);
    let sort_by = query.sort_by;
    let limit = query.limit;
    let offset = query.offset;

    // データ取得
    let equipments = service::equipment::find_list(only_having, sort_by, limit, offset);

    // レスポンス加工
    let mut response = GetListResponseEntry {
        total_count: equipments.total_count,
        equipments: Vec::new(),
    };
    for equipment in &equipments.equipments {
        response
            .equipments
            .push(EquipmentEntryOfGetListResponseEntry {
                id: equipment.id,
                name: equipment.name.to_string(),
                ruby: equipment.ruby.to_string(),
                flavor: equipment.flavor.to_string(),
                add_socket_count: equipment.add_socket_count,
                display_order: equipment.display_order,
                is_deleted: equipment.is_deleted,
            });
    }
    return HttpResponse::Ok().json(response);
}

// 装備登録APIリクエスト
#[derive(Deserialize)]
pub struct RegisterRequestBody {
    name: String,          // 装備名
    ruby: String,          // ルビ
    flavor: String,        // フレーバーテキスト
    add_socket_count: u32, // 装備時に増えるソケット数
    display_order: u32,    // 表示順
}
// 装備登録APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct RegisterResponseEntry {
    id: u32,            // 装備ID
    name: String,       // 装備名
    ruby: String,       // ルビ
    flavor: String,     // フレーバーテキスト
    display_order: u32, // 装備時に増えるソケット数
    is_deleted: bool,   // 削除済みかどうか
    version: u32,       // バージョン
}
// 装備登録API
#[post("/api/v1/manager/equipments")]
pub async fn register(
    request_body: web::Json<RegisterRequestBody>, // リクエストボディ
) -> impl Responder {
    // リクエスト取得
    let name = request_body.name.to_string();
    let ruby = request_body.ruby.to_string();
    let flavor = request_body.flavor.to_string();
    let add_socket_count = request_body.add_socket_count;
    let display_order = request_body.display_order;

    // データ登録
    let equipment =
        service::equipment::register(name, ruby, flavor, add_socket_count, display_order);

    // レスポンス加工
    return HttpResponse::Ok().json(RegisterResponseEntry {
        id: equipment.id,
        name: equipment.name,
        ruby: equipment.ruby,
        flavor: equipment.flavor,
        display_order: equipment.display_order,
        is_deleted: equipment.is_deleted,
        version: equipment.version,
    });
}

// 装備更新APIリクエスト
#[derive(Deserialize)]
pub struct UpdateRequestBody {
    name: String,          // 装備名
    ruby: String,          // ルビ
    flavor: String,        // フレーバーテキスト
    add_socket_count: u32, // 装備時に増えるソケット数
    display_order: u32,    // 表示順
    is_deleted: bool,      // 削除済みかどうか
}
// 装備更新APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct UpdateResponseEntry {
    id: u32,            // 装備ID
    name: String,       // 装備名
    ruby: String,       // ルビ
    flavor: String,     // フレーバーテキスト
    display_order: u32, // 表示順
    is_deleted: bool,   // 削除済みかどうか
    version: u32,       // バージョン
}
// 装備更新API
#[put("/api/v1/manager/equipments/{equipment_id}")]
pub async fn update(
    web::Path(equipment_id): web::Path<u32>, // 装備ID - パスパラメータ
    request_body: web::Json<UpdateRequestBody>, // リクエストボディ
) -> impl Responder {
    // リクエスト取得
    let equipment_id: Option<u32> = Some(equipment_id);
    let name = request_body.name.to_string();
    let ruby = request_body.ruby.to_string();
    let flavor = request_body.flavor.to_string();
    let add_socket_count = request_body.add_socket_count;
    let display_order = request_body.display_order;
    let is_deleted = request_body.is_deleted;

    // データ更新
    let equipment = service::equipment::update(
        equipment_id.unwrap(),
        name,
        ruby,
        flavor,
        add_socket_count,
        display_order,
        is_deleted,
    );

    // レスポンス加工
    return HttpResponse::Ok().json(UpdateResponseEntry {
        id: equipment.id,
        name: equipment.name,
        ruby: equipment.ruby,
        flavor: equipment.flavor,
        display_order: equipment.display_order,
        is_deleted: equipment.is_deleted,
        version: equipment.version,
    });
}

// 装備削除APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct DeleteResponseEntry {
    id: u32,            // 装備ID
    name: String,       // 装備名
    ruby: String,       // ルビ
    flavor: String,     // フレーバー
    display_order: u32, // 表示順
    is_deleted: bool,   // 削除済みかどうか
    version: u32,       // バージョン
}
// 装備削除API
#[delete("/api/v1/manager/equipments/{equipment_id}")]
pub async fn delete(
    web::Path(equipment_id): web::Path<u32>, // 装備ID - パスパラメータ
) -> impl Responder {
    // リクエスト取得
    let equipment_id: Option<u32> = Some(equipment_id);

    // データ削除
    let equipment = service::equipment::delete(equipment_id.unwrap());

    // レスポンス加工
    return HttpResponse::Ok().json(DeleteResponseEntry {
        id: equipment.id,
        name: equipment.name,
        ruby: equipment.ruby,
        flavor: equipment.flavor,
        display_order: equipment.display_order,
        is_deleted: equipment.is_deleted,
        version: equipment.version,
    });
}
