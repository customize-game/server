use crate::dao;

use diesel::connection::Connection;
use diesel::result::Error;

use crate::utils::establish_connection;

// ステータス
pub struct StatusEntry {
  pub body_id: i32,       // 素体ID
  pub parameter_id: i32,  // パラメータID
  pub num: i32,           // 増減値
  pub name: String,       // パラメータ名
  pub display_order: i32, // 表示順
  pub version: i32,       // パラメータバージョン
}

// hogeインタフェース
pub struct HogeInterfaceEntry {
  pub body_id: i32,           // 素体ID
  pub hoge_interface_id: i32, // hogeインタフェースID
  pub name: String,           // hogeインタフェース名
  pub display_order: i32,     // 表示順
  pub version: i32,           // hogeインタフェースバージョン
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
      statuses: dao::body_status::find_body_statuses_list( &connection, robot.id )
        .unwrap()
        .iter()
        .map(|status| StatusEntry {
          body_id: status.body_id,
          parameter_id: status.parameter_id,
          name: status.name.to_string(),
          display_order: status.display_order,
          num: status.num,
          version: status.version,
        })
        .collect(),
      hoge_interfaces: dao::body_hoge_interface::find_body_hoge_interfaces_list( &connection, robot.id )
        .unwrap()
        .iter()
        .map(|hoge_interface| HogeInterfaceEntry{
          body_id: hoge_interface.body_id,
          hoge_interface_id: hoge_interface.hoge_interface_id,
          name: hoge_interface.name.to_string(),
          display_order: hoge_interface.display_order,
          version: hoge_interface.version,
        })
        .collect(),
      sockets: dao::body_free_socket::find_free_sockets_list( &connection, robot.id )
        .unwrap()
        .iter()
        .map(|socket| SocketEntry {
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
  _sort_by: Option<String>,   // ソート種別
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
        statuses: dao::body_status::find_body_statuses_list( &connection, robot.id )
          .unwrap()
          .iter()
          .map(|status| StatusEntry {
            body_id: status.body_id,
            parameter_id: status.parameter_id,
            name: status.name.to_string(),
            display_order: status.display_order,
            num: status.num,
            version: status.version,
          })
          .collect(),
        hoge_interfaces: dao::body_hoge_interface::find_body_hoge_interfaces_list( &connection, robot.id )
          .unwrap()
          .iter()
          .map(|hoge_interface| HogeInterfaceEntry{
            body_id: hoge_interface.body_id,
            hoge_interface_id: hoge_interface.hoge_interface_id,
            name: hoge_interface.name.to_string(),
            display_order: hoge_interface.display_order,
            version: hoge_interface.version,
          })
          .collect(),
        sockets: dao::body_free_socket::find_free_sockets_list( &connection, robot.id )
          .unwrap()
          .iter()
          .map(|socket| SocketEntry {
            body_id: socket.body_id,
            x: socket.x,
            y: socket.y,
            operator: socket.operator.clone(),
            num: socket.num,
            version: socket.version,
          })
          .collect(),
      })
      .collect(),
    });
  });
}

// 空きソケット登録用Entry
pub struct RegisterSocketEntry {
  pub x: i32,                   // X座標
  pub y: i32,                   // Y座標
  pub operator: Option<String>, // 演算子
  pub num: Option<i32>,         // 増減値
}
// hogeインタフェース登録用Entry
pub struct RegisterHogeInterfaceEntry {
  pub hoge_interface_id: i32, // hogeインタフェースID
}
// ステータス登録用Entry
pub struct RegisterStatusEntry {
  pub parameter_id: i32, // パラメータID
  pub num: i32,          // 増減値
}
// 素体登録
pub fn register(
  _name: String,                                     // 素体名
  _ruby: Option<String>,                             // ルビ
  _flavor: Option<String>,                           // フレーバーテキスト
  _display_order: i32,                               // 表示順
  _sockets: Vec<RegisterSocketEntry>,                // 空きソケット一覧
  _hoge_interfaces: Vec<RegisterHogeInterfaceEntry>, // hogeインタフェース一覧
  _statuses: Vec<RegisterStatusEntry>,               // ステータス一覧
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

    let _body_id = dao::robot::get_max_id(&connection).unwrap().first().unwrap().max_id;
    println!( "body id is {}" , _body_id );

    let _socket_result = dao::body_free_socket::register_free_sockets(
      &connection,
      _sockets.iter().map(|socket| dao::body_free_socket::BodyFreeSocket{
        body_id: _body_id,
        x: socket.x,
        y: socket.y,
        operator: socket.operator.clone(),
        num: socket.num,
        version: 0, // TODO 登録Entryと取得Entryと分けたほうがいい？
      })
      .collect(),
    ).unwrap();
    println!("socket result is {}" , _socket_result );

    let _hoge_interface_result = dao::body_hoge_interface::register_hoge_interfaces(
      &connection,
      _hoge_interfaces.iter().map(|_hoge_interface| dao::body_hoge_interface::BodyHogeInterface{
        body_id: _body_id,
        hoge_interface_id: _hoge_interface.hoge_interface_id,
        name: "".to_string(), // TODO 使わない
        display_order: 0, // TODO 使わない
        version: 0, // TODO 登録Entryと取得Entryと分けたほうがいい？
      })
      .collect(),
    ).unwrap();
    println!("hoge interface result is {}" , _socket_result );
    
    let _status_result = dao::body_status::register_body_statuses(
      &connection,
      _statuses.iter().map(|_status| dao::body_status::BodyStatus{
        body_id: _body_id,
        parameter_id: _status.parameter_id,
        name: "".to_string(), // TODO 使わない
        display_order: 0, // TODO 使わない
        num: _status.num,
        version: 0, // TODO 登録Entryと取得Entryと分けたほうがいい？
      })
      .collect(),
    ).unwrap();
    println!("status result is {}" , _socket_result );

    // データ加工
    return Ok(result.unwrap());
  });
}

// 素体更新
pub fn update(
  _id: i32,                           // 素体ID
  _name: String,                      // 素体名
  _ruby: Option<String>,              // ルビ
  _flavor: Option<String>,            // フレーバーテキスト
  _display_order: i32,                // 表示順
  _version: i32,                      // バージョン
  _sockets: Vec<RegisterSocketEntry>, // 空きソケット一覧
  _hoge_interfaces: Vec<RegisterHogeInterfaceEntry>, // hogeインタフェース一覧
  _statuses: Vec<RegisterStatusEntry>,               // ステータス一覧
) -> Result<usize,Error> {
  let connection = establish_connection();
  return connection.transaction::<usize, _, _>(|| {

    let _socket_result = dao::body_free_socket::delete_free_sockets(
      &connection,
      _id ,
    ).unwrap();
    println!("delete socket result is {}" , _socket_result );
    
    let _hoge_interface_result = dao::body_hoge_interface::delete_hoge_interfaces(
      &connection,
      _id ,
    ).unwrap();
    println!("delete hoge interface result is {}" , _socket_result );
    
    let _status_result = dao::body_status::delete_body_statuses(
      &connection,
      _id ,
    ).unwrap();
    println!("delete status result is {}" , _socket_result );


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

    let _socket_result = dao::body_free_socket::register_free_sockets(
      &connection,
      _sockets.iter().map(|socket| dao::body_free_socket::BodyFreeSocket{
        body_id: _id,
        x: socket.x,
        y: socket.y,
        operator: socket.operator.clone(),
        num: socket.num,
        version: 0, // TODO 登録Entryと取得Entryと分けたほうがいい？
      })
      .collect(),
    ).unwrap();
    println!("register socket result is {}" , _socket_result );

    let _hoge_interface_result = dao::body_hoge_interface::register_hoge_interfaces(
      &connection,
      _hoge_interfaces.iter().map(|_hoge_interface| dao::body_hoge_interface::BodyHogeInterface{
        body_id: _id,
        hoge_interface_id: _hoge_interface.hoge_interface_id,
        name: "".to_string(), // TODO 使わない
        display_order: 0, // TODO 使わない
        version: 0, // TODO 登録Entryと取得Entryと分けたほうがいい？
      })
      .collect(),
    ).unwrap();
    println!("hoge interface result is {}" , _socket_result );
    
    let _status_result = dao::body_status::register_body_statuses(
      &connection,
      _statuses.iter().map(|_status| dao::body_status::BodyStatus{
        body_id: _id,
        parameter_id: _status.parameter_id,
        name: "".to_string(), // TODO 使わない
        display_order: 0, // TODO 使わない
        num: _status.num,
        version: 0, // TODO 登録Entryと取得Entryと分けたほうがいい？
      })
      .collect(),
    ).unwrap();
    println!("status result is {}" , _socket_result );

    // データ加工
    return Ok(result.unwrap());
  });
}

// 素体削除
pub fn delete(
  _id: i32,      // 素体ID
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