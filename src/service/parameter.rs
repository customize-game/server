use crate::dao;

use diesel::connection::Connection;
use diesel::result::Error;

use crate::utils::establish_connection;

// パラメータ
pub struct ParameterEntry {
  pub id: i32,            // パラメータID
  pub name: String,       // パラメータ名
  pub display_order: i32, // 表示順
  pub version: i32,       // バージョン
}

// パラメータ一覧
pub struct ParameterTemplate {
  pub total_count: usize,                // 合計数
  pub parameters: Vec<ParameterEntry>, // パラメータ一覧
}

// パラメータ取得
pub fn find_by_id(_id: i32, // パラメータID
) -> Result<ParameterEntry, Error> {
  let connection = establish_connection();
  return connection.transaction::<ParameterEntry, _, _>(|| {
    
    // データ取得
    let result = dao::parameter::find_by_id(&connection, _id ).unwrap();
    let parameter = result.first().unwrap();

    // データ加工
    return Ok(ParameterEntry{
      id: parameter.id,
      name: parameter.name.to_string(),
      display_order: parameter.display_order,
      version: parameter.version,
    });

  });
}

// パラメータ一覧
pub fn find_list(
  _sort_by: Option<i32>, // ソート種別
  _limit: Option<i32>,   // 取得数
  _offset: Option<i32>,  // 取得位置
) -> Result<ParameterTemplate, Error> {
  let connection = establish_connection();
  return connection.transaction::<ParameterTemplate, _, _>(|| {
    
    // データ取得
    let result = dao::parameter::find_list(
      &connection,
      _sort_by,
      _limit,
      _offset
    ).unwrap();

    // データ加工
    return Ok(ParameterTemplate{
      total_count: result.len() ,
      parameters: result.iter().map(|parameter| ParameterEntry {
        id: parameter.id,
        name: parameter.name.to_string(),
        display_order: parameter.display_order,
        version: parameter.version,
      })
      .collect(),
    });

  });
}

// パラメータ登録
pub fn register(
  _name: String,       // パラメータ名
  _display_order: i32, // 表示順
) -> Result<usize, Error> {
  let connection = establish_connection();
  return connection.transaction::<usize, _, _>(|| {
    // データ登録
    let result = dao::parameter::register(
      &connection,
      _name,
      _display_order
    );
    // データ加工
    return Ok(result.unwrap());

  });
}

// パラメータ更新
pub fn update(
  _id: i32,            // パラメータID
  _name: String,       // パラメータ名
  _display_order: i32, // 表示順
  _version: i32,       // バージョン
) -> Result<usize,Error> {
  let connection = establish_connection();
  return connection.transaction::<usize, _, _>(|| {
    // データ更新
    let result = dao::parameter::update(
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

// パラメータ削除
pub fn delete(
  _id: i32,      // hogeインタフェースID
  _version: i32, // バージョン
) -> Result<usize,Error> {
  let connection = establish_connection();
  return connection.transaction::<usize, _, _>(|| {
    // データ更新
    let result = dao::parameter::delete(
      &connection,
      _id,
      _version
    );
    // データ加工
    return Ok(result.unwrap());
  });
}
