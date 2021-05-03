use diesel::deserialize::QueryableByName;
use diesel::mysql::Mysql;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::Integer;

#[derive(Debug, Queryable)]
pub struct BodyHogeInterface {
  pub body_id: i32,           // 素体ID
  pub hoge_interface_id: i32, // hogeインタフェースID
  pub name: String,           // hogeインタフェース名
  pub display_order: i32,     // 表示順
  pub version: i32,           // バージョン TODO hogeインタフェースの？素体：hogeインタフェースの？
}
impl QueryableByName<Mysql> for BodyHogeInterface {
  fn build<R: diesel::row::NamedRow<Mysql>>(row: &R) -> diesel::deserialize::Result<Self> {
    return Ok(BodyHogeInterface {
      body_id: row.get("body_id")?,
      hoge_interface_id: row.get("hoge_interface_id")?,
      name: row.get("name")?,
      display_order: row.get("display_order")?,
      version: row.get("version")?,
    });
  }
}

// 素体につくhogeインタフェース一覧取得
pub fn find_body_hoge_interfaces_list(
  _connection: &diesel::MysqlConnection, // 接続情報
  _body_id: i32,                         // 素体ID
) -> Result<Vec<BodyHogeInterface>, diesel::result::Error> {
  let result: Result<Vec<BodyHogeInterface>, diesel::result::Error> = sql_query(
    "SELECT
      bhi.body_id ,
      bhi.hoge_interface_id ,
      h.name ,
      h.display_order ,
      h.version
    FROM
      hoge_interfaces h
    INNER JOIN
      bodies_hoge_interfaces bhi
    ON
      h.id = bhi.hoge_interface_id
    AND
      bhi.body_id = ?
    AND
      is_deleted = 0
    ", 
  )
  .bind::<Integer, _>(_body_id)
  .load(_connection);
  return result;
}

// 素体につくhogeインタフェース登録
pub fn register_hoge_interfaces(
  _connection: &diesel::MysqlConnection,    // 接続情報
  _hoge_interfaces: Vec<BodyHogeInterface>, // 素体につくhogeインタフェース一覧
) -> Result<usize, diesel::result::Error> {
  if _hoge_interfaces.len() == 0 {
    return Ok(0);
  }
  let mut query = "
    INSERT INTO
      bodies_hoge_interfaces (
        body_id ,
        hoge_interface_id ,
        created_datetime ,
        updated_datetime ,
        version
      )
      VALUES 
  ".to_string();
  let mut is_first = true;
  for hoge_interface in _hoge_interfaces.iter() {
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
      hoge_interface.body_id ,
      hoge_interface.hoge_interface_id
    ).to_string();
  }
  println!("{}",query);
  let result = sql_query( query ).execute(_connection);
  return result;
}

// 素体につくhogeインタフェース削除
// TODO versionの扱い検討
pub fn delete_hoge_interfaces(
  _connection: &diesel::MysqlConnection, // 接続情報
  _id: i32,                              // 素体ID
) -> Result<usize, diesel::result::Error> {
  let result: Result<usize, diesel::result::Error> = sql_query(
    "DELETE FROM
      bodies_hoge_interfaces
    WHERE
      body_id = ?
    ",
  )
  .bind::<Integer, _>(_id)
  .execute(_connection);
  return result;
}