use diesel::deserialize::QueryableByName;
use diesel::mysql::Mysql;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::Integer;
use diesel::sql_types::Varchar;

#[derive(Debug, Queryable)]
pub struct Parameter {
  pub id: i32,            // パラメータID
  pub name: String,       // パラメータ名
  pub display_order: i32, // 表示順
  pub version: i32,       // バージョン
}
impl QueryableByName<Mysql> for Parameter {
  fn build<R: diesel::row::NamedRow<Mysql>>(row: &R) -> diesel::deserialize::Result<Self> {
    return Ok(Parameter {
      id: row.get("id")?,
      name: row.get("name")?,
      display_order: row.get("display_order")?,
      version: row.get("version")?,
    });
  }
}
// パラメータ取得
pub fn find_by_id(
  _connection: &diesel::MysqlConnection, // 接続情報
  _id: i32,                              // パラメータID
) -> Result<Vec<Parameter>, diesel::result::Error> {
  let result: Result<Vec<Parameter>, diesel::result::Error> = sql_query(
    "SELECT
      p.id ,
      p.name ,
      p.display_order,
      p.version
    FROM
      parameters p
    WHERE
      p.is_deleted = 0
    AND
      p.id = ?
    ",
  )
  .bind::<Integer, _>(_id)
  .load(_connection);
  return result;
}

// パラメータ一覧取得
// TODO _sort_by,_limit,_offsetが使われてない
// これらOptionの定義がされてなければSQLにもLIMIT句がない　みたいなことしないといけない？
pub fn find_list(
  _connection: &diesel::MysqlConnection, // 接続情報
  _sort_by: Option<i32>,                 // ソート種別
  _limit: Option<i32>,                   // 取得数
  _offset: Option<i32>,                  // 取得位置
) -> Result<Vec<Parameter>, diesel::result::Error> {
  let result: Result<Vec<Parameter>, diesel::result::Error> = sql_query(
    "SELECT
      p.id ,
      p.name ,
      p.display_order,
      p.version
    FROM
      parameters p
    WHERE
      p.is_deleted = 0
    ",
  )
  .load(_connection);
  return result;
}

// パラメータ登録
// 
// TODO parameters.idにauto_incrementが必要
// alter table parameters modify id int auto_increment;
// 
pub fn register(
  _connection: &diesel::MysqlConnection, // 接続情報
  _name: String,                         // パラメータ名
  _display_order: i32,                   // 表示順
) -> Result<usize, diesel::result::Error> {
  let result: Result<usize, diesel::result::Error> = sql_query(
    "INSERT INTO
      parameters (
        name ,
        display_order ,
        is_deleted ,
        created_datetime ,
        updated_datetime ,
        version
      )
      VALUES(
        ? ,
        ? ,
        0 ,
        now() ,
        now() ,
        0 
      )
    ",
  )
  .bind::<Varchar, _>(_name)
  .bind::<Integer, _>(_display_order)
  .execute(_connection);
  return result;
}

// パラメータ更新
pub fn update(
  _connection: &diesel::MysqlConnection, // 接続情報
  _id: i32,                              // パラメータID
  _name: String,                         // パラメータ名
  _display_order: i32,                   // 表示順
  _version: i32,                         // バージョン
) -> Result<usize, diesel::result::Error> {
  let result: Result<usize, diesel::result::Error> = sql_query(
    "UPDATE
      parameters
    SET
      name = ? ,
      display_order = ? ,
      updated_datetime = now() ,
      version = version + 1
    WHERE
      id = ?
    AND
      version = ?
    ",
  )
  .bind::<Varchar, _>(_name)
  .bind::<Integer, _>(_display_order)
  .bind::<Integer, _>(_id)
  .bind::<Integer, _>(_version)
  .execute(_connection);
  return result;
}

// パラメータ削除
pub fn delete(
  _connection: &diesel::MysqlConnection, // 接続情報
  _id: i32,                              // パラメータID
  _version: i32,                         // バージョン
) -> Result<usize, diesel::result::Error> {
  let result: Result<usize, diesel::result::Error> = sql_query(
    "UPDATE
      parameters
    SET
      is_deleted = 1 ,
      updated_datetime = now() ,
      version = version + 1
    WHERE
      id = ?
    AND
      version = ?
    ",
  )
  .bind::<Integer, _>(_id)
  .bind::<Integer, _>(_version)
  .execute(_connection);
  return result;
}