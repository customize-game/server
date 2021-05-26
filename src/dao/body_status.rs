use diesel::deserialize::QueryableByName;
use diesel::mysql::Mysql;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::Integer;

// TODO numはnull許可
#[derive(Debug, Queryable)]
pub struct BodyStatus {
  pub body_id: i32,       // 素体ID
  pub parameter_id: i32,  // パラメータID
  pub name: String,       // パラメータ名
  pub display_order: i32, // 表示順
  pub num: i32,           // 増減値
  pub version: i32,       // バージョン TODO 素体ステータスの？　パラメータの？
}
impl QueryableByName<Mysql> for BodyStatus {
  fn build<R: diesel::row::NamedRow<Mysql>>(row: &R) -> diesel::deserialize::Result<Self> {
    return Ok(BodyStatus {
      body_id: row.get("body_id")?,
      parameter_id: row.get("parameter_id")?,
      name: row.get("name")?,
      display_order: row.get("display_order")?,
      num: row.get("num")?,
      version: row.get("version")?,
    });
  }
}

// 素体ステータス一覧取得
pub fn find_body_statuses_list(
  _connection: &diesel::MysqlConnection, // 接続情報
  _body_id: i32,                         // 素体ID
) -> Result<Vec<BodyStatus>, diesel::result::Error> {
  let result: Result<Vec<BodyStatus>, diesel::result::Error> = sql_query(
    "SELECT
      bs.body_id ,
      bs.parameter_id ,
      p.name ,
      p.display_order ,
      bs.num ,
      p.version
    FROM
      parameters p
    INNER JOIN
      body_statuses bs
    ON
      p.id = bs.parameter_id
    AND
      bs.body_id = ?
    AND
      p.is_deleted = 0
    ", 
  )
  .bind::<Integer, _>(_body_id)
  .load(_connection);
  return result;
}

// 素体ステータス登録
pub fn register_body_statuses(
  _connection: &diesel::MysqlConnection, // 接続情報
  _statuses: Vec<BodyStatus>,            // 素体ステータス一覧
) -> Result<usize, diesel::result::Error> {
  if _statuses.len() == 0 {
    return Ok(0);
  }
  let mut query = "
    INSERT INTO
      body_statuses (
        body_id ,
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
      _status.body_id ,
      _status.parameter_id ,
      _status.num
    ).to_string();
  }
  println!("{}",query);
  let result = sql_query( query ).execute(_connection);
  return result;
}

// 素体ステータス削除
// TODO versionの扱い検討
pub fn delete_body_statuses(
  _connection: &diesel::MysqlConnection, // 接続情報
  _id: i32,                              // 素体ID
) -> Result<usize, diesel::result::Error> {
  let result: Result<usize, diesel::result::Error> = sql_query(
    "DELETE FROM
      body_statuses
    WHERE
      body_id = ?
    ",
  )
  .bind::<Integer, _>(_id)
  .execute(_connection);
  return result;
}