use crate::dao;

use diesel::connection::Connection;
use diesel::result::Error;

use crate::utils::establish_connection;

// ステータス
pub struct StatusEntry {
  pub equipment_id: i32,  // 装備ID
  pub parameter_id: i32,  // パラメータID
  pub num: i32,   // 増減値
  pub name: String,       // パラメータ名
  pub display_order: i32, // 表示順
  pub version: i32,       // パラメータバージョン
}

// hogeインタフェース
// TODO このEntryが共通structになってるからunequiping_hoge_interfacesが必要なくても定義する必要が出てくる
pub struct HogeInterfaceEntry {
  pub equipment_id: i32,                                   // 装備ID
  pub hoge_interface_id: i32,                              // hogeインタフェースID
  pub name: String,                                        // インタフェース名
  pub display_order: i32,                                  // 表示順
  pub version: i32,                                        // バージョン
  pub unequiping_hoge_interfaces: Vec<HogeInterfaceEntry>, // 装備すると装備できなくなるhogeインタフェース一覧
}

// 装備
pub struct EquipmentEntry {
  pub id: i32,                                             // 装備ID
  pub name: String,                                        // 装備名
  pub ruby: Option<String>,                                // ルビ
  pub flavor: Option<String>,                              // フレーバーテキスト
  pub add_socket_count: i32,                               // 装備時に増えるソケット数
  pub display_order: i32,                                  // 表示順
  pub version: i32,                                        // バージョン
  pub having: Option<bool>,                                // 所持しているかどうか
  pub statuses: Vec<StatusEntry>,                          // ステータス
  pub increasing_hoge_interfaces: Vec<HogeInterfaceEntry>, // 装備すると増えるhogeインタフェース一覧
  pub equipable_hoge_interfaces: Vec<HogeInterfaceEntry>,  // 装備できるhogeインタフェース一覧
}

// 装備一覧
pub struct EquipmentTemplate {
  pub total_count: usize,                // 合計数
  pub equipments: Vec<EquipmentEntry>, // 装備一覧
}

// 装備取得
pub fn find_by_id(
  _id: i32,              // 装備ID
  _user_id: Option<i32>, // ユーザID
) -> Result<EquipmentEntry, Error> {
  let connection = establish_connection();
  return connection.transaction::<EquipmentEntry, _, _>(|| {  

    // データ取得
    let result = dao::equipment::find_by_id(
      &connection, 
      _id,
      _user_id
     ).unwrap();
    let equipment = result.first().unwrap();

    // データ加工
    return Ok(EquipmentEntry {
      id: equipment.id,
      name: equipment.name.to_string(),
      ruby: equipment.ruby.clone(),
      flavor: equipment.flavor.clone(),
      add_socket_count: equipment.add_socket_count,
      display_order: equipment.display_order,
      version: equipment.version,
      having: Some(false),
      statuses: dao::equipment_status::find_equipment_statuses_list( &connection, equipment.id )
        .unwrap()
        .iter()
        .map(|status| StatusEntry {
          equipment_id: status.equipment_id,
          parameter_id: status.parameter_id,
          name: status.name.to_string(),
          display_order: status.display_order,
          num: status.num,
          version: status.version,
        })
        .collect(),
      increasing_hoge_interfaces: dao::increasing_hoge_interface::find_increasing_hoge_interfaces_list(
          &connection,
          equipment.id
        )
        .unwrap()
        .iter()
        .map(|hoge_interface| HogeInterfaceEntry {
          equipment_id: hoge_interface.equipment_id,
          hoge_interface_id: hoge_interface.hoge_interface_id,
          name: hoge_interface.name.to_string(),
          display_order: hoge_interface.display_order,
          version: hoge_interface.version,
          unequiping_hoge_interfaces: Vec::new(),
        })
        .collect(),
      equipable_hoge_interfaces: dao::equipable_hoge_interface::find_equipable_hoge_interfaces_list(
          &connection,
          equipment.id
        )
        .unwrap()
        .iter()
        .map(|hoge_interface| HogeInterfaceEntry {
          equipment_id: hoge_interface.equipment_id,
          hoge_interface_id: hoge_interface.hoge_interface_id,
          name: hoge_interface.name.to_string(),
          display_order: hoge_interface.display_order,
          version: hoge_interface.version,
          unequiping_hoge_interfaces: dao::unequipping_hoge_interface::find_unequipping_hoge_interfaces_list(
              &connection,
              hoge_interface.equipment_id,
              hoge_interface.hoge_interface_id
            )
            .unwrap()
            .iter()
            .map(|unequiping_hoge_interface| HogeInterfaceEntry {
              equipment_id: unequiping_hoge_interface.equipment_id,
              hoge_interface_id: unequiping_hoge_interface.unequipping_hoge_interface_id,
              name: unequiping_hoge_interface.name.to_string(),
              display_order: unequiping_hoge_interface.display_order,
              version: unequiping_hoge_interface.version,
              unequiping_hoge_interfaces: Vec::new(),
            })
            .collect(),
        })
        .collect(),
    });
  });
}

// 装備一覧取得
pub fn find_list(
  _user_id: Option<i32>,      // ユーザID
  _only_having: Option<bool>, // 取得済みのみ取得するかどうか
  _sort_by: Option<String>,   // ソート種別
  _limit: Option<i32>,        // 取得数
  _offset: Option<i32>,       // 取得位置
) -> Result<EquipmentTemplate, Error> {
  let connection = establish_connection();
  return connection.transaction::<EquipmentTemplate, _, _>(|| {  

    // データ取得
    let result = dao::equipment::find_list(
      &connection,
      _user_id,
      _only_having,
      _sort_by,
      _limit,
      _offset
    ).unwrap();

    // データ加工
    return Ok(EquipmentTemplate {
      total_count: result.len(),
      equipments: result.iter().map(|equipment| EquipmentEntry{
        id: equipment.id,
        name: equipment.name.to_string(),
        ruby: equipment.ruby.clone(),
        flavor: equipment.flavor.clone(),
        add_socket_count: equipment.add_socket_count,
        display_order: equipment.display_order,
        version: equipment.version,
        having: Some(false),
        statuses: dao::equipment_status::find_equipment_statuses_list( &connection, equipment.id )
          .unwrap()
          .iter()
          .map(|status| StatusEntry {
            equipment_id: status.equipment_id,
            parameter_id: status.parameter_id,
            name: status.name.to_string(),
            display_order: status.display_order,
            num: status.num,
            version: status.version,
          })
          .collect(),
        increasing_hoge_interfaces: dao::increasing_hoge_interface::find_increasing_hoge_interfaces_list(
            &connection,
            equipment.id
          )
          .unwrap()
          .iter()
          .map(|hoge_interface| HogeInterfaceEntry {
            equipment_id: hoge_interface.equipment_id,
            hoge_interface_id: hoge_interface.hoge_interface_id,
            name: hoge_interface.name.to_string(),
            display_order: hoge_interface.display_order,
            version: hoge_interface.version,
            unequiping_hoge_interfaces: Vec::new(),
          })
          .collect(),
        equipable_hoge_interfaces: dao::equipable_hoge_interface::find_equipable_hoge_interfaces_list(
            &connection,
            equipment.id
          )
          .unwrap()
          .iter()
          .map(|hoge_interface| HogeInterfaceEntry {
            equipment_id: hoge_interface.equipment_id,
            hoge_interface_id: hoge_interface.hoge_interface_id,
            name: hoge_interface.name.to_string(),
            display_order: hoge_interface.display_order,
            version: hoge_interface.version,
            unequiping_hoge_interfaces: dao::unequipping_hoge_interface::find_unequipping_hoge_interfaces_list(
                &connection,
                hoge_interface.equipment_id,
                hoge_interface.hoge_interface_id
              )
              .unwrap()
              .iter()
              .map(|unequiping_hoge_interface| HogeInterfaceEntry {
                equipment_id: unequiping_hoge_interface.equipment_id,
                hoge_interface_id: unequiping_hoge_interface.unequipping_hoge_interface_id,
                name: unequiping_hoge_interface.name.to_string(),
                display_order: unequiping_hoge_interface.display_order,
                version: unequiping_hoge_interface.version,
                unequiping_hoge_interfaces: Vec::new(),
              })
              .collect(),
          })
          .collect()
      })
      .collect(),
    });
  });
}

// 装備登録
pub fn register(
  _name: String,           // 装備名
  _ruby: Option<String>,   // ルビ
  _flavor: Option<String>, // フレーバーテキスト
  _add_socket_count: i32,  // 増えるソケット数
  _display_order: i32,     // 表示順
) -> Result<usize, Error> {
  let connection = establish_connection();
  return connection.transaction::<usize, _, _>(|| {  

    // データ登録
    let _result = dao::equipment::register(
      &connection,
      _name,
      _ruby,
      _flavor,
      _add_socket_count,
      _display_order,
    );

    let _body_id = dao::equipment::get_max_id(&connection).unwrap().first().unwrap().max_id;
    println!( "body id is {}" , _body_id );

    // ここ！！！！！！！！！！！！！！
    // 装備すると増えるhogeインタフェース登録
    // 装備できるhogeインタフェース登録

    // データ加工
    return Ok(_result.unwrap());
  });
}

// 装備更新
pub fn update(
  _id: i32,                // 装備ID
  _name: String,           // 装備名
  _ruby: Option<String>,   // ルビ
  _flavor: Option<String>, // フレーバーテキスト
  _add_socket_count: i32,  // 増えるソケット数
  _display_order: i32,     // 表示順
  _version: i32,           // バージョン
) -> Result<usize, Error> {
  let connection = establish_connection();
  return connection.transaction::<usize, _, _>(|| {

    // ここ！！！！！！！！！！！！！！
    // 装備すると増えるhogeインタフェース削除
    // 装備できるhogeインタフェース削除

    // データ更新
    let _result = dao::equipment::update(
      &connection,
      _id,
      _name,
      _ruby,
      _flavor,
      _add_socket_count,
      _display_order,
      _version
    );

    // ここ！！！！！！！！！！！！！！
    // 装備すると増えるhogeインタフェース登録
    // 装備できるhogeインタフェース登録

    // データ加工
    return Ok(_result.unwrap());
  });
}

// 装備削除
pub fn delete(
  _id: i32,      // 装備ID
  _version: i32, // バージョン
) -> Result<usize, Error> {
  let connection = establish_connection();
  return connection.transaction::<usize, _, _>(|| {
    // データ更新
    let _result = dao::equipment::delete(
      &connection,
      _id,
      _version
    );
    // データ加工
    return Ok(_result.unwrap());
  });
}
