use diesel::deserialize::QueryableByName;
use diesel::mysql::Mysql;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::Integer;

#[derive(Debug, Queryable)]
pub struct IncreasingHogeInterface {
  pub equipment_id: i32,      // 装備ID
  pub hoge_interface_id: i32, // hogeインタフェースID
  pub name: String,           // hogeインタフェース名
  pub display_order: i32,     // 表示順
  pub version: i32,           // バージョン
}
impl QueryableByName<Mysql> for IncreasingHogeInterface {
  fn build<R: diesel::row::NamedRow<Mysql>>(row: &R) -> diesel::deserialize::Result<Self> {
    return Ok(IncreasingHogeInterface {
      equipment_id: row.get("equipment_id")?,
      hoge_interface_id: row.get("hoge_interface_id")?,
      name: row.get("name")?,
      display_order: row.get("display_order")?,
      version: row.get("version")?,
    });
  }
}

// 装備すると増加するhogeインタフェース一覧取得
pub fn find_increasing_hoge_interfaces_list(
  _connection: &diesel::MysqlConnection, // 接続情報
  _equipment_id: i32,                    // 装備ID
) -> Result<Vec<IncreasingHogeInterface>, diesel::result::Error> {
  let result: Result<Vec<IncreasingHogeInterface>, diesel::result::Error> = sql_query(
    "SELECT
      ewihi.equipment_id ,
      ewihi.hoge_interface_id ,
      hi.name , 
      hi.display_order ,
      hi.version
    FROM
      hoge_interfaces hi
    INNER JOIN
      equipped_when_increasing_hoge_interfaces ewihi
    ON
      hi.id = ewihi.hoge_interface_id
    AND
      ewihi.equipment_id = ?
    AND
      hi.is_deleted = 0
    ", 
  )
  .bind::<Integer, _>(_equipment_id)
  .load(_connection);
  return result;
}

// 装備すると増加するhogeインタフェース登録
pub fn register_increasing_hoge_interfaces(
  _connection: &diesel::MysqlConnection,                     // 接続情報
  _increasing_hoge_interfaces: Vec<IncreasingHogeInterface>, // 装備すると増加するhogeインタフェース一覧
) -> Result<usize, diesel::result::Error> {
  if _increasing_hoge_interfaces.len() == 0 {
    return Ok(0);
  }
  let mut query = "
    INSERT INTO
      equipped_when_increasing_hoge_interfaces (
        equipment_id ,
        hoge_interface_id ,
        created_datetime ,
        updated_datetime ,
        version
      )
      VALUES 
  ".to_string();
  let mut is_first = true;
  for _increasing_hoge_interface in _increasing_hoge_interfaces.iter() {
    if !is_first {
      query += &", ".to_string();
    }
    else {
      is_first = false;
    }
    query += &format!( "(
      {} ,
      {} ,
      now() ,
      now() ,
      0 ) ",
      _increasing_hoge_interface.equipment_id ,
      _increasing_hoge_interface.hoge_interface_id
    ).to_string();
  }
  println!("{}",query);
  let result = sql_query( query ).execute(_connection);
  return result;
}

// 装備すると増加するhogeインタフェース削除
// TODO versionの扱い検討
pub fn delete_increasing_hoge_interfaces(
  _connection: &diesel::MysqlConnection, // 接続情報
  _id: i32,                              // 装備ID
) -> Result<usize, diesel::result::Error> {
  let result: Result<usize, diesel::result::Error> = sql_query(
    "DELETE FROM
      equipped_when_increasing_hoge_interfaces
    WHERE
      equipment_id = ?
    ",
  )
  .bind::<Integer, _>(_id)
  .execute(_connection);
  return result;
}