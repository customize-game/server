use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
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
    statuses: Vec<StatusOfGetOneResponseEntry>,               // ステータス
    hoge_interfaces: Vec<HogeInterfaceOfGetOneResponseEntry>, // hogeインタフェース
    sockets: Vec<SocketOfGetOneResponseEntry>,                // ソケット
}
// ロボット取得API
// ex.)
//   curl -X GET -H "Content-Type: application/json" -v http://localhost:5000/api/v1/manager/robots/3 | jq
#[get("/api/v1/manager/robots/{body_id}")]
pub async fn get_one(
    web::Path(body_id): web::Path<i32>, // ロボットID - パスパラメータ
) -> impl Responder {

    let user_id = None;

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
    sort_by: Option<String>, // ソート種別
    limit: Option<i32>,   // 取得数
    offset: Option<i32>,  // 取得位置
}
// ロボット取得APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct RobotEntryOfGetListResponseEntry {
    id: i32,                // ロボットID
    name: String,           // ロボット名
    ruby: Option<String>,   // ルビ
    flavor: Option<String>, // フレーバーテキスト
    display_order: i32,     // 表示順
}
// ロボット一覧取得APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct GetListResponseEntry {
    total_count: usize,                              // 合計数
    robots: Vec<RobotEntryOfGetListResponseEntry>, // 装備一覧
}
// ロボット一覧取得API
// ex.)
//   curl -X GET -H "Content-Type: application/json" -v http://localhost:5000/api/v1/manager/robots | jq
#[get("/api/v1/manager/robots")]
pub async fn get_list(
    query: web::Query<GetListRequest>, // クエリパラメータ
) -> impl Responder {
    // リクエスト取得
    let user_id = None;
    let only_having = None;
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
                ruby: robot.ruby.clone(),
                flavor: robot.flavor.clone(),
                display_order: robot.display_order,
            })
            .collect(),
    });
}

// ロボット登録APIの空きソケット
#[derive(Deserialize)]
pub struct FreeSocketOfRegisterRequestBody {
    x: i32,                   // X座標
    y: i32,                   // Y座標
    operator: Option<String>, // 演算子
    num: Option<i32>,         // 増減値
}
// ロボット登録APIのステータス
#[derive(Deserialize)]
pub struct StatusOfRegisterRequestBody {
    parameter_id: i32, // パラメータID
    num: i32,          // 増減値
}
// ロボット登録APIリクエスト
#[derive(Deserialize)]
pub struct RegisterRequestBody {
    name: String,                                             // ロボット名
    ruby: Option<String>,                                     // ルビ
    flavor: Option<String>,                                   // フレーバーテキスト
    display_order: i32,                                       // 表示順
    sockets: Vec<FreeSocketOfRegisterRequestBody>,            // 空きソケット一覧
    hoge_interfaces: Vec<i32>, // hogeインタフェース一覧
    statuses: Vec<StatusOfRegisterRequestBody>,               // ステータス一覧
}
// ロボット登録APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct RegisterResponseEntry {
    register_count: usize, // 登録件数
}
// ロボット登録API
// ex.)
//   curl -X POST -H "Content-Type: application/json" -v http://localhost:5000/api/v1/manager/robots --data '{"name": "ザク2","ruby": "ザクツー","flavor": "強くなったやられ役","display_order": 323,"sockets": [{"x": 24,"y": 2,"operator": "plus","num": 3},{"x": 1,"y": 4,"operator": "minus","num": 4},{"x": 1,"y": 3}],"hoge_interfaces":[1,3,5],"statuses":[{"parameter_id":1,"num":3},{"parameter_id":3,"num":4},{"parameter_id":2,"num":5}]}' | jq
#[post("/api/v1/manager/robots")]
pub async fn register(
    request_body: web::Json<RegisterRequestBody>, // リクエストボディ
) -> impl Responder {
    // リクエスト取得
    let name = request_body.name.to_string();
    let ruby = request_body.ruby.clone();
    let flavor = request_body.flavor.clone();
    let display_order = request_body.display_order;

    // データ登録
    let register_count = service::robot::register(
        name, 
        ruby, 
        flavor, 
        display_order,
        request_body.sockets.iter().map(|socket|service::robot::RegisterSocketEntry{
            x: socket.x,
            y: socket.y,
            operator: socket.operator.clone(),
            num: socket.num,
        })
        .collect(),
        request_body.hoge_interfaces.iter().map(|hoge_interface|service::robot::RegisterHogeInterfaceEntry{
            hoge_interface_id: *hoge_interface,
        })
        .collect(),
        request_body.statuses.iter().map(|status|service::robot::RegisterStatusEntry{
            parameter_id: status.parameter_id,
            num: status.num,
        })
        .collect(),
    ).unwrap();

    // レスポンス加工
    return HttpResponse::Ok().json(RegisterResponseEntry {
        register_count: register_count,
    });
}

// ロボット更新APIの空きソケット
#[derive(Deserialize)]
pub struct FreeSocketOfUpdateRequestBody {
    x: i32,                   // X座標
    y: i32,                   // Y座標
    operator: Option<String>, // 演算子
    num: Option<i32>,         // 増減値
}
// ロボット更新APIのステータス
#[derive(Deserialize)]
pub struct StatusOfUpdateRequestBody {
    parameter_id: i32, // パラメータID
    num: i32,          // 増減値
}
// ロボット更新APIリクエスト
#[derive(Deserialize)]
pub struct UpdateRequestBody {
    name: String,           // ロボット名
    ruby: Option<String>,   // ルビ
    flavor: Option<String>, // フレーバーテキスト
    display_order: i32,     // 表示順
    version: i32,           // バージョン
    sockets: Vec<FreeSocketOfUpdateRequestBody>, // 空きソケット一覧
    hoge_interfaces: Vec<i32>, // hogeインタフェース一覧
    statuses: Vec<StatusOfUpdateRequestBody>,               // ステータス一覧
}
// ロボット更新APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct UpdateResponseEntry {
    update_count: usize, // 更新件数
}
// ロボット更新API
// ex.)
//   curl -X PUT -H "Content-Type: application/json" -v http://localhost:5000/api/v1/manager/robots/3 --data '{"name": "ザク3","ruby": "ザクスリー","flavor": "さらに強くなった","display_order": 223,"version":0,"sockets": [{"x": 2,"y": 12,"operator": "plus","num": 33},{"x": 1,"y": 41,"operator": "minus","num": 41},{"x": 1,"y": 3}],"hoge_interfaces":[1,3,5,2,4,6],"statuses":[{"parameter_id":2,"num":3},{"parameter_id":3,"num":4},{"parameter_id":4,"num":5}]}' | jq
#[put("/api/v1/manager/robots/{body_id}")]
pub async fn update(
    web::Path(body_id): web::Path<i32>, // ロボットID - パスパラメータ
    request_body: web::Json<UpdateRequestBody>, // リクエストボディ
) -> impl Responder {
    // リクエスト取得
    let body_id: Option<i32> = Some(body_id);
    let name = request_body.name.to_string();
    let ruby = request_body.ruby.clone();
    let flavor = request_body.flavor.clone();
    let display_order = request_body.display_order;
    let version = request_body.version;

    // データ更新
    let update_count = service::robot::update(
        body_id.unwrap(),
        name,
        ruby,
        flavor,
        display_order,
        version,
        request_body.sockets.iter().map(|socket|service::robot::RegisterSocketEntry{
            x: socket.x,
            y: socket.y,
            operator: socket.operator.clone(),
            num: socket.num,
        })
        .collect(),
        request_body.hoge_interfaces.iter().map(|hoge_interface|service::robot::RegisterHogeInterfaceEntry{
            hoge_interface_id: *hoge_interface,
        })
        .collect(),
        request_body.statuses.iter().map(|status|service::robot::RegisterStatusEntry{
            parameter_id: status.parameter_id,
            num: status.num,
        })
        .collect(),
    ).unwrap();

    // レスポンス加工
    return HttpResponse::Ok().json(UpdateResponseEntry {
        update_count: update_count,
    });
}

// ロボット削除APIリクエスト
#[derive(Deserialize)]
pub struct DeleteRequestBody {
    version: i32, // バージョン
}
// ロボット削除APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct DeleteResponseEntry {
    delete_count: usize, // 削除件数
}
// ロボット削除API
// ex.)
//   curl -X DELETE -H "Content-Type: application/json" -v http://localhost:5000/api/v1/manager/robots/4 --data '{"version":0}' | jq
#[delete("/api/v1/manager/robots/{body_id}")]
pub async fn delete(
    web::Path(body_id): web::Path<i32>,         // ロボットID - パスパラメータ
    request_body: web::Json<DeleteRequestBody>, // リクエストボディ
) -> impl Responder {
    // リクエスト取得
    let version = request_body.version;

    // データ削除
    let delete_count = service::robot::delete(body_id, version).unwrap();

    // レスポンス加工
    return HttpResponse::Ok().json(DeleteResponseEntry {
        delete_count: delete_count,
    });
}
