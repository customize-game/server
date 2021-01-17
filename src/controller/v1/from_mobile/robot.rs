use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use server::service;

// ロボット取得APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct GetOneResponseEntry {
    id: u32,        // ロボットID
    name: String,   // ロボット名
    ruby: String,   // ルビ
    flavor: String, // フレーバーテキスト
}
// ロボット取得API
#[get("/api/v1/mobile/robots/{body_id}")]
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
    });
}

// ロボット一覧取得APIクエリパラメータ
#[derive(Deserialize)]
pub struct GetListRequest {
    only_having: Option<bool>, // 取得済みのみ取得するかどうか
    sort_by: Option<u32>,      // ソート種別
    limit: Option<u32>,        // 取得数
    offset: Option<u32>,       // 取得位置
}
// ロボット一覧取得APIレスポンスのロボット
#[derive(Serialize, Deserialize, Debug)]
struct RobotEntryOfGetListResponseEntry {
    id: u32,              // ロボットID
    name: String,         // ロボット名
    having: Option<bool>, // 取得済みかどうか
    display_order: u32,   // 表示順
}
// ロボット一覧取得APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct GetListResponseEntry {
    total_count: u32,                              // 合計数
    robots: Vec<RobotEntryOfGetListResponseEntry>, // 装備一覧
}
// ロボット一覧取得API
#[get("/api/v1/mobile/robots")]
pub async fn get_list(
    query: web::Query<GetListRequest>, // クエリパラメータ
) -> impl Responder {
    // リクエスト取得
    let user_id = Some(3); // TODO 認証情報から取得
    let only_having = query.only_having;
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
            having: Some(false),
            display_order: robot.display_order,
        });
    }
    return HttpResponse::Ok().json(response);
}
