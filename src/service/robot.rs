use crate::dao;

use diesel::connection::Connection;
use diesel::result::Error;

use crate::utils::establish_connection;

// ステータス
pub struct StatusEntry {
  pub body_id: i32,           // 素体ID
  pub parameter_id: i32,      // パラメータID
  pub num: Option<i32>,       // 増減値
  pub status_version: i32,    // ステータスバージョン
  pub name: String,           // パラメータ名
  pub display_order: i32,     // 表示順
  pub is_deleted: bool,       // 削除済みかどうか
  pub parameter_version: i32, // パラメータバージョン
}

// hogeインタフェース
pub struct HogeInterfaceEntry {
  pub body_id: i32,                        // 素体ID
  pub hoge_interface_id: i32,              // hogeインタフェースID
  pub bodies_hoge_interfaces_version: i32, // 素体：hogeインタフェースバージョン
  pub name: String,                        // hogeインタフェース名
  pub display_order: i32,                  // 表示順
  pub is_deleted: bool,                    // 削除済みかどうか
  pub hoge_interface_version: i32,         // hogeインタフェースバージョン
}

// ソケット
pub struct SocketEntry {
  pub body_id: i32,             // 素体ID
  pub x: i32,                   // X座標
  pub y: i32,                   // Y座標
  pub operator: Option<String>, // 演算子
  pub num: Option<i32>,         // 増減値
  pub version: i32,             // バージョン
}

// 素体
pub struct RobotEntry {
  pub id: i32,                                  // 素体ID
  pub name: String,                             // 素体名
  pub ruby: Option<String>,                     // ルビ
  pub flavor: Option<String>,                   // フレーバーテキスト
  pub display_order: i32,                       // 表示順
  pub version: i32,                             // バージョン
  pub having: Option<bool>,                     // 所持しているかどうか
  pub statuses: Vec<StatusEntry>,               // ステータス
  pub hoge_interfaces: Vec<HogeInterfaceEntry>, // hogeインタフェース
  pub sockets: Vec<SocketEntry>,                // ソケット
}

// 素体一覧
pub struct RobotTemplate {
  pub total_count: usize,        // 合計数
  pub robots: Vec<RobotEntry>, // 素体一覧
}

// 素体取得
pub fn find_by_id(
  _id: i32,              // 素体ID
  _user_id: Option<i32>, // ユーザID
) -> Result<RobotEntry, Error> {
  let connection = establish_connection();
  return connection.transaction::<RobotEntry, _, _>(|| {  

    // データ取得
    let result = dao::robot::find_by_id(
      &connection, 
      _id,
      _user_id
     ).unwrap();
    let robot = result.first().unwrap();

    // データ加工
    return Ok(RobotEntry{
      id: robot.id,
      name: robot.name.to_string(),
      ruby: robot.ruby.clone(),
      flavor: robot.flavor.clone(),
      display_order: robot.display_order,
      version: robot.version,
      having: Some(true),
      statuses: vec![
        StatusEntry {
          body_id: 3,
          parameter_id: 4,
          num: Some(4),
          status_version: 2,
          name: String::from("HP"),
          display_order: 4,
          is_deleted: false,
          parameter_version: 5,
        },
        StatusEntry {
          body_id: 3,
          parameter_id: 4,
          num: Some(4),
          status_version: 2,
          name: String::from("HP"),
          display_order: 4,
          is_deleted: false,
          parameter_version: 5,
        },
      ],
      hoge_interfaces: vec![
        HogeInterfaceEntry {
          body_id: 3,
          hoge_interface_id: 2,
          bodies_hoge_interfaces_version: 3,
          name: String::from("右腕"),
          display_order: 2,
          is_deleted: false,
          hoge_interface_version: 1,
        },
        HogeInterfaceEntry {
          body_id: 3,
          hoge_interface_id: 2,
          bodies_hoge_interfaces_version: 3,
          name: String::from("右腕"),
          display_order: 2,
          is_deleted: false,
          hoge_interface_version: 1,
        },
      ],
      sockets: robot.sockets.iter().map(|socket| SocketEntry {
        body_id: socket.body_id,
        x: socket.x,
        y: socket.y,
        operator: socket.operator.clone(),
        num: socket.num,
        version: socket.version,
      })
      .collect(),
    });

  });
}

// 素体一覧取得
pub fn find_list(
  _user_id: Option<i32>,      // ユーザID
  _only_having: Option<bool>, // 取得した素体のみを取得するかどうか
  _sort_by: Option<i32>,      // ソート種別
  _limit: Option<i32>,        // 取得数
  _offset: Option<i32>,       // 取得位置
) -> Result<RobotTemplate, Error> {
  let connection = establish_connection();
  return connection.transaction::<RobotTemplate, _, _>(|| {

    // データ取得
    let result = dao::robot::find_list(
      &connection,
      _user_id,
      _only_having,
      _sort_by,
      _limit,
      _offset
    ).unwrap();

    // データ加工
    return Ok(RobotTemplate {
      total_count: result.len(),
      robots: result.iter().map(|robot| RobotEntry{
        id: robot.id,
        name: robot.name.to_string(),
        ruby: robot.ruby.clone(),
        flavor: robot.flavor.clone(),
        display_order: robot.display_order,
        version: robot.version,
        having: Some(true),
        statuses: vec![
          StatusEntry {
            body_id: 3,
            parameter_id: 4,
            num: Some(4),
            status_version: 2,
            name: String::from("HP"),
            display_order: 4,
            is_deleted: false,
            parameter_version: 5,
          },
        ],
        hoge_interfaces: vec![
          HogeInterfaceEntry {
            body_id: 3,
            hoge_interface_id: 2,
            bodies_hoge_interfaces_version: 3,
            name: String::from("右腕"),
            display_order: 2,
            is_deleted: false,
            hoge_interface_version: 1,
          },
        ],
        sockets: vec![
          SocketEntry {
            body_id: 2,
            x: 4,
            y: 9,
            operator: Some(String::from("plus")),
            num: Some(2),
            version: 9,
          },
        ],
      })
      .collect(),
    });
  });
}

// 素体登録
pub fn register(
  _name: String,           // 素体名
  _ruby: Option<String>,   // ルビ
  _flavor: Option<String>, // フレーバーテキスト
  _display_order: i32,     // 表示順
) -> Result<usize, Error> {
  let connection = establish_connection();
  return connection.transaction::<usize, _, _>(|| {
    // データ登録
    let result = dao::robot::register(
      &connection,
      _name,
      _ruby,
      _flavor,
      _display_order
    );
    // データ加工
    return Ok(result.unwrap());
  });
}

// 素体更新
pub fn update(
  _id: i32,                // 素体ID
  _name: String,           // 素体名
  _ruby: Option<String>,   // ルビ
  _flavor: Option<String>, // フレーバーテキスト
  _display_order: i32,     // 表示順
  _version: i32,           // バージョン
) -> Result<usize,Error> {
  let connection = establish_connection();
  return connection.transaction::<usize, _, _>(|| {
    // データ更新
    let result = dao::robot::update(
      &connection,
      _id,
      _name,
      _ruby,
      _flavor,
      _display_order,
      _version
    );
    // データ加工
    return Ok(result.unwrap());
  });
}

// 素体削除
pub fn delete(
  _id: i32, // 素体ID
  _version: i32, // バージョン
) -> Result<usize,Error> {
  let connection = establish_connection();
  return connection.transaction::<usize, _, _>(|| {
    // データ更新
    let result = dao::robot::delete(
      &connection,
      _id,
      _version
    );
    // データ加工
    return Ok(result.unwrap());
  });
}