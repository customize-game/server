use diesel::deserialize::QueryableByName;
use diesel::mysql::Mysql;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::Integer;

#[derive(Debug, Queryable)]
pub struct UnequippingHogeInterface {
  pub equipment_id: i32,                  // 装備ID
  pub equipped_hoge_interface_id: i32,    // 装備したhogeインタフェースID
  pub unequipping_hoge_interface_id: i32, // 装備できなくなるhogeインタフェースID
  pub name: String,                       // 装備できなくなるhogeインタフェース名
  pub display_order: i32,                 // 表示順
  pub version: i32,                       // バージョン
}
impl QueryableByName<Mysql> for UnequippingHogeInterface {
  fn build<R: diesel::row::NamedRow<Mysql>>(row: &R) -> diesel::deserialize::Result<Self> {
    return Ok(UnequippingHogeInterface {
      equipment_id: row.get("equipment_id")?,
      equipped_hoge_interface_id: row.get("equipped_hoge_interface_id")?,
      unequipping_hoge_interface_id: row.get("unequipping_hoge_interface_id")?,
      name: row.get("name")?,
      display_order: row.get("display_order")?,
      version: row.get("version")?,
    });
  }
}

// 装備できなくなるhogeインタフェース一覧
pub fn find_unequipping_hoge_interfaces_list(
  _connection: &diesel::MysqlConnection, // 接続情報
  _equipment_id: i32,                    // 装備ID
  _equipped_hoge_interface_id: i32,      // 装備したhogeインタフェースID
) -> Result<Vec<UnequippingHogeInterface>, diesel::result::Error> {
  let result: Result<Vec<UnequippingHogeInterface>, diesel::result::Error> = sql_query(
    "SELECT
      ewuhi.equipment_id ,
      ewuhi.equipped_hoge_interface_id ,
      ewuhi.unequipping_hoge_interface_id ,
      hi.name ,
      hi.display_order ,
      hi.version
    FROM
      hoge_interfaces hi
    INNER JOIN
      equipped_when_unequipping_hoge_interfaces ewuhi
    ON
      hi.id = ewuhi.unequipping_hoge_interface_id
    AND
      ewuhi.equipment_id = ?
    AND
      ewuhi.equipped_hoge_interface_id = ?
    AND
      hi.is_deleted = 0
    ", 
  )
  .bind::<Integer, _>(_equipment_id)
  .bind::<Integer, _>(_equipped_hoge_interface_id)
  .load(_connection);
  return result;
}

// 装備できなくなるhogeインタフェース登録
pub fn register_unequipping_hoge_interfaces(
  _connection: &diesel::MysqlConnection,           // 接続情報
  _hoge_interfaces: Vec<UnequippingHogeInterface>, // 素体ステータス一覧
) -> Result<usize, diesel::result::Error> {
  if _hoge_interfaces.len() == 0 {
    return Ok(0);
  }
  let mut query = "
    INSERT INTO
      equipped_when_unequipping_hoge_interfaces (
        equipment_id ,
        equipped_hoge_interface_id ,
        unequipping_hoge_interface_id ,
        created_datetime ,
        updated_datetime ,
        version
      )
      VALUES 
  ".to_string();
  let mut is_first = true;
  for _hoge_interface in _hoge_interfaces.iter() {
    if !is_first {
      query += &", ".to_string();
    }
    else {
      is_first = false;
    }
    query += &format!( "(
      {} ,
      {} ,
      {} ,
      now() ,
      now() ,
      0 ) ",
      _hoge_interface.equipment_id ,
      _hoge_interface.equipped_hoge_interface_id ,
      _hoge_interface.unequipping_hoge_interface_id
    ).to_string();
  }
  println!("{}",query);
  let result = sql_query( query ).execute(_connection);
  return result;
}

// 装備できなくなるhogeインタフェース削除
// TODO versionの扱い検討
pub fn delete_equipable_hoge_interfaces(
  _connection: &diesel::MysqlConnection, // 接続情報
  _equipment_id: i32,                    // 装備ID
  _equipped_hoge_interface_id: i32,      // 装備ID
) -> Result<usize, diesel::result::Error> {
  let result: Result<usize, diesel::result::Error> = sql_query(
    "DELETE FROM
      equipped_when_unequipping_hoge_interfaces
    WHERE
      equipment_id = ?
    AND
      equipped_hoge_interface_id = ?
    ",
  )
  .bind::<Integer, _>(_equipment_id)
  .bind::<Integer, _>(_equipped_hoge_interface_id)
  .execute(_connection);
  return result;
}