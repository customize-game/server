use diesel::deserialize::QueryableByName;
use diesel::mysql::Mysql;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::Integer;

#[derive(Debug, Queryable)]
pub struct EquipableHogeInterface {
  pub equipment_id: i32,      // 装備ID
  pub hoge_interface_id: i32, // hogeインタフェースID
  pub name: String,           // hogeインタフェース名
  pub display_order: i32,     // 表示順
  pub version: i32,           // バージョン
}
impl QueryableByName<Mysql> for EquipableHogeInterface {
  fn build<R: diesel::row::NamedRow<Mysql>>(row: &R) -> diesel::deserialize::Result<Self> {
    return Ok(EquipableHogeInterface {
      equipment_id: row.get("equipment_id")?,
      hoge_interface_id: row.get("hoge_interface_id")?,
      name: row.get("name")?,
      display_order: row.get("display_order")?,
      version: row.get("version")?,
    });
  }
}

// 装備できるhogeインタフェース一覧
pub fn find_equipable_hoge_interfaces_list(
  _connection: &diesel::MysqlConnection, // 接続情報
  _equipment_id: i32,                    // 装備ID
) -> Result<Vec<EquipableHogeInterface>, diesel::result::Error> {
  let result: Result<Vec<EquipableHogeInterface>, diesel::result::Error> = sql_query(
    "SELECT
      eeihi.equipment_id ,
      eeihi.hoge_interface_id ,
      hi.name ,
      hi.display_order ,
      hi.version
    FROM
      hoge_interfaces hi
    INNER JOIN
      equipments_equipable_in_hoge_interfaces eeihi
    ON
      hi.id = eeihi.hoge_interface_id
    AND
      eeihi.equipment_id = ?
    AND
      hi.is_deleted = 0
    ", 
  )
  .bind::<Integer, _>(_equipment_id)
  .load(_connection);
  return result;
}

// 装備できるhogeインタフェース登録
pub fn register_equipable_hoge_interfaces(
  _connection: &diesel::MysqlConnection,         // 接続情報
  _hoge_interfaces: Vec<EquipableHogeInterface>, // 素体ステータス一覧
) -> Result<usize, diesel::result::Error> {
  if _hoge_interfaces.len() == 0 {
    return Ok(0);
  }
  let mut query = "
    INSERT INTO
      equipments_equipable_in_hoge_interfaces (
        equipment_id ,
        hoge_interface_id ,
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
      now() ,
      now() ,
      0 ) ",
      _hoge_interface.equipment_id ,
      _hoge_interface.hoge_interface_id
    ).to_string();
  }
  println!("{}",query);
  let result = sql_query( query ).execute(_connection);
  return result;
}

// 装備できるhogeインタフェース削除
// TODO versionの扱い検討
pub fn delete_equipable_hoge_interfaces(
  _connection: &diesel::MysqlConnection, // 接続情報
  _id: i32,                              // 装備ID
) -> Result<usize, diesel::result::Error> {
  let result: Result<usize, diesel::result::Error> = sql_query(
    "DELETE FROM
      equipments_equipable_in_hoge_interfaces
    WHERE
      equipment_id = ?
    ",
  )
  .bind::<Integer, _>(_id)
  .execute(_connection);
  return result;
}