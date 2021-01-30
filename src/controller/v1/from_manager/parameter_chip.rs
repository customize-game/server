use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use server::service;

// パラメータチップ取得APIのソケット
#[derive(Serialize, Deserialize, Debug)]
struct SocketOfGetOneResponseEntry {
    parameter_chip_id: u32, // パラメータチップID
    x: u32,                 // X座標
    y: u32,                 // Y座標
    version: u32,           // ソケットバージョン
}
// パラメータチップ取得APIの効果
#[derive(Serialize, Deserialize, Debug)]
struct EffectOfGetOneResponseEntry {
    parameter_chip_id: u32,     // パラメータチップID
    parameter_id: u32,          // パラメータID
    num: Option<u32>,           // 増減値
    effect_version: u32,        // パラメータチップ効果バージョン
    name: String,               // パラメータ名
    display_order: u32,         // パラメータ表示順
    parameter_is_deleted: bool, // パラメータが削除されているかどうか
    parameter_version: u32,     // パラメータバージョン
}
// パラメータチップ取得APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct GetOneResponseEntry {
    id: u32,                                   // パラメータチップID
    name: String,                              // パラメータチップ名
    display_order: u32,                        // 表示順
    version: u32,                              // バージョン
    having_count: Option<u32>,                 // 所持数
    sockets: Vec<SocketOfGetOneResponseEntry>, // ソケット一覧
    effects: Vec<EffectOfGetOneResponseEntry>, // 効果一覧
}
// パラメータチップ取得API
#[get("/api/v1/manager/parameter-chips/{parameter_chip_id}")]
pub async fn get_one(
    web::Path(parameter_chip_id): web::Path<u32>, // パラメータチップID - パスパラメータ
) -> impl Responder {
    // リクエスト取得
    let parameter_chip_id: Option<u32> = Some(parameter_chip_id);

    // データ取得
    let parameter_chip = service::parameter_chip::find_by_id(parameter_chip_id.unwrap());

    // レスポンス加工
    return HttpResponse::Ok().json(GetOneResponseEntry {
        id: parameter_chip.id,
        name: parameter_chip.name,
        display_order: parameter_chip.display_order,
        version: parameter_chip.version,
        having_count: parameter_chip.having_count,
        sockets: parameter_chip
            .sockets
            .iter()
            .map(|socket| SocketOfGetOneResponseEntry {
                parameter_chip_id: socket.parameter_chip_id,
                x: socket.x,
                y: socket.y,
                version: socket.version,
            })
            .collect(),
        effects: parameter_chip
            .effects
            .iter()
            .map(|effect| EffectOfGetOneResponseEntry {
                parameter_chip_id: effect.parameter_chip_id,
                parameter_id: effect.parameter_id,
                num: effect.num,
                effect_version: effect.effect_version,
                name: effect.name.to_string(),
                display_order: effect.display_order,
                parameter_is_deleted: effect.parameter_is_deleted,
                parameter_version: effect.parameter_version,
            })
            .collect(),
    });
}

// パラメータチップ一覧取得APIクエリパラメータ
#[derive(Deserialize)]
pub struct GetListRequest {
    sort_by: Option<u32>, // ソート種別
    limit: Option<u32>,   // 取得数
    offset: Option<u32>,  // 取得位置
}
// パラメータチップ一覧APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct ParameterChipEntryOfGetListResponseEntry {
    id: u32,            // パラメータチップID
    name: String,       // パラメータチップ名
    display_order: u32, // 表示順
    version: u32,       // バージョン
}
// パラメータチップ一覧取得APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct GetListResponseEntry {
    total_count: u32,                                               // 合計数
    parameter_chips: Vec<ParameterChipEntryOfGetListResponseEntry>, // パラメータチップ一覧
}
// パラメータチップ一覧取得API
#[get("/api/v1/manager/parameter-chips")]
pub async fn get_list(
    query: web::Query<GetListRequest>, // クエリパラメータ
) -> impl Responder {
    // リクエスト取得
    let user_id = None; // TODO 認証情報から取得
    let only_having = None;
    let sort_by = query.sort_by;
    let limit = query.limit;
    let offset = query.offset;

    // データ取得
    let parameter_chips =
        service::parameter_chip::find_list(user_id, only_having, sort_by, limit, offset);

    // レスポンス加工
    return HttpResponse::Ok().json(GetListResponseEntry {
        total_count: parameter_chips.total_count,
        parameter_chips: parameter_chips
            .parameter_chips
            .iter()
            .map(|parameter_chip| ParameterChipEntryOfGetListResponseEntry {
                id: parameter_chip.id,
                name: parameter_chip.name.to_string(),
                display_order: parameter_chip.display_order,
                version: parameter_chip.version,
            })
            .collect(),
    });
}

// パラメータチップ登録APIリクエスト
#[derive(Deserialize)]
pub struct RegisterRequestBody {
    name: String,       // パラメータチップ名
    display_order: u32, // 表示順
}
// パラメータチップ登録APIのソケット
#[derive(Serialize, Deserialize, Debug)]
struct SocketOfRegisterResponseEntry {
    parameter_chip_id: u32, // パラメータチップID
    x: u32,                 // X座標
    y: u32,                 // Y座標
    version: u32,           // ソケットバージョン
}
// パラメータチップ登録APIの効果
#[derive(Serialize, Deserialize, Debug)]
struct EffectOfRegisterResponseEntry {
    parameter_chip_id: u32,     // パラメータチップID
    parameter_id: u32,          // パラメータID
    num: Option<u32>,           // 増減値
    effect_version: u32,        // パラメータチップ効果バージョン
    name: String,               // パラメータ名
    display_order: u32,         // パラメータ表示順
    parameter_is_deleted: bool, // パラメータが削除されているかどうか
    parameter_version: u32,     // パラメータバージョン
}
// パラメータチップ登録APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct RegisterResponseEntry {
    id: u32,                                     // パラメータチップID
    name: String,                                // パラメータチップ名
    display_order: u32,                          // 表示順
    version: u32,                                // バージョン
    having_count: Option<u32>,                   // 所持数
    sockets: Vec<SocketOfRegisterResponseEntry>, // ソケット一覧
    effects: Vec<EffectOfRegisterResponseEntry>, // 効果一覧
}
// パラメータチップ登録API
#[post("/api/v1/manager/parameter-chips")]
pub async fn register(
    request_body: web::Json<RegisterRequestBody>, // リクエストボディ
) -> impl Responder {
    // リクエスト取得
    let name = request_body.name.to_string();
    let display_order = request_body.display_order;

    // データ登録
    let parameter_chip = service::parameter_chip::register(name, display_order);

    // レスポンス加工
    return HttpResponse::Ok().json(RegisterResponseEntry {
        id: parameter_chip.id,
        name: parameter_chip.name,
        display_order: parameter_chip.display_order,
        version: parameter_chip.version,
        having_count: parameter_chip.having_count,
        sockets: parameter_chip
            .sockets
            .iter()
            .map(|socket| SocketOfRegisterResponseEntry {
                parameter_chip_id: socket.parameter_chip_id,
                x: socket.x,
                y: socket.y,
                version: socket.version,
            })
            .collect(),
        effects: parameter_chip
            .effects
            .iter()
            .map(|effect| EffectOfRegisterResponseEntry {
                parameter_chip_id: effect.parameter_chip_id,
                parameter_id: effect.parameter_id,
                num: effect.num,
                effect_version: effect.effect_version,
                name: effect.name.to_string(),
                display_order: effect.display_order,
                parameter_is_deleted: effect.parameter_is_deleted,
                parameter_version: effect.parameter_version,
            })
            .collect(),
    });
}

// パラメータチップ更新APIリクエスト
#[derive(Deserialize)]
pub struct UpdateRequestBody {
    name: String,       // パラメータチップ名
    display_order: u32, // 表示順
    is_deleted: bool,   // 削除済みかどうか
}
// パラメータチップ更新APIのソケット
#[derive(Serialize, Deserialize, Debug)]
struct SocketOfUpdateResponseEntry {
    parameter_chip_id: u32, // パラメータチップID
    x: u32,                 // X座標
    y: u32,                 // Y座標
    version: u32,           // ソケットバージョン
}
// パラメータチップ更新APIの効果
#[derive(Serialize, Deserialize, Debug)]
struct EffectOfUpdateResponseEntry {
    parameter_chip_id: u32,     // パラメータチップID
    parameter_id: u32,          // パラメータID
    num: Option<u32>,           // 増減値
    effect_version: u32,        // パラメータチップ効果バージョン
    name: String,               // パラメータ名
    display_order: u32,         // パラメータ表示順
    parameter_is_deleted: bool, // パラメータが削除されているかどうか
    parameter_version: u32,     // パラメータバージョン
}
// パラメータチップ更新APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct UpdateResponseEntry {
    id: u32,                                   // パラメータチップID
    name: String,                              // パラメータチップ名
    display_order: u32,                        // 表示順
    version: u32,                              // バージョン
    having_count: Option<u32>,                 // 所持数
    sockets: Vec<SocketOfUpdateResponseEntry>, // ソケット一覧
    effects: Vec<EffectOfUpdateResponseEntry>, // 効果一覧
}
// パラメータチップ更新API
#[put("/api/v1/manager/parameter-chips/{parameter_chip_id}")]
pub async fn update(
    web::Path(parameter_chip_id): web::Path<u32>, // パラメータチップID - パスパラメータ
    request_body: web::Json<UpdateRequestBody>,   // リクエストボディ
) -> impl Responder {
    // リクエスト取得
    let parameter_chip_id: Option<u32> = Some(parameter_chip_id);
    let name = request_body.name.to_string();
    let display_order = request_body.display_order;
    let is_deleted = request_body.is_deleted;

    // データ更新
    let parameter_chip = service::parameter_chip::update(
        parameter_chip_id.unwrap(),
        name,
        display_order,
        is_deleted,
    );

    // レスポンス加工
    return HttpResponse::Ok().json(UpdateResponseEntry {
        id: parameter_chip.id,
        name: parameter_chip.name,
        display_order: parameter_chip.display_order,
        version: parameter_chip.version,
        having_count: parameter_chip.having_count,
        sockets: parameter_chip
            .sockets
            .iter()
            .map(|socket| SocketOfUpdateResponseEntry {
                parameter_chip_id: socket.parameter_chip_id,
                x: socket.x,
                y: socket.y,
                version: socket.version,
            })
            .collect(),
        effects: parameter_chip
            .effects
            .iter()
            .map(|effect| EffectOfUpdateResponseEntry {
                parameter_chip_id: effect.parameter_chip_id,
                parameter_id: effect.parameter_id,
                num: effect.num,
                effect_version: effect.effect_version,
                name: effect.name.to_string(),
                display_order: effect.display_order,
                parameter_is_deleted: effect.parameter_is_deleted,
                parameter_version: effect.parameter_version,
            })
            .collect(),
    });
}

// パラメータチップ削除APIのソケット
#[derive(Serialize, Deserialize, Debug)]
struct SocketOfDeleteResponseEntry {
    parameter_chip_id: u32, // パラメータチップID
    x: u32,                 // X座標
    y: u32,                 // Y座標
    version: u32,           // ソケットバージョン
}
// パラメータチップ削除APIの効果
#[derive(Serialize, Deserialize, Debug)]
struct EffectOfDeleteResponseEntry {
    parameter_chip_id: u32,     // パラメータチップID
    parameter_id: u32,          // パラメータID
    num: Option<u32>,           // 増減値
    effect_version: u32,        // パラメータチップ効果バージョン
    name: String,               // パラメータ名
    display_order: u32,         // パラメータ表示順
    parameter_is_deleted: bool, // パラメータが削除されているかどうか
    parameter_version: u32,     // パラメータバージョン
}
// パラメータチップ削除APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct DeleteResponseEntry {
    id: u32,                                   // パラメータチップID
    name: String,                              // パラメータチップ名
    display_order: u32,                        // 表示順
    version: u32,                              // バージョン
    having_count: Option<u32>,                 // 所持数
    sockets: Vec<SocketOfDeleteResponseEntry>, // ソケット一覧
    effects: Vec<EffectOfDeleteResponseEntry>, // 効果一覧
}
// パラメータチップ削除API
#[delete("/api/v1/manager/parameter-chips/{parameter_chip_id}")]
pub async fn delete(
    web::Path(parameter_chip_id): web::Path<u32>, // パラメータチップID - パスパラメータ
) -> impl Responder {
    // リクエスト取得
    let parameter_chip_id: Option<u32> = Some(parameter_chip_id);

    // データ削除
    let parameter_chip = service::parameter_chip::delete(parameter_chip_id.unwrap());

    // レスポンス加工
    return HttpResponse::Ok().json(DeleteResponseEntry {
        id: parameter_chip.id,
        name: parameter_chip.name,
        display_order: parameter_chip.display_order,
        version: parameter_chip.version,
        having_count: parameter_chip.having_count,
        sockets: parameter_chip
            .sockets
            .iter()
            .map(|socket| SocketOfDeleteResponseEntry {
                parameter_chip_id: socket.parameter_chip_id,
                x: socket.x,
                y: socket.y,
                version: socket.version,
            })
            .collect(),
        effects: parameter_chip
            .effects
            .iter()
            .map(|effect| EffectOfDeleteResponseEntry {
                parameter_chip_id: effect.parameter_chip_id,
                parameter_id: effect.parameter_id,
                num: effect.num,
                effect_version: effect.effect_version,
                name: effect.name.to_string(),
                display_order: effect.display_order,
                parameter_is_deleted: effect.parameter_is_deleted,
                parameter_version: effect.parameter_version,
            })
            .collect(),
    });
}
