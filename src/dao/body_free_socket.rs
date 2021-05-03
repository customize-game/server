use diesel::deserialize::QueryableByName;
use diesel::mysql::Mysql;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::Integer;

#[derive(Debug, Queryable)]
pub struct BodyFreeSocket {
  pub body_id: i32,             // 素体ID
  pub x: i32,                   // X座標
  pub y: i32,                   // Y座標
  pub operator: Option<String>, // 演算子
  pub num: Option<i32>,         // 増減値
  pub version: i32,             // バージョン
}
impl QueryableByName<Mysql> for BodyFreeSocket {
  fn build<R: diesel::row::NamedRow<Mysql>>(row: &R) -> diesel::deserialize::Result<Self> {
    return Ok(BodyFreeSocket {
      body_id: row.get("body_id")?,
      x: row.get("x")?,
      y: row.get("y")?,
      operator: row.get("operator")?,
      num: row.get("num")?,
      version: row.get("version")?,
    });
  }
}

// 空きソケット一覧取得
pub fn find_free_sockets_list(
  _connection: &diesel::MysqlConnection, // 接続情報
  _body_id: i32,                         // 素体ID
) -> Result<Vec<BodyFreeSocket>, diesel::result::Error> {
  let result: Result<Vec<BodyFreeSocket>, diesel::result::Error> = sql_query(
    "SELECT
      bfs.body_id ,
      bfs.x ,
      bfs.y ,
      bfs.operator ,
      bfs.num ,
      bfs.version
    FROM
      body_free_sockets bfs
    WHERE
      bfs.body_id = ?
    ", 
  )
  .bind::<Integer, _>(_body_id)
  .load(_connection);
  return result;
}

// 空きソケット登録
pub fn register_free_sockets(
  _connection: &diesel::MysqlConnection, // 接続情報
  _sockets: Vec<BodyFreeSocket>,         // ソケット一覧
) -> Result<usize, diesel::result::Error> {
  if _sockets.len() == 0 {
    return Ok(0);
  }
  let mut query = "
    INSERT INTO
      body_free_sockets (
        body_id ,
        x ,
        y ,
        created_datetime ,
        updated_datetime ,
        version ,
        operator ,
        num
      )
      VALUES 
  ".to_string();
  let mut is_first = true;
  for socket in _sockets.iter() {
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
      0 , ",
      socket.body_id ,
      socket.x ,
      socket.y
    ).to_string();
    match socket.operator.clone() {
      Some(o) => query += &format!( "'{}' , " , o.to_string() ).to_string() ,
      None => query += &"NULL , ".to_string() ,
    }
    match socket.num {
      Some(n) => query += &format!( "{}  " , n.to_string() ).to_string() ,
      None => query += &"NULL  ".to_string() ,
    }
    query += &" ) ".to_string();
  }
  println!("{}",query);
  let result = sql_query( query ).execute(_connection);
  return result;
}

// 空きソケット削除
// TODO versionの扱い検討
pub fn delete_free_sockets(
  _connection: &diesel::MysqlConnection, // 接続情報
  _id: i32,                              // 素体ID
) -> Result<usize, diesel::result::Error> {
  let result: Result<usize, diesel::result::Error> = sql_query(
    "DELETE FROM
      body_free_sockets
    WHERE
      body_id = ?
    ",
  )
  .bind::<Integer, _>(_id)
  .execute(_connection);
  return result;
}