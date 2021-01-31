use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use server::service;

// パラメータチップ取得APIのソケット
#[derive(Serialize, Deserialize, Debug)]
struct SocketOfGetOneResponseEntry {
    parameter_chip_id: u32, // パラメータチップID
    x: u32,                 // X座標
    y: u32,                 // Y座標
}
// パラメータチップ取得APIの効果
#[derive(Serialize, Deserialize, Debug)]
struct EffectOfGetOneResponseEntry {
    parameter_chip_id: u32, // パラメータチップID
    parameter_id: u32,      // パラメータID
    num: Option<u32>,       // 増減値
    name: String,           // パラメータ名
    display_order: u32,     // パラメータ表示順
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
#[get("/api/v1/game/parameter-chips/{parameter_chip_id}")]
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
            })
            .collect(),
        effects: parameter_chip
            .effects
            .iter()
            .map(|effect| EffectOfGetOneResponseEntry {
                parameter_chip_id: effect.parameter_chip_id,
                parameter_id: effect.parameter_id,
                num: effect.num,
                name: effect.name.to_string(),
                display_order: effect.display_order,
            })
            .collect(),
    });
}

// パラメータチップ一覧取得APIクエリパラメータ
#[derive(Deserialize)]
pub struct GetListRequest {
    only_having: Option<bool>, // 取得済みのみ取得するかどうか
    sort_by: Option<u32>,      // ソート種別
    limit: Option<u32>,        // 取得数
    offset: Option<u32>,       // 取得位置
}
// パラメータチップ一覧取得APIレスポンスのパラメータチップ
#[derive(Serialize, Deserialize, Debug)]
struct ParameterChipEntryOfGetListResponseEntry {
    id: u32,                   // パラメータチップID
    name: String,              // パラメータチップ名
    display_order: u32,        // 表示順
    version: u32,              // バージョン
    having_count: Option<u32>, // 所持数
}
// パラメータチップ一覧取得APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct GetListResponseEntry {
    total_count: u32,                                               // 合計数
    parameter_chips: Vec<ParameterChipEntryOfGetListResponseEntry>, // パラメータチップ一覧
}
// パラメータチップ一覧取得API
#[get("/api/v1/game/parameter-chips")]
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
                having_count: parameter_chip.having_count,
                display_order: parameter_chip.display_order,
                version: parameter_chip.version,
            })
            .collect(),
    });
}
