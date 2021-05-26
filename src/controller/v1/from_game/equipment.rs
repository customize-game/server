use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use server::service;

// ステータス
#[derive(Serialize, Deserialize, Debug)]
struct StatusOfGetOneResponseEntry {
    equipment_id: i32,  // 装備ID
    parameter_id: i32,  // パラメータID
    num: i32,           // 増減値
    name: String,       // パラメータ名
    display_order: i32, // 表示順
}
// hogeインタフェース
#[derive(Serialize, Deserialize, Debug)]
struct HogeInterfaceOfGetOneResponseEntry {
    equipment_id: i32,                                                   // 装備ID
    hoge_interface_id: i32,                                              // hogeインタフェースID
    display_order: i32,                                                  // 表示順
    unequiping_hoge_interfaces: Vec<HogeInterfaceOfGetOneResponseEntry>, // 装備すると装備できなくなるhogeインタフェース一覧
}
// 装備取得APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct GetOneResponseEntry {
    id: i32,                                                             // 装備ID
    name: String,                                                        // 装備名
    ruby: Option<String>,                                                // ルビ
    flavor: Option<String>,                                              // フレーバーテキスト
    add_socket_count: i32,                                               // 装備時に増えるソケット数
    statuses: Vec<StatusOfGetOneResponseEntry>,                          // ステータス
    increasing_hoge_interfaces: Vec<HogeInterfaceOfGetOneResponseEntry>, // 装備すると増えるhogeインタフェース一覧
    equipable_hoge_interfaces: Vec<HogeInterfaceOfGetOneResponseEntry>,  // 装備できるhogeインタフェース一覧
}
// 装備取得API
#[get("/api/v1/game/equipments/{equipment_id}")]
pub async fn get_one(
    web::Path(equipment_id): web::Path<i32>, // 装備ID - パスパラメータ
) -> impl Responder {

    let user_id = Some(3); // TODO 認証情報から取得

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
        statuses: equipment
            .statuses
            .iter()
            .map(|status| StatusOfGetOneResponseEntry {
                equipment_id: status.equipment_id,
                parameter_id: status.parameter_id,
                num: status.num,
                name: status.name.clone(),
                display_order: status.display_order,
            })
            .collect(),
        increasing_hoge_interfaces: equipment
            .increasing_hoge_interfaces
            .iter()
            .map(|hoge_interface| HogeInterfaceOfGetOneResponseEntry {
                equipment_id: hoge_interface.equipment_id,
                hoge_interface_id: hoge_interface.hoge_interface_id,
                display_order: hoge_interface.display_order,
                unequiping_hoge_interfaces: Vec::new(),
            })
            .collect(),
        equipable_hoge_interfaces: equipment
            .equipable_hoge_interfaces
            .iter()
            .map(|hoge_interface| HogeInterfaceOfGetOneResponseEntry {
                equipment_id: hoge_interface.equipment_id,
                hoge_interface_id: hoge_interface.hoge_interface_id,
                display_order: hoge_interface.display_order,
                unequiping_hoge_interfaces: hoge_interface
                    .unequiping_hoge_interfaces
                    .iter()
                    .map(
                        |unequiping_hoge_interface| HogeInterfaceOfGetOneResponseEntry {
                            equipment_id: unequiping_hoge_interface.equipment_id,
                            hoge_interface_id: unequiping_hoge_interface.hoge_interface_id,
                            display_order: unequiping_hoge_interface.display_order,
                            unequiping_hoge_interfaces: Vec::new(),
                        },
                    )
                    .collect(),
            })
            .collect(),
    });
}

// 装備一覧取得APIクエリパラメータ
#[derive(Deserialize)]
pub struct GetListRequest {
    only_having: Option<bool>, // 取得済みのみ取得するかどうか
    sort_by: Option<String>,   // ソート種別
    limit: Option<i32>,        // 取得数
    offset: Option<i32>,       // 取得位置
}
// 装備一覧取得APIレスポンスの装備
#[derive(Serialize, Deserialize, Debug)]
struct EquipmentEntryOfGetListResponseEntry {
    id: i32,              // 装備ID
    name: String,         // 装備名
    having: Option<bool>, // 取得済みかどうか
    display_order: i32,   // 表示順
}
// 装備一覧取得APIレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct GetListResponseEntry {
    total_count: usize,                                    // 合計数
    equipments: Vec<EquipmentEntryOfGetListResponseEntry>, // 装備一覧
}
// 装備一覧取得API
#[get("/api/v1/game/equipments")]
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
    let equipments = service::equipment::find_list(
        user_id, 
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
                having: Some(false),
                display_order: equipment.display_order,
            })
            .collect(),
    });
}
