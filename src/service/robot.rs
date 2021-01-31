// ステータス
pub struct StatusEntry {
  pub body_id: u32,           // 素体ID
  pub parameter_id: u32,      // パラメータID
  pub num: Option<u32>,       // 増減値
  pub status_version: u32,    // ステータスバージョン
  pub name: String,           // パラメータ名
  pub display_order: u32,     // 表示順
  pub is_deleted: bool,       // 削除済みかどうか
  pub parameter_version: u32, // パラメータバージョン
}

// hogeインタフェース
pub struct HogeInterfaceEntry {
  pub body_id: u32,                        // 素体ID
  pub hoge_interface_id: u32,              // hogeインタフェースID
  pub bodies_hoge_interfaces_version: u32, // 素体：hogeインタフェースバージョン
  pub name: String,                        // hogeインタフェース名
  pub display_order: u32,                  // 表示順
  pub is_deleted: bool,                    // 削除済みかどうか
  pub hoge_interface_version: u32,         // hogeインタフェースバージョン
}

// ソケット
pub struct SocketEntry {
  pub body_id: u32,             // 素体ID
  pub x: u32,                   // X座標
  pub y: u32,                   // Y座標
  pub operator: Option<String>, // 演算子
  pub num: Option<u32>,         // 増減値
  pub version: u32,             // バージョン
}

// 素体
pub struct RobotEntry {
  pub id: u32,                                  // 素体ID
  pub name: String,                             // 素体名
  pub ruby: Option<String>,                     // ルビ
  pub flavor: Option<String>,                   // フレーバーテキスト
  pub display_order: u32,                       // 表示順
  pub is_deleted: bool,                         // 削除済みかどうか
  pub version: u32,                             // バージョン
  pub having: Option<bool>,                     // 所持しているかどうか
  pub statuses: Vec<StatusEntry>,               // ステータス
  pub hoge_interfaces: Vec<HogeInterfaceEntry>, // hogeインタフェース
  pub sockets: Vec<SocketEntry>,                // ソケット
}

// 素体一覧
pub struct RobotTemplate {
  pub total_count: u32,        // 合計数
  pub robots: Vec<RobotEntry>, // 素体一覧
}

// 素体取得
pub fn find_by_id(_id: u32, // 素体ID
) -> RobotEntry {
  // データ取得

  // データ加工
  return RobotEntry {
    id: 4,
    name: String::from("ZAKU-II"),
    ruby: Some(String::from("ザク・ツー")),
    flavor: Some(String::from("ジオンで量産されるヤツ")),
    display_order: 1,
    is_deleted: false,
    version: 0,
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
    sockets: vec![
      SocketEntry {
        body_id: 2,
        x: 4,
        y: 9,
        operator: Some(String::from("plus")),
        num: Some(2),
        version: 9,
      },
      SocketEntry {
        body_id: 2,
        x: 4,
        y: 9,
        operator: Some(String::from("plus")),
        num: Some(2),
        version: 9,
      },
      SocketEntry {
        body_id: 2,
        x: 4,
        y: 9,
        operator: Some(String::from("plus")),
        num: Some(2),
        version: 9,
      },
    ],
  };
}

// 素体一覧取得
pub fn find_list(
  _user_id: Option<u32>,      // ユーザID
  _only_having: Option<bool>, // 取得した素体のみを取得するかどうか
  _sort_by: Option<u32>,      // ソート種別
  _limit: Option<u32>,        // 取得数
  _offset: Option<u32>,       // 取得位置
) -> RobotTemplate {
  // データ取得

  // データ加工
  return RobotTemplate {
    total_count: 340,
    robots: vec![
      RobotEntry {
        id: 4,
        name: String::from("ZAKU-II"),
        ruby: Some(String::from("ザク・ツー")),
        flavor: Some(String::from("ジオンで量産されるヤツ")),
        display_order: 1,
        is_deleted: false,
        version: 0,
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
        sockets: vec![
          SocketEntry {
            body_id: 2,
            x: 4,
            y: 9,
            operator: Some(String::from("plus")),
            num: Some(2),
            version: 9,
          },
          SocketEntry {
            body_id: 2,
            x: 4,
            y: 9,
            operator: Some(String::from("plus")),
            num: Some(2),
            version: 9,
          },
          SocketEntry {
            body_id: 2,
            x: 4,
            y: 9,
            operator: Some(String::from("plus")),
            num: Some(2),
            version: 9,
          },
        ],
      },
      RobotEntry {
        id: 4,
        name: String::from("ZAKU-II"),
        ruby: Some(String::from("ザク・ツー")),
        flavor: Some(String::from("ジオンで量産されるヤツ")),
        display_order: 1,
        is_deleted: false,
        version: 0,
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
        sockets: vec![
          SocketEntry {
            body_id: 2,
            x: 4,
            y: 9,
            operator: Some(String::from("plus")),
            num: Some(2),
            version: 9,
          },
          SocketEntry {
            body_id: 2,
            x: 4,
            y: 9,
            operator: Some(String::from("plus")),
            num: Some(2),
            version: 9,
          },
          SocketEntry {
            body_id: 2,
            x: 4,
            y: 9,
            operator: Some(String::from("plus")),
            num: Some(2),
            version: 9,
          },
        ],
      },
    ],
  };
}

// 素体登録
pub fn register(
  _name: String,           // 素体名
  _ruby: Option<String>,   // ルビ
  _flavor: Option<String>, // フレーバーテキスト
  _display_order: u32,     // 表示順
) -> RobotEntry {
  // データ登録

  // データ加工
  return RobotEntry {
    id: 4,
    name: String::from("ZAKU-II"),
    ruby: Some(String::from("ザク・ツー")),
    flavor: Some(String::from("ジオンで量産されるヤツ")),
    display_order: 1,
    is_deleted: false,
    version: 0,
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
    sockets: vec![
      SocketEntry {
        body_id: 2,
        x: 4,
        y: 9,
        operator: Some(String::from("plus")),
        num: Some(2),
        version: 9,
      },
      SocketEntry {
        body_id: 2,
        x: 4,
        y: 9,
        operator: Some(String::from("plus")),
        num: Some(2),
        version: 9,
      },
      SocketEntry {
        body_id: 2,
        x: 4,
        y: 9,
        operator: Some(String::from("plus")),
        num: Some(2),
        version: 9,
      },
    ],
  };
}

// 素体更新
pub fn update(
  _id: u32,                // 素体ID
  _name: String,           // 素体名
  _ruby: Option<String>,   // ルビ
  _flavor: Option<String>, // フレーバーテキスト
  _display_order: u32,     // 表示順
  _is_deleted: bool,       // 削除済みかどうか
) -> RobotEntry {
  // データ更新

  // データ加工
  return RobotEntry {
    id: 4,
    name: String::from("ZAKU-II"),
    ruby: Some(String::from("ザク・ツー")),
    flavor: Some(String::from("ジオンで量産されるヤツ")),
    display_order: 1,
    is_deleted: false,
    version: 0,
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
    sockets: vec![
      SocketEntry {
        body_id: 2,
        x: 4,
        y: 9,
        operator: Some(String::from("plus")),
        num: Some(2),
        version: 9,
      },
      SocketEntry {
        body_id: 2,
        x: 4,
        y: 9,
        operator: Some(String::from("plus")),
        num: Some(2),
        version: 9,
      },
      SocketEntry {
        body_id: 2,
        x: 4,
        y: 9,
        operator: Some(String::from("plus")),
        num: Some(2),
        version: 9,
      },
    ],
  };
}

// 素体削除
pub fn delete(_id: u32, // 素体ID
) -> RobotEntry {
  // データ削除

  // データ加工
  return RobotEntry {
    id: 4,
    name: String::from("ZAKU-II"),
    ruby: Some(String::from("ザク・ツー")),
    flavor: Some(String::from("ジオンで量産されるヤツ")),
    display_order: 1,
    is_deleted: false,
    version: 0,
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
    sockets: vec![
      SocketEntry {
        body_id: 2,
        x: 4,
        y: 9,
        operator: Some(String::from("plus")),
        num: Some(2),
        version: 9,
      },
      SocketEntry {
        body_id: 2,
        x: 4,
        y: 9,
        operator: Some(String::from("plus")),
        num: Some(2),
        version: 9,
      },
      SocketEntry {
        body_id: 2,
        x: 4,
        y: 9,
        operator: Some(String::from("plus")),
        num: Some(2),
        version: 9,
      },
    ],
  };
}
