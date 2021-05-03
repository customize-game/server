use diesel::deserialize::QueryableByName;
use diesel::mysql::Mysql;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::Integer;
use diesel::sql_types::Varchar;

// ID最大値取得用Entry
// TODO もっと簡単にとれる方法はないのか・・・
#[derive(Debug, Queryable)]
pub struct MaxId {
  pub max_id: i32, // 素体ID
}
impl QueryableByName<Mysql> for MaxId {
  fn build<R: diesel::row::NamedRow<Mysql>>(row: &R) -> diesel::deserialize::Result<Self> {
    return Ok(MaxId {
      max_id: row.get("max_id")?,
    });
  }
}

#[derive(Debug, Queryable)]
pub struct Robot {
  pub id: i32,                      // 素体ID
  pub name: String,                 // 素体名
  pub ruby: Option<String>,         // 素体名ルビ
  pub flavor: Option<String>,       // フレーバーテキスト
  pub display_order: i32,           // 表示順
  pub version: i32,                 // バージョン
}
impl QueryableByName<Mysql> for Robot {
  fn build<R: diesel::row::NamedRow<Mysql>>(row: &R) -> diesel::deserialize::Result<Self> {
    return Ok(Robot {
      id: row.get("id")?,
      name: row.get("name")?,
      ruby: row.get("ruby")?,
      flavor: row.get("flavor")?,
      display_order: row.get("display_order")?,
      version: row.get("version")?,
    });
  }
}

// IDの最大値を取得
// TODO 登録時に他テーブルに登録する際にIDを取得できないといけない 何か他にいい方法はないのか
pub fn get_max_id(
  _connection: &diesel::MysqlConnection, // 接続情報
) -> Result<Vec<MaxId>, diesel::result::Error> {
  let result: Result<Vec<MaxId>, diesel::result::Error> = sql_query( 
    "SELECT
      MAX(b.id) AS max_id
    FROM
      bodies b"
  )
  .load(_connection);
  return result;
}

// 素体取得
// TODO WHERE句に分けずにON句の下にANDをつなげた方が実行計画短くなりそう・・・？
pub fn find_by_id(
  _connection: &diesel::MysqlConnection, // 接続情報
  _id: i32,                              // 素体ID
  _user_id: Option<i32>,                 // ユーザID
) -> Result<Vec<Robot>, diesel::result::Error> {
  let mut query = "
    SELECT
      b.id ,
      b.name ,
      b.ruby ,
      b.flavor ,
      b.display_order ,
      b.version
    FROM
      bodies b".to_string();
  if let Some(s) = _user_id {
    query += &format!("
      INNER JOIN 
        having_bodies hb
      ON
        b.id = hb.body_id
      AND
        hb.user_id = {}
      ", s.to_string() ).to_string()
  }
  query += "
    WHERE
      b.is_deleted = 0
    AND
      b.id = ?
  ";

  let result: Result<Vec<Robot>, diesel::result::Error> = sql_query( query )
    .bind::<Integer, _>(_id)
    .load(_connection);
  return result;
}

// 素体一覧取得
// TODO WHERE句に分けずにON句の下にANDをつなげた方が実行計画短くなりそう・・・？
// TODO 取得した素体のみの表示でない場合に、どれが取得している素体かわからない
// TODO SQLインジェクション可能 ORDER BYのところ書き方変える必要あり
pub fn find_list(
  _connection: &diesel::MysqlConnection, // 接続情報
  _user_id: Option<i32>,                 // ユーザID
  _only_having: Option<bool>,            // 取得した素体のみを取得するかどうか
  _sort_by: Option<i32>,                 // ソート種別
  _limit: Option<i32>,                   // 取得数
  _offset: Option<i32>,                  // 取得位置
) -> Result<Vec<Robot>, diesel::result::Error> {
  let mut query = "
    SELECT
      b.id ,
      b.name ,
      b.ruby ,
      b.flavor ,
      b.display_order,
      b.version
    FROM
      bodies b".to_string();
  if let Some(s) = _user_id {
    query += &format!("
      {}
        having_bodies hb
      ON
        b.id = hb.body_id
      AND
        hb.user_id = {}
      ", 
      match _only_having {
        None => " LEFT JOIN ".to_string(),
        Some(s) => if s { 
          " INNER JOIN ".to_string() 
        }
        else { 
          " LEFT JOIN ".to_string()
        }
      } ,
      s.to_string() 
    ).to_string()
  }
  query += "
    WHERE
      b.is_deleted = 0
  ";
  if let Some(s) = _sort_by {
    query += &format!(" ORDER BY h.{}", s.to_string()).to_string();
  };
  if let Some(s) = _limit {
    query += &format!(" LIMIT {}", s.to_string()).to_string();
  }
  if let Some(s) = _offset {
    query += &format!(" OFFSET {}", s.to_string()).to_string();
  }

  let result: Result<Vec<Robot>, diesel::result::Error> = sql_query( query )
    .load(_connection);
  return result;
}

// 素体登録
// 
// TODO bodies.idにauto_incrementが必要
// alter table bodies modify id int auto_increment;
// 
// TODO SQLインジェクション可能 ruby, flavorのところ書き方変える必要あり
pub fn register(
  _connection: &diesel::MysqlConnection, // 接続情報
  _name: String,                         // 素体名
  _ruby: Option<String>,                 // 素体名ルビ
  _flavor: Option<String>,               // フレーバーテキスト
  _display_order: i32,                   // 表示順
) -> Result<usize, diesel::result::Error> {
  let mut query = "
    INSERT INTO
      bodies (
        name ,
        display_order ,
        is_deleted ,
        created_datetime ,
        updated_datetime ,
        version".to_string();

  if let Some(_) = _ruby {
    query += ", ruby";
  };
  if let Some(_) = _flavor {
    query += ", flavor";
  };
  query += ") 
    VALUES ( 
      ? , 
      ? , 
      0 ,
      now() ,
      now() ,
      0";
  if let Some(s) = _ruby {
    query += &format!(", '{}'" , s.to_string()).to_string();
  };
  if let Some(s) = _flavor {
    query += &format!(", '{}'" , s.to_string()).to_string();
  };
  query += ")";

  println!("{}",query);
  let result = sql_query( query )
    .bind::<Varchar, _>(_name)
    .bind::<Integer, _>(_display_order)
    .execute(_connection);
  return result;
}

// 素体更新
pub fn update(
  _connection: &diesel::MysqlConnection, // 接続情報
  _id: i32,                              // 素体ID
  _name: String,                         // 素体名
  _ruby: Option<String>,                 // 素体名ルビ
  _flavor: Option<String>,               // フレーバーテキスト
  _display_order: i32,                   // 表示順
  _version: i32,                         // バージョン
) -> Result<usize, diesel::result::Error> {
  let mut query = "
    UPDATE
      bodies
    SET
      name = ? ,
      display_order = ? ,
      updated_datetime = now() ,
      version = version + 1".to_string();
  if let Some(s) = _ruby {
    query += &format!(" , ruby = '{}' " , s.to_string()).to_string();
  };
  if let Some(s) = _flavor {
    query += &format!(" , flavor = '{}' " , s.to_string()).to_string();
  };
  query += "
    WHERE
      id = ?
    AND
      version = ?
    ";
  let result: Result<usize, diesel::result::Error> = sql_query( query )
  .bind::<Varchar, _>(_name)
  .bind::<Integer, _>(_display_order)
  .bind::<Integer, _>(_id)
  .bind::<Integer, _>(_version)
  .execute(_connection);
  return result;
}

// 素体削除
pub fn delete(
  _connection: &diesel::MysqlConnection, // 接続情報
  _id: i32,                              // 素体ID
  _version: i32,                         // バージョン
) -> Result<usize, diesel::result::Error> {
  let result: Result<usize, diesel::result::Error> = sql_query(
    "UPDATE
      bodies
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