use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use server::service;

// ロボット取得APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct GetOneResponseEntry {
    id: u32,            // ロボットID
    name: String,       // ロボット名
    ruby: String,       // ルビ
    flavor: String,     // フレーバーテキスト
    display_order: u32, // 表示順
    is_deleted: bool,   // 削除済みかどうか
    version: u32,       // バージョン
}
// ロボット取得API
#[get("/api/v1/manager/robots/{body_id}")]
pub async fn get_one(
    web::Path(body_id): web::Path<u32>, // ロボットID - パスパラメータ
) -> impl Responder {
    // リクエスト取得
    let body_id: Option<u32> = Some(body_id);
    // データ取得
    let robot = service::robot::find_by_id(body_id.unwrap());

    // レスポンス加工
    return HttpResponse::Ok().json(GetOneResponseEntry {
        id: robot.id,
        name: robot.name,
        ruby: robot.ruby,
        flavor: robot.flavor,
        display_order: robot.display_order,
        is_deleted: robot.is_deleted,
        version: robot.version,
    });
}

// ロボット一覧取得APIクエリパラメータ
#[derive(Deserialize)]
pub struct GetListRequest {
    sort_by: Option<u32>, // ソート種別
    limit: Option<u32>,   // 取得数
    offset: Option<u32>,  // 取得位置
}
// ロボット一覧取得APIレスポンスのロボット
#[derive(Serialize, Deserialize, Debug)]
struct RobotEntryOfGetListResponseEntry {
    id: u32,            // ロボットID
    name: String,       // ロボット名
    display_order: u32, // 表示順
    is_deleted: bool,   // 削除済みかどうか
}
// ロボット一覧取得APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct GetListResponseEntry {
    total_count: u32,                              // 合計数
    robots: Vec<RobotEntryOfGetListResponseEntry>, // 装備一覧
}
// ロボット一覧取得API
#[get("/api/v1/manager/robots")]
pub async fn get_list(
    query: web::Query<GetListRequest>, // クエリパラメータ
) -> impl Responder {
    // リクエスト取得
    let user_id = None;
    let only_having = None;
    let sort_by = query.sort_by;
    let limit = query.limit;
    let offset = query.offset;
    // データ取得
    let robots = service::robot::find_list(user_id, only_having, sort_by, limit, offset);

    // レスポンス加工
    let mut response = GetListResponseEntry {
        total_count: robots.total_count,
        robots: Vec::new(),
    };
    for robot in &robots.robots {
        response.robots.push(RobotEntryOfGetListResponseEntry {
            id: robot.id,
            name: robot.name.to_string(),
            display_order: robot.display_order,
            is_deleted: robot.is_deleted,
        });
    }
    return HttpResponse::Ok().json(response);
}

// ロボット登録APIリクエスト
#[derive(Deserialize)]
pub struct RegisterRequestBody {
    name: String,       // ロボット名
    ruby: String,       // ルビ
    flavor: String,     // フレーバーテキスト
    display_order: u32, // 表示順
}
// ロボット登録APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct RegisterResponseEntry {
    id: u32,            // ロボットID
    name: String,       // ロボット名
    ruby: String,       // ルビ
    flavor: String,     // フレーバーテキスト
    display_order: u32, // 表示順
    is_deleted: bool,   // 削除済みかどうか
    version: u32,       // バージョン
}
// ロボット登録API
#[post("/api/v1/manager/robots")]
pub async fn register(
    request_body: web::Json<RegisterRequestBody>, // リクエストボディ
) -> impl Responder {
    // リクエスト取得
    let name = request_body.name.to_string();
    let ruby = request_body.ruby.to_string();
    let flavor = request_body.flavor.to_string();
    let display_order = request_body.display_order;

    // データ登録
    let robot = service::robot::register(name, ruby, flavor, display_order);

    // レスポンス加工
    return HttpResponse::Ok().json(RegisterResponseEntry {
        id: robot.id,
        name: robot.name,
        ruby: robot.ruby,
        flavor: robot.flavor,
        display_order: robot.display_order,
        is_deleted: robot.is_deleted,
        version: robot.version,
    });
}

// ロボット更新APIリクエスト
#[derive(Deserialize)]
pub struct UpdateRequestBody {
    name: String,       // ロボット名
    ruby: String,       // ルビ
    flavor: String,     // フレーバーテキスト
    display_order: u32, // 表示順
    is_deleted: bool,   // 削除済みかどうか
}
// ロボット更新APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct UpdateResponseEntry {
    id: u32,            // ロボットID
    name: String,       // ロボット名
    ruby: String,       // ルビ
    flavor: String,     // フレーバーテキスト
    display_order: u32, // 表示順
    is_deleted: bool,   // 削除済みかどうか
    version: u32,       // バージョン
}
// ロボット更新API
#[put("/api/v1/manager/robots/{body_id}")]
pub async fn update(
    web::Path(body_id): web::Path<u32>, // ロボットID - パスパラメータ
    request_body: web::Json<UpdateRequestBody>, // リクエストボディ
) -> impl Responder {
    // リクエスト取得
    let body_id: Option<u32> = Some(body_id);
    let name = request_body.name.to_string();
    let ruby = request_body.ruby.to_string();
    let flavor = request_body.flavor.to_string();
    let display_order = request_body.display_order;
    let is_deleted = request_body.is_deleted;

    // データ更新
    let robot = service::robot::update(
        body_id.unwrap(),
        name,
        ruby,
        flavor,
        display_order,
        is_deleted,
    );

    // レスポンス加工
    return HttpResponse::Ok().json(UpdateResponseEntry {
        id: robot.id,
        name: robot.name,
        ruby: robot.ruby,
        flavor: robot.flavor,
        display_order: robot.display_order,
        is_deleted: robot.is_deleted,
        version: robot.version,
    });
}
// ロボット削除APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct DeleteResponseEntry {
    id: u32,            // ロボットID
    name: String,       // ロボット名
    ruby: String,       // ルビ
    flavor: String,     // フレーバーテキスト
    display_order: u32, // 表示順
    is_deleted: bool,   // 削除済みかどうか
    version: u32,       // バージョン
}
// ロボット削除API
#[delete("/api/v1/manager/robots/{body_id}")]
pub async fn delete(
    web::Path(body_id): web::Path<u32>, // ロボットID - パスパラメータ
) -> impl Responder {
    // リクエスト取得
    let body_id: Option<u32> = Some(body_id);

    // データ削除
    let robot = service::robot::delete(body_id.unwrap());

    // レスポンス加工
    return HttpResponse::Ok().json(UpdateResponseEntry {
        id: robot.id,
        name: robot.name,
        ruby: robot.ruby,
        flavor: robot.flavor,
        display_order: robot.display_order,
        is_deleted: robot.is_deleted,
        version: robot.version,
    });
}
