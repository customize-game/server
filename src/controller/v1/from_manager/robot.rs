use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use server::service;

// ステータス
#[derive(Serialize, Deserialize, Debug)]
struct StatusOfGetOneResponseEntry {
    body_id: u32,           // 素体ID
    parameter_id: u32,      // パラメータID
    num: Option<u32>,       // 増減値
    status_version: u32,    // ステータスバージョン
    name: String,           // パラメータ名
    display_order: u32,     // 表示順
    is_deleted: bool,       // 削除済みかどうか
    parameter_version: u32, // パラメータバージョン
}
// hogeインタフェース
#[derive(Serialize, Deserialize, Debug)]
struct HogeInterfaceOfGetOneResponseEntry {
    body_id: u32,                        // 素体ID
    hoge_interface_id: u32,              // hogeインタフェースID
    bodies_hoge_interfaces_version: u32, // 素体：hogeインタフェースバージョン
    name: String,                        // hogeインタフェース名
    display_order: u32,                  // 表示順
    is_deleted: bool,                    // 削除済みかどうか
    hoge_interface_version: u32,         // hogeインタフェースバージョン
}
// ソケット
#[derive(Serialize, Deserialize, Debug)]
struct SocketOfGetOneResponseEntry {
    body_id: u32,             // 素体ID
    x: u32,                   // X座標
    y: u32,                   // Y座標
    operator: Option<String>, // 演算子
    num: Option<u32>,         // 増減値
    version: u32,             // バージョン
}
// ロボット取得APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct GetOneResponseEntry {
    id: u32,                                                  // ロボットID
    name: String,                                             // ロボット名
    ruby: Option<String>,                                     // ルビ
    flavor: Option<String>,                                   // フレーバーテキスト
    display_order: u32,                                       // 表示順
    is_deleted: bool,                                         // 削除済みかどうか
    version: u32,                                             // バージョン
    statuses: Vec<StatusOfGetOneResponseEntry>,               // ステータス
    hoge_interfaces: Vec<HogeInterfaceOfGetOneResponseEntry>, // hogeインタフェース
    sockets: Vec<SocketOfGetOneResponseEntry>,                // ソケット
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
        statuses: robot
            .statuses
            .iter()
            .map(|status| StatusOfGetOneResponseEntry {
                body_id: status.body_id,
                parameter_id: status.parameter_id,
                num: status.num,
                status_version: status.status_version,
                name: status.name.to_string(),
                display_order: status.display_order,
                is_deleted: status.is_deleted,
                parameter_version: status.parameter_version,
            })
            .collect(),
        hoge_interfaces: robot
            .hoge_interfaces
            .iter()
            .map(|hoge_interface| HogeInterfaceOfGetOneResponseEntry {
                body_id: hoge_interface.body_id,
                hoge_interface_id: hoge_interface.hoge_interface_id,
                bodies_hoge_interfaces_version: hoge_interface.bodies_hoge_interfaces_version,
                name: hoge_interface.name.to_string(),
                display_order: hoge_interface.display_order,
                is_deleted: hoge_interface.is_deleted,
                hoge_interface_version: hoge_interface.hoge_interface_version,
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
    sort_by: Option<u32>, // ソート種別
    limit: Option<u32>,   // 取得数
    offset: Option<u32>,  // 取得位置
}
// ロボット取得APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct RobotEntryOfGetListResponseEntry {
    id: u32,                // ロボットID
    name: String,           // ロボット名
    ruby: Option<String>,   // ルビ
    flavor: Option<String>, // フレーバーテキスト
    display_order: u32,     // 表示順
    is_deleted: bool,       // 削除済みかどうか
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
                is_deleted: robot.is_deleted,
            })
            .collect(),
    });
}

// ロボット登録APIリクエスト
#[derive(Deserialize)]
pub struct RegisterRequestBody {
    name: String,           // ロボット名
    ruby: Option<String>,   // ルビ
    flavor: Option<String>, // フレーバーテキスト
    display_order: u32,     // 表示順
}
// ステータス
#[derive(Serialize, Deserialize, Debug)]
struct StatusOfRegisterResponseEntry {
    body_id: u32,           // 素体ID
    parameter_id: u32,      // パラメータID
    num: Option<u32>,       // 増減値
    status_version: u32,    // ステータスバージョン
    name: String,           // パラメータ名
    display_order: u32,     // 表示順
    is_deleted: bool,       // 削除済みかどうか
    parameter_version: u32, // パラメータバージョン
}
// hogeインタフェース
#[derive(Serialize, Deserialize, Debug)]
struct HogeInterfaceOfRegisterResponseEntry {
    body_id: u32,                        // 素体ID
    hoge_interface_id: u32,              // hogeインタフェースID
    bodies_hoge_interfaces_version: u32, // 素体：hogeインタフェースバージョン
    name: String,                        // hogeインタフェース名
    display_order: u32,                  // 表示順
    is_deleted: bool,                    // 削除済みかどうか
    hoge_interface_version: u32,         // hogeインタフェースバージョン
}
// ソケット
#[derive(Serialize, Deserialize, Debug)]
struct SocketOfRegisterResponseEntry {
    body_id: u32,             // 素体ID
    x: u32,                   // X座標
    y: u32,                   // Y座標
    operator: Option<String>, // 演算子
    num: Option<u32>,         // 増減値
    version: u32,             // バージョン
}
// ロボット登録APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct RegisterResponseEntry {
    id: u32,                                                    // ロボットID
    name: String,                                               // ロボット名
    ruby: Option<String>,                                       // ルビ
    flavor: Option<String>,                                     // フレーバーテキスト
    display_order: u32,                                         // 表示順
    is_deleted: bool,                                           // 削除済みかどうか
    version: u32,                                               // バージョン
    statuses: Vec<StatusOfRegisterResponseEntry>,               // ステータス
    hoge_interfaces: Vec<HogeInterfaceOfRegisterResponseEntry>, // hogeインタフェース
    sockets: Vec<SocketOfRegisterResponseEntry>,                // ソケット
}
// ロボット登録API
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
        statuses: robot
            .statuses
            .iter()
            .map(|status| StatusOfRegisterResponseEntry {
                body_id: status.body_id,
                parameter_id: status.parameter_id,
                num: status.num,
                status_version: status.status_version,
                name: status.name.to_string(),
                display_order: status.display_order,
                is_deleted: status.is_deleted,
                parameter_version: status.parameter_version,
            })
            .collect(),
        hoge_interfaces: robot
            .hoge_interfaces
            .iter()
            .map(|hoge_interface| HogeInterfaceOfRegisterResponseEntry {
                body_id: hoge_interface.body_id,
                hoge_interface_id: hoge_interface.hoge_interface_id,
                bodies_hoge_interfaces_version: hoge_interface.bodies_hoge_interfaces_version,
                name: hoge_interface.name.to_string(),
                display_order: hoge_interface.display_order,
                is_deleted: hoge_interface.is_deleted,
                hoge_interface_version: hoge_interface.hoge_interface_version,
            })
            .collect(),
        sockets: robot
            .sockets
            .iter()
            .map(|socket| SocketOfRegisterResponseEntry {
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

// ロボット更新APIリクエスト
#[derive(Deserialize)]
pub struct UpdateRequestBody {
    name: String,           // ロボット名
    ruby: Option<String>,   // ルビ
    flavor: Option<String>, // フレーバーテキスト
    display_order: u32,     // 表示順
    is_deleted: bool,       // 削除済みかどうか
}
// ステータス
#[derive(Serialize, Deserialize, Debug)]
struct StatusOfUpdateResponseEntry {
    body_id: u32,           // 素体ID
    parameter_id: u32,      // パラメータID
    num: Option<u32>,       // 増減値
    status_version: u32,    // ステータスバージョン
    name: String,           // パラメータ名
    display_order: u32,     // 表示順
    is_deleted: bool,       // 削除済みかどうか
    parameter_version: u32, // パラメータバージョン
}
// hogeインタフェース
#[derive(Serialize, Deserialize, Debug)]
struct HogeInterfaceOfUpdateResponseEntry {
    body_id: u32,                        // 素体ID
    hoge_interface_id: u32,              // hogeインタフェースID
    bodies_hoge_interfaces_version: u32, // 素体：hogeインタフェースバージョン
    name: String,                        // hogeインタフェース名
    display_order: u32,                  // 表示順
    is_deleted: bool,                    // 削除済みかどうか
    hoge_interface_version: u32,         // hogeインタフェースバージョン
}
// ソケット
#[derive(Serialize, Deserialize, Debug)]
struct SocketOfUpdateResponseEntry {
    body_id: u32,             // 素体ID
    x: u32,                   // X座標
    y: u32,                   // Y座標
    operator: Option<String>, // 演算子
    num: Option<u32>,         // 増減値
    version: u32,             // バージョン
}
// ロボット更新APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct UpdateResponseEntry {
    id: u32,                                                  // ロボットID
    name: String,                                             // ロボット名
    ruby: Option<String>,                                     // ルビ
    flavor: Option<String>,                                   // フレーバーテキスト
    display_order: u32,                                       // 表示順
    is_deleted: bool,                                         // 削除済みかどうか
    version: u32,                                             // バージョン
    statuses: Vec<StatusOfUpdateResponseEntry>,               // ステータス
    hoge_interfaces: Vec<HogeInterfaceOfUpdateResponseEntry>, // hogeインタフェース
    sockets: Vec<SocketOfUpdateResponseEntry>,                // ソケット
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
    let ruby = request_body.ruby.clone();
    let flavor = request_body.flavor.clone();
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
        statuses: robot
            .statuses
            .iter()
            .map(|status| StatusOfUpdateResponseEntry {
                body_id: status.body_id,
                parameter_id: status.parameter_id,
                num: status.num,
                status_version: status.status_version,
                name: status.name.to_string(),
                display_order: status.display_order,
                is_deleted: status.is_deleted,
                parameter_version: status.parameter_version,
            })
            .collect(),
        hoge_interfaces: robot
            .hoge_interfaces
            .iter()
            .map(|hoge_interface| HogeInterfaceOfUpdateResponseEntry {
                body_id: hoge_interface.body_id,
                hoge_interface_id: hoge_interface.hoge_interface_id,
                bodies_hoge_interfaces_version: hoge_interface.bodies_hoge_interfaces_version,
                name: hoge_interface.name.to_string(),
                display_order: hoge_interface.display_order,
                is_deleted: hoge_interface.is_deleted,
                hoge_interface_version: hoge_interface.hoge_interface_version,
            })
            .collect(),
        sockets: robot
            .sockets
            .iter()
            .map(|socket| SocketOfUpdateResponseEntry {
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

// ステータス
#[derive(Serialize, Deserialize, Debug)]
struct StatusOfDeleteResponseEntry {
    body_id: u32,           // 素体ID
    parameter_id: u32,      // パラメータID
    num: Option<u32>,       // 増減値
    status_version: u32,    // ステータスバージョン
    name: String,           // パラメータ名
    display_order: u32,     // 表示順
    is_deleted: bool,       // 削除済みかどうか
    parameter_version: u32, // パラメータバージョン
}
// hogeインタフェース
#[derive(Serialize, Deserialize, Debug)]
struct HogeInterfaceOfDeleteResponseEntry {
    body_id: u32,                        // 素体ID
    hoge_interface_id: u32,              // hogeインタフェースID
    bodies_hoge_interfaces_version: u32, // 素体：hogeインタフェースバージョン
    name: String,                        // hogeインタフェース名
    display_order: u32,                  // 表示順
    is_deleted: bool,                    // 削除済みかどうか
    hoge_interface_version: u32,         // hogeインタフェースバージョン
}
// ソケット
#[derive(Serialize, Deserialize, Debug)]
struct SocketOfDeleteResponseEntry {
    body_id: u32,             // 素体ID
    x: u32,                   // X座標
    y: u32,                   // Y座標
    operator: Option<String>, // 演算子
    num: Option<u32>,         // 増減値
    version: u32,             // バージョン
}
// ロボット更新APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct DeleteResponseEntry {
    id: u32,                                                  // ロボットID
    name: String,                                             // ロボット名
    ruby: Option<String>,                                     // ルビ
    flavor: Option<String>,                                   // フレーバーテキスト
    display_order: u32,                                       // 表示順
    is_deleted: bool,                                         // 削除済みかどうか
    version: u32,                                             // バージョン
    statuses: Vec<StatusOfDeleteResponseEntry>,               // ステータス
    hoge_interfaces: Vec<HogeInterfaceOfDeleteResponseEntry>, // hogeインタフェース
    sockets: Vec<SocketOfDeleteResponseEntry>,                // ソケット
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
    return HttpResponse::Ok().json(DeleteResponseEntry {
        id: robot.id,
        name: robot.name,
        ruby: robot.ruby,
        flavor: robot.flavor,
        display_order: robot.display_order,
        is_deleted: robot.is_deleted,
        version: robot.version,
        statuses: robot
            .statuses
            .iter()
            .map(|status| StatusOfDeleteResponseEntry {
                body_id: status.body_id,
                parameter_id: status.parameter_id,
                num: status.num,
                status_version: status.status_version,
                name: status.name.to_string(),
                display_order: status.display_order,
                is_deleted: status.is_deleted,
                parameter_version: status.parameter_version,
            })
            .collect(),
        hoge_interfaces: robot
            .hoge_interfaces
            .iter()
            .map(|hoge_interface| HogeInterfaceOfDeleteResponseEntry {
                body_id: hoge_interface.body_id,
                hoge_interface_id: hoge_interface.hoge_interface_id,
                bodies_hoge_interfaces_version: hoge_interface.bodies_hoge_interfaces_version,
                name: hoge_interface.name.to_string(),
                display_order: hoge_interface.display_order,
                is_deleted: hoge_interface.is_deleted,
                hoge_interface_version: hoge_interface.hoge_interface_version,
            })
            .collect(),
        sockets: robot
            .sockets
            .iter()
            .map(|socket| SocketOfDeleteResponseEntry {
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
