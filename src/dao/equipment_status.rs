use diesel::deserialize::QueryableByName;
use diesel::mysql::Mysql;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::Integer;

#[derive(Debug, Queryable)]
pub struct EquipmentStatus {
  pub equipment_id: i32,  // 装備ID
  pub parameter_id: i32,  // パラメータID
  pub name: String,       // パラメータ名
  pub display_order: i32, // 表示順
  pub num: i32,           // 増減値
  pub version: i32,       // バージョン TODO 素体ステータスの？　パラメータの？
}
impl QueryableByName<Mysql> for EquipmentStatus {
  fn build<R: diesel::row::NamedRow<Mysql>>(row: &R) -> diesel::deserialize::Result<Self> {
    return Ok(EquipmentStatus {
      equipment_id: row.get("equipment_id")?,
      parameter_id: row.get("parameter_id")?,
      name: row.get("name")?,
      display_order: row.get("display_order")?,
      num: row.get("num")?,
      version: row.get("version")?,
    });
  }
}

// 装備ステータス一覧取得
// TODO テーブル名修正 equipment_statuses
pub fn find_equipment_statuses_list(
  _connection: &diesel::MysqlConnection, // 接続情報
  _equipment_id: i32,                    // 装備ID
) -> Result<Vec<EquipmentStatus>, diesel::result::Error> {
  let result: Result<Vec<EquipmentStatus>, diesel::result::Error> = sql_query(
    "SELECT
      es.equipment_id ,
      es.parameter_id ,
      p.name ,
      p.display_order ,
      es.num ,
      p.version
    FROM
      parameters p
    INNER JOIN
      equipment_status es
    ON
      p.id = es.parameter_id
    AND
      es.equipment_id = ?
    AND
      p.is_deleted = 0
    ", 
  )
  .bind::<Integer, _>(_equipment_id)
  .load(_connection);
  return result;
}

// 装備ステータス登録
pub fn register_equipment_statuses(
  _connection: &diesel::MysqlConnection, // 接続情報
  _statuses: Vec<EquipmentStatus>,       // 素体ステータス一覧
) -> Result<usize, diesel::result::Error> {
  if _statuses.len() == 0 {
    return Ok(0);
  }
  let mut query = "
    INSERT INTO
      equipment_status (
        equipment_id ,
        parameter_id ,
        num ,
        created_datetime ,
        updated_datetime ,
        version
      )
      VALUES 
  ".to_string();
  let mut is_first = true;
  for _status in _statuses.iter() {
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
      _status.equipment_id ,
      _status.parameter_id ,
      _status.num
    ).to_string();
  }
  println!("{}",query);
  let result = sql_query( query ).execute(_connection);
  return result;
}

// 装備ステータス削除
// TODO versionの扱い検討
pub fn delete_equipment_statuses(
  _connection: &diesel::MysqlConnection, // 接続情報
  _id: i32,                              // 装備ID
) -> Result<usize, diesel::result::Error> {
  let result: Result<usize, diesel::result::Error> = sql_query(
    "DELETE FROM
      equipment_status
    WHERE
      equipment_id = ?
    ",
  )
  .bind::<Integer, _>(_id)
  .execute(_connection);
  return result;
}