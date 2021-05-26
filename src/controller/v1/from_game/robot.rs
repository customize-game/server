use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use server::service;

// ステータス
#[derive(Serialize, Deserialize, Debug)]
struct StatusOfGetOneResponseEntry {
    body_id: i32,       // 素体ID
    parameter_id: i32,  // パラメータID
    num: i32,           // 増減値
    name: String,       // パラメータ名
    display_order: i32, // 表示順
    version: i32,       // バージョン
}
// hogeインタフェース
#[derive(Serialize, Deserialize, Debug)]
struct HogeInterfaceOfGetOneResponseEntry {
    body_id: i32,           // 素体ID
    hoge_interface_id: i32, // hogeインタフェースID
    name: String,           // hogeインタフェース名
    display_order: i32,     // 表示順
    version: i32,           // バージョン
}
// ソケット
#[derive(Serialize, Deserialize, Debug)]
struct SocketOfGetOneResponseEntry {
    body_id: i32,             // 素体ID
    x: i32,                   // X座標
    y: i32,                   // Y座標
    operator: Option<String>, // 演算子
    num: Option<i32>,         // 増減値
    version: i32,             // バージョン
}
// ロボット取得APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct GetOneResponseEntry {
    id: i32,                                                  // ロボットID
    name: String,                                             // ロボット名
    ruby: Option<String>,                                     // ルビ
    flavor: Option<String>,                                   // フレーバーテキスト
    display_order: i32,                                       // 表示順
    version: i32,                                             // バージョン
    having: bool,                                             // 取得済みかどうか
    statuses: Vec<StatusOfGetOneResponseEntry>,               // ステータス
    hoge_interfaces: Vec<HogeInterfaceOfGetOneResponseEntry>, // hogeインタフェース
    sockets: Vec<SocketOfGetOneResponseEntry>,                // ソケット
}
// ロボット取得API
#[get("/api/v1/game/robots/{body_id}")]
pub async fn get_one(
    web::Path(body_id): web::Path<i32>, // ロボットID - パスパラメータ
) -> impl Responder {
    
    let user_id = Some(3); // TODO 認証情報から取得

    // データ取得
    let robot = service::robot::find_by_id(
        body_id,
        user_id,
    ).unwrap();

    // レスポンス加工
    return HttpResponse::Ok().json(GetOneResponseEntry {
        id: robot.id,
        name: robot.name,
        ruby: robot.ruby,
        flavor: robot.flavor,
        display_order: robot.display_order,
        version: robot.version,
        having: robot.having.unwrap(),
        statuses: robot
            .statuses
            .iter()
            .map(|status| StatusOfGetOneResponseEntry {
                body_id: status.body_id,
                parameter_id: status.parameter_id,
                num: status.num,
                name: status.name.to_string(),
                display_order: status.display_order,
                version: status.version,
            })
            .collect(),
        hoge_interfaces: robot
            .hoge_interfaces
            .iter()
            .map(|hoge_interface| HogeInterfaceOfGetOneResponseEntry {
                body_id: hoge_interface.body_id,
                hoge_interface_id: hoge_interface.hoge_interface_id,
                name: hoge_interface.name.to_string(),
                display_order: hoge_interface.display_order,
                version: hoge_interface.version,
            })
            .collect(),
        sockets: robot
            .sockets
            .iter()
            .map(|socket| SocketOfGetOneResponseEntry {
                body_id: socket.body_id,
                x: socket.x,
                y: socket.y,
                operator: socket.operator.clone(),
                num: socket.num,
                version: socket.version,
            })
            .collect(),
    });
}

// ロボット一覧取得APIクエリパラメータ
#[derive(Deserialize)]
pub struct GetListRequest {
    only_having: Option<bool>, // 取得済みのみ取得するかどうか
    sort_by: Option<String>,   // ソート種別
    limit: Option<i32>,        // 取得数
    offset: Option<i32>,       // 取得位置
}
// ロボット一覧取得APIレスポンスのロボット
#[derive(Serialize, Deserialize, Debug)]
struct RobotEntryOfGetListResponseEntry {
    id: i32,            // ロボットID
    name: String,       // ロボット名
    having: bool,       // 取得済みかどうか
    display_order: i32, // 表示順
}
// ロボット一覧取得APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct GetListResponseEntry {
    total_count: usize,                              // 合計数
    robots: Vec<RobotEntryOfGetListResponseEntry>, // 装備一覧
}
// ロボット一覧取得API
#[get("/api/v1/game/robots/myself")]
pub async fn get_list(
    query: web::Query<GetListRequest>, // クエリパラメータ
) -> impl Responder {
    // リクエスト取得
    let user_id = Some(3); // TODO 認証情報から取得
    let only_having = query.only_having;
    let sort_by = query.sort_by.clone();
    let limit = query.limit;
    let offset = query.offset;

    // データ取得
    let robots = service::robot::find_list(
        user_id, 
        only_having, 
        sort_by, 
        limit, 
        offset
    ).unwrap();

    // レスポンス加工
    return HttpResponse::Ok().json(GetListResponseEntry {
        total_count: robots.total_count,
        robots: robots
            .robots
            .iter()
            .map(|robot| RobotEntryOfGetListResponseEntry {
                id: robot.id,
                name: robot.name.to_string(),
                having: robot.having.unwrap(),
                display_order: robot.display_order,
            })
            .collect(),
    });
}
