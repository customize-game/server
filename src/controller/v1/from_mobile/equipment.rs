use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use server::service;

// 装備取得APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct GetOneResponseDataEntry {
    id: u32,               // 装備ID
    name: String,          // 装備名
    ruby: String,          // ルビ
    flavor: String,        // フレーバーテキスト
    add_socket_count: u32, // 装備時に増えるソケット数
}
// 装備取得API
#[get("/api/v1/mobile/equipments/{equipment_id}")]
pub async fn get_one(
    web::Path(equipment_id): web::Path<u32>, // 装備ID - パスパラメータ
) -> impl Responder {
    // リクエスト取得
    let equipment_id: Option<u32> = Some(equipment_id);

    // データ取得
    let equipment = service::equipment::find_by_id(equipment_id.unwrap());

    // レスポンス加工
    return HttpResponse::Ok().json(GetOneResponseDataEntry {
        id: equipment.id,
        name: equipment.name,
        ruby: equipment.ruby,
        flavor: equipment.flavor,
        add_socket_count: equipment.add_socket_count,
    });
}

// 装備一覧取得APIクエリパラメータ
#[derive(Deserialize)]
pub struct GetListRequest {
    only_having: Option<bool>, // 取得済みのみ取得するかどうか
    sort_by: Option<u32>,      // ソート種別
    limit: Option<u32>,        // 取得数
    offset: Option<u32>,       // 取得位置
}
// 装備一覧取得APIレスポンスの装備
#[derive(Serialize, Deserialize, Debug)]
struct EquipmentEntryOfGetListResponseEntry {
    id: u32,              // 装備ID
    name: String,         // 装備名
    having: Option<bool>, // 取得済みかどうか
    display_order: u32,   // 表示順
}
// 装備一覧取得APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct GetListResponseEntry {
    total_count: u32,                                      // 合計数
    equipments: Vec<EquipmentEntryOfGetListResponseEntry>, // 装備一覧
}
// 装備一覧取得API
#[get("/api/v1/mobile/equipments")]
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
    let equipments = service::equipment::find_list(user_id, only_having, sort_by, limit, offset);

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
                having: Some(false),
                display_order: equipment.display_order,
            });
    }
    return HttpResponse::Ok().json(response);
}
