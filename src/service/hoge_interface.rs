use crate::dao;

use diesel::connection::Connection;
use diesel::result::Error;

use crate::utils::establish_connection;

// hogeインタフェース
pub struct HogeInterfaceEntry {
  pub id: i32,            // hogeインタフェースID
  pub name: String,       // hogeインタフェース名
  pub display_order: i32, // 表示順
  pub version: i32,       // バージョン
}

// hogeインタフェース一覧
pub struct HogeInterfaceTemplate {
  pub total_count: usize,                         // 合計数
  pub hoge_interfaces: Vec<HogeInterfaceEntry>, // hogeインタフェース一覧
}

// hogeインタフェース取得
pub fn find_by_id(_id: i32, // hogeインタフェースID
) -> Result<HogeInterfaceEntry, Error> {
  let connection = establish_connection();
  return connection.transaction::<HogeInterfaceEntry, _, _>(|| {  

    // データ取得
    let result = dao::hoge_interface::find_by_id(&connection, _id ).unwrap();
    let hoge_interface = result.first().unwrap();

    // データ加工
    return Ok(HogeInterfaceEntry{
      id: hoge_interface.id,
      name: hoge_interface.name.to_string(),
      display_order: hoge_interface.display_order,
      version: hoge_interface.version,
    });

  });
}

// hogeインタフェース一覧取得
pub fn find_list(
  _sort_by: Option<i32>, // ソート種別
  _limit: Option<i32>,   // 取得数
  _offset: Option<i32>,  // 取得位置
) -> Result<HogeInterfaceTemplate, Error> {
  let connection = establish_connection();
  return connection.transaction::<HogeInterfaceTemplate, _, _>(|| {

    // データ取得
    let result = dao::hoge_interface::find_list(&connection).unwrap();

    // データ加工
    return Ok(HogeInterfaceTemplate{
      total_count: result.len() ,
      hoge_interfaces: result.iter().map(|hoge_interface| HogeInterfaceEntry {
        id: hoge_interface.id,
        name: hoge_interface.name.to_string(),
        display_order: hoge_interface.display_order,
        version: hoge_interface.version,
      })
      .collect(),
    });
  });
}

// hogeインタフェース登録
pub fn register(
  _name: String,       // hogeインタフェース名
  _display_order: i32, // 表示順
) -> Result<usize, Error> {
  let connection = establish_connection();
  return connection.transaction::<usize, _, _>(|| {
    // データ登録
    let result = dao::hoge_interface::register(
      &connection,
      _name,
      _display_order
    );
    // データ加工
    return Ok(result.unwrap());

  });
}

// hogeインタフェース更新
pub fn update(
  _id: i32,            // hogeインタフェースID
  _name: String,       // hogeインタフェース名
  _display_order: i32, // 表示順
  _version: i32,       // バージョン
) -> Result<usize,Error> {
  let connection = establish_connection();
  return connection.transaction::<usize, _, _>(|| {
    // データ更新
    let result = dao::hoge_interface::update(
      &connection,
      _id,
      _name,
      _display_order,
      _version
    );
    // データ加工
    return Ok(result.unwrap());
  });
}

// hogeインタフェース削除
pub fn delete(
  _id: i32,      // hogeインタフェースID
  _version: i32, // バージョン
) -> Result<usize,Error> {
  let connection = establish_connection();
  return connection.transaction::<usize, _, _>(|| {
    // データ更新
    let result = dao::hoge_interface::delete(
      &connection,
      _id,
      _version
    );
    // データ加工
    return Ok(result.unwrap());
  });
}
