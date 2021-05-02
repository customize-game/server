use diesel::deserialize::QueryableByName;
use diesel::mysql::Mysql;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::Integer;
use diesel::sql_types::Varchar;

#[derive(Debug, Queryable)]
pub struct HogeInterface {
  pub id: i32,            // hogeインタフェースID
  pub name: String,       // hogeインタフェース名
  pub display_order: i32, // 表示順
  pub version: i32,       // バージョン
}
impl QueryableByName<Mysql> for HogeInterface {
  fn build<R: diesel::row::NamedRow<Mysql>>(row: &R) -> diesel::deserialize::Result<Self> {
    return Ok(HogeInterface {
      id: row.get("id")?,
      name: row.get("name")?,
      display_order: row.get("display_order")?,
      version: row.get("version")?,
    });
  }
}
// hogeインタフェース取得
pub fn find_by_id(
  _connection: &diesel::MysqlConnection, // 接続情報
  _id: i32,                              // hogeインタフェースID
) -> Result<Vec<HogeInterface>, diesel::result::Error> {
  let result: Result<Vec<HogeInterface>, diesel::result::Error> = sql_query(
    "SELECT
      h.id ,
      h.name ,
      h.display_order,
      h.version
    FROM
      hoge_interfaces h
    WHERE
      h.is_deleted = 0
    AND
      h.id = ?
    ",
  )
  .bind::<Integer, _>(_id)
  .load(_connection);
  return result;
}

// hogeインタフェース一覧取得
// TODO SQLインジェクション可能 ORDER BYのところ書き方変える必要あり
pub fn find_list(
  _connection: &diesel::MysqlConnection, // 接続情報
  _sort_by: Option<String>,              // ソート種別
  _limit: Option<i32>,                   // 取得数
  _offset: Option<i32>,                  // 取得位置
) -> Result<Vec<HogeInterface>, diesel::result::Error> {
  let mut query = "
    SELECT
      h.id ,
      h.name ,
      h.display_order,
      h.version
    FROM
      hoge_interfaces h
    WHERE
      h.is_deleted = 0
  ".to_string();
  if let Some(s) = _sort_by {
    query += &format!(" ORDER BY h.{}", s.to_string()).to_string();
  };
  if let Some(s) = _limit {
    query += &format!(" LIMIT {}", s.to_string()).to_string();
  }
  if let Some(s) = _offset {
    query += &format!(" OFFSET {}", s.to_string()).to_string();
  }
  let result: Result<Vec<HogeInterface>, diesel::result::Error> = sql_query( query )
    .load(_connection);
  return result;
}

// hogeインタフェース登録
// 
// TODO hoge_interfaces.idにauto_incrementが必要
// alter table hoge_interfaces modify id int auto_increment;
// 
pub fn register(
  _connection: &diesel::MysqlConnection, // 接続情報
  _name: String,                         // hogeインタフェース名
  _display_order: i32,                   // 表示順
) -> Result<usize, diesel::result::Error> {
  let result: Result<usize, diesel::result::Error> = sql_query(
    "INSERT INTO
      hoge_interfaces (
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

// hogeインタフェース更新
pub fn update(
  _connection: &diesel::MysqlConnection, // 接続情報
  _id: i32,                              // hogeインタフェースID
  _name: String,                         // hogeインタフェース名
  _display_order: i32,                   // 表示順
  _version: i32,                         // バージョン
) -> Result<usize, diesel::result::Error> {
  let result: Result<usize, diesel::result::Error> = sql_query(
    "UPDATE
      hoge_interfaces
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

// hogeインタフェース削除
pub fn delete(
  _connection: &diesel::MysqlConnection, // 接続情報
  _id: i32,                              // hogeインタフェースID
  _version: i32,                         // バージョン
) -> Result<usize, diesel::result::Error> {
  let result: Result<usize, diesel::result::Error> = sql_query(
    "UPDATE
      hoge_interfaces
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