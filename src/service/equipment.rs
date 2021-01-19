// ステータス
pub struct StatusEntry {
  pub equipment_id: u32,      // 装備ID
  pub parameter_id: u32,      // パラメータID
  pub num: Option<u32>,       // 増減値
  pub status_version: u32,    // 装備ステータスバージョン
  pub parameter_name: String, // パラメータ名
  pub display_order: u32,     // 表示順
  pub is_deleted: bool,       // 削除済みかどうか
  pub parameter_version: u32, // パラメータバージョン
}

// hogeインタフェース
pub struct HogeInterfaceEntry {
  pub equipment_id: u32,                                   // 装備ID
  pub hoge_interface_id: u32,                              // hogeインタフェースID
  pub display_order: u32,                                  // 表示順
  pub is_deleted: bool,                                    // 削除済みかどうか
  pub version: u32,                                        // バージョン
  pub unequiping_hoge_interfaces: Vec<HogeInterfaceEntry>, // 装備すると装備できなくなるhogeインタフェース一覧
}

// 装備
pub struct EquipmentEntry {
  pub id: u32,                                             // 装備ID
  pub name: String,                                        // 装備名
  pub ruby: Option<String>,                                // ルビ
  pub flavor: Option<String>,                              // フレーバーテキスト
  pub add_socket_count: u32,                               // 装備時に増えるソケット数
  pub display_order: u32,                                  // 表示順
  pub is_deleted: bool,                                    // 削除済みかどうか
  pub version: u32,                                        // バージョン
  pub having: Option<bool>,                                // 所持しているかどうか
  pub statuses: Vec<StatusEntry>,                          // ステータス
  pub increasing_hoge_interfaces: Vec<HogeInterfaceEntry>, // 装備すると増えるhogeインタフェース一覧
  pub equipable_hoge_interfaces: Vec<HogeInterfaceEntry>,  // 装備できるhogeインタフェース一覧
}

// 装備一覧
pub struct EquipmentTemplate {
  pub total_count: u32,                // 合計数
  pub equipments: Vec<EquipmentEntry>, // 装備一覧
}

// 装備取得
pub fn find_by_id(_id: u32, // 装備ID
) -> EquipmentEntry {
  // データ取得

  // データ加工
  return EquipmentEntry {
    id: _id,
    name: String::from("AK-74M"),
    ruby: Some(String::from("エーケー-74エム")),
    flavor: Some(String::from("よく聞くやつ")),
    add_socket_count: 3,
    display_order: 1,
    is_deleted: false,
    version: 0,
    having: Some(false),
    statuses: vec![
      StatusEntry {
        equipment_id: 3,
        parameter_id: 2,
        num: Some(3),
        status_version: 4,
        parameter_name: String::from("攻撃力"),
        display_order: 4,
        is_deleted: false,
        parameter_version: 4,
      },
      StatusEntry {
        equipment_id: 3,
        parameter_id: 2,
        num: Some(3),
        status_version: 4,
        parameter_name: String::from("攻撃力"),
        display_order: 4,
        is_deleted: false,
        parameter_version: 4,
      },
    ],
    increasing_hoge_interfaces: vec![
      HogeInterfaceEntry {
        equipment_id: 4,
        hoge_interface_id: 2,
        display_order: 9,
        is_deleted: false,
        version: 2,
        unequiping_hoge_interfaces: Vec::new(),
      },
      HogeInterfaceEntry {
        equipment_id: 4,
        hoge_interface_id: 2,
        display_order: 9,
        is_deleted: false,
        version: 2,
        unequiping_hoge_interfaces: Vec::new(),
      },
    ],
    equipable_hoge_interfaces: vec![
      HogeInterfaceEntry {
        equipment_id: 4,
        hoge_interface_id: 2,
        display_order: 9,
        is_deleted: false,
        version: 2,
        unequiping_hoge_interfaces: vec![
          HogeInterfaceEntry {
            equipment_id: 4,
            hoge_interface_id: 2,
            display_order: 9,
            is_deleted: false,
            version: 2,
            unequiping_hoge_interfaces: Vec::new(),
          },
          HogeInterfaceEntry {
            equipment_id: 4,
            hoge_interface_id: 2,
            display_order: 9,
            is_deleted: false,
            version: 2,
            unequiping_hoge_interfaces: Vec::new(),
          },
        ],
      },
      HogeInterfaceEntry {
        equipment_id: 4,
        hoge_interface_id: 2,
        display_order: 9,
        is_deleted: false,
        version: 2,
        unequiping_hoge_interfaces: vec![
          HogeInterfaceEntry {
            equipment_id: 4,
            hoge_interface_id: 2,
            display_order: 9,
            is_deleted: false,
            version: 2,
            unequiping_hoge_interfaces: Vec::new(),
          },
          HogeInterfaceEntry {
            equipment_id: 4,
            hoge_interface_id: 2,
            display_order: 9,
            is_deleted: false,
            version: 2,
            unequiping_hoge_interfaces: Vec::new(),
          },
        ],
      },
    ],
  };
}

// 装備一覧取得
pub fn find_list(
  _user_id: Option<u32>,      // ユーザID
  _only_having: Option<bool>, // 取得済みのみ取得するかどうか
  _sort_by: Option<u32>,      // ソート種別
  _limit: Option<u32>,        // 取得数
  _offset: Option<u32>,       // 取得位置
) -> EquipmentTemplate {
  // データ取得

  // データ加工
  return EquipmentTemplate {
    total_count: 340,
    equipments: vec![
      EquipmentEntry {
        id: 3,
        name: String::from("AK-74M"),
        ruby: Some(String::from("エーケー-74エム")),
        flavor: Some(String::from("よく聞くやつ")),
        add_socket_count: 3,
        display_order: 1,
        is_deleted: false,
        version: 0,
        having: Some(false),
        statuses: vec![
          StatusEntry {
            equipment_id: 3,
            parameter_id: 2,
            num: Some(3),
            status_version: 4,
            parameter_name: String::from("攻撃力"),
            display_order: 4,
            is_deleted: false,
            parameter_version: 4,
          },
          StatusEntry {
            equipment_id: 3,
            parameter_id: 2,
            num: Some(3),
            status_version: 4,
            parameter_name: String::from("攻撃力"),
            display_order: 4,
            is_deleted: false,
            parameter_version: 4,
          },
        ],
        increasing_hoge_interfaces: vec![
          HogeInterfaceEntry {
            equipment_id: 4,
            hoge_interface_id: 2,
            display_order: 9,
            is_deleted: false,
            version: 2,
            unequiping_hoge_interfaces: Vec::new(),
          },
          HogeInterfaceEntry {
            equipment_id: 4,
            hoge_interface_id: 2,
            display_order: 9,
            is_deleted: false,
            version: 2,
            unequiping_hoge_interfaces: Vec::new(),
          },
        ],
        equipable_hoge_interfaces: vec![
          HogeInterfaceEntry {
            equipment_id: 4,
            hoge_interface_id: 2,
            display_order: 9,
            is_deleted: false,
            version: 2,
            unequiping_hoge_interfaces: vec![
              HogeInterfaceEntry {
                equipment_id: 4,
                hoge_interface_id: 2,
                display_order: 9,
                is_deleted: false,
                version: 2,
                unequiping_hoge_interfaces: Vec::new(),
              },
              HogeInterfaceEntry {
                equipment_id: 4,
                hoge_interface_id: 2,
                display_order: 9,
                is_deleted: false,
                version: 2,
                unequiping_hoge_interfaces: Vec::new(),
              },
            ],
          },
          HogeInterfaceEntry {
            equipment_id: 4,
            hoge_interface_id: 2,
            display_order: 9,
            is_deleted: false,
            version: 2,
            unequiping_hoge_interfaces: vec![
              HogeInterfaceEntry {
                equipment_id: 4,
                hoge_interface_id: 2,
                display_order: 9,
                is_deleted: false,
                version: 2,
                unequiping_hoge_interfaces: Vec::new(),
              },
              HogeInterfaceEntry {
                equipment_id: 4,
                hoge_interface_id: 2,
                display_order: 9,
                is_deleted: false,
                version: 2,
                unequiping_hoge_interfaces: Vec::new(),
              },
            ],
          },
        ],
      },
      EquipmentEntry {
        id: 4,
        name: String::from("AK-74M"),
        ruby: Some(String::from("エーケー-74エム")),
        flavor: Some(String::from("よく聞くやつ")),
        add_socket_count: 3,
        display_order: 1,
        is_deleted: false,
        version: 0,
        having: Some(false),
        statuses: vec![
          StatusEntry {
            equipment_id: 3,
            parameter_id: 2,
            num: Some(3),
            status_version: 4,
            parameter_name: String::from("攻撃力"),
            display_order: 4,
            is_deleted: false,
            parameter_version: 4,
          },
          StatusEntry {
            equipment_id: 3,
            parameter_id: 2,
            num: Some(3),
            status_version: 4,
            parameter_name: String::from("攻撃力"),
            display_order: 4,
            is_deleted: false,
            parameter_version: 4,
          },
        ],
        increasing_hoge_interfaces: vec![
          HogeInterfaceEntry {
            equipment_id: 4,
            hoge_interface_id: 2,
            display_order: 9,
            is_deleted: false,
            version: 2,
            unequiping_hoge_interfaces: Vec::new(),
          },
          HogeInterfaceEntry {
            equipment_id: 4,
            hoge_interface_id: 2,
            display_order: 9,
            is_deleted: false,
            version: 2,
            unequiping_hoge_interfaces: Vec::new(),
          },
        ],
        equipable_hoge_interfaces: vec![
          HogeInterfaceEntry {
            equipment_id: 4,
            hoge_interface_id: 2,
            display_order: 9,
            is_deleted: false,
            version: 2,
            unequiping_hoge_interfaces: vec![
              HogeInterfaceEntry {
                equipment_id: 4,
                hoge_interface_id: 2,
                display_order: 9,
                is_deleted: false,
                version: 2,
                unequiping_hoge_interfaces: Vec::new(),
              },
              HogeInterfaceEntry {
                equipment_id: 4,
                hoge_interface_id: 2,
                display_order: 9,
                is_deleted: false,
                version: 2,
                unequiping_hoge_interfaces: Vec::new(),
              },
            ],
          },
          HogeInterfaceEntry {
            equipment_id: 4,
            hoge_interface_id: 2,
            display_order: 9,
            is_deleted: false,
            version: 2,
            unequiping_hoge_interfaces: vec![
              HogeInterfaceEntry {
                equipment_id: 4,
                hoge_interface_id: 2,
                display_order: 9,
                is_deleted: false,
                version: 2,
                unequiping_hoge_interfaces: Vec::new(),
              },
              HogeInterfaceEntry {
                equipment_id: 4,
                hoge_interface_id: 2,
                display_order: 9,
                is_deleted: false,
                version: 2,
                unequiping_hoge_interfaces: Vec::new(),
              },
            ],
          },
        ],
      },
    ],
  };
}

// 装備登録
pub fn register(
  _name: String,           // 装備名
  _ruby: Option<String>,   // ルビ
  _flavor: Option<String>, // フレーバーテキスト
  _add_socket_count: u32,  // 増えるソケット数
  _display_order: u32,     // 表示順
) -> EquipmentEntry {
  // データ取得

  // データ加工
  return EquipmentEntry {
    id: 2,
    name: String::from("AK-74M"),
    ruby: Some(String::from("エーケー-74エム")),
    flavor: Some(String::from("よく聞くやつ")),
    add_socket_count: 3,
    display_order: 1,
    is_deleted: false,
    version: 0,
    having: Some(false),
    statuses: vec![
      StatusEntry {
        equipment_id: 3,
        parameter_id: 2,
        num: Some(3),
        status_version: 4,
        parameter_name: String::from("攻撃力"),
        display_order: 4,
        is_deleted: false,
        parameter_version: 4,
      },
      StatusEntry {
        equipment_id: 3,
        parameter_id: 2,
        num: Some(3),
        status_version: 4,
        parameter_name: String::from("攻撃力"),
        display_order: 4,
        is_deleted: false,
        parameter_version: 4,
      },
    ],
    increasing_hoge_interfaces: vec![
      HogeInterfaceEntry {
        equipment_id: 4,
        hoge_interface_id: 2,
        display_order: 9,
        is_deleted: false,
        version: 2,
        unequiping_hoge_interfaces: Vec::new(),
      },
      HogeInterfaceEntry {
        equipment_id: 4,
        hoge_interface_id: 2,
        display_order: 9,
        is_deleted: false,
        version: 2,
        unequiping_hoge_interfaces: Vec::new(),
      },
    ],
    equipable_hoge_interfaces: vec![
      HogeInterfaceEntry {
        equipment_id: 4,
        hoge_interface_id: 2,
        display_order: 9,
        is_deleted: false,
        version: 2,
        unequiping_hoge_interfaces: vec![
          HogeInterfaceEntry {
            equipment_id: 4,
            hoge_interface_id: 2,
            display_order: 9,
            is_deleted: false,
            version: 2,
            unequiping_hoge_interfaces: Vec::new(),
          },
          HogeInterfaceEntry {
            equipment_id: 4,
            hoge_interface_id: 2,
            display_order: 9,
            is_deleted: false,
            version: 2,
            unequiping_hoge_interfaces: Vec::new(),
          },
        ],
      },
      HogeInterfaceEntry {
        equipment_id: 4,
        hoge_interface_id: 2,
        display_order: 9,
        is_deleted: false,
        version: 2,
        unequiping_hoge_interfaces: vec![
          HogeInterfaceEntry {
            equipment_id: 4,
            hoge_interface_id: 2,
            display_order: 9,
            is_deleted: false,
            version: 2,
            unequiping_hoge_interfaces: Vec::new(),
          },
          HogeInterfaceEntry {
            equipment_id: 4,
            hoge_interface_id: 2,
            display_order: 9,
            is_deleted: false,
            version: 2,
            unequiping_hoge_interfaces: Vec::new(),
          },
        ],
      },
    ],
  };
}

// 装備更新
pub fn update(
  _id: u32,                // 装備ID
  _name: String,           // 装備名
  _ruby: Option<String>,   // ルビ
  _flavor: Option<String>, // フレーバーテキスト
  _add_socket_count: u32,  // 増えるソケット数
  _display_order: u32,     // 表示順
  _is_deleted: bool,       // 削除済みかどうか
) -> EquipmentEntry {
  // データ取得

  // データ加工
  return EquipmentEntry {
    id: 5,
    name: String::from("AK-74M"),
    ruby: Some(String::from("エーケー-74エム")),
    flavor: Some(String::from("よく聞くやつ")),
    add_socket_count: 3,
    display_order: 1,
    is_deleted: false,
    version: 0,
    having: Some(false),
    statuses: vec![
      StatusEntry {
        equipment_id: 3,
        parameter_id: 2,
        num: Some(3),
        status_version: 4,
        parameter_name: String::from("攻撃力"),
        display_order: 4,
        is_deleted: false,
        parameter_version: 4,
      },
      StatusEntry {
        equipment_id: 3,
        parameter_id: 2,
        num: Some(3),
        status_version: 4,
        parameter_name: String::from("攻撃力"),
        display_order: 4,
        is_deleted: false,
        parameter_version: 4,
      },
    ],
    increasing_hoge_interfaces: vec![
      HogeInterfaceEntry {
        equipment_id: 4,
        hoge_interface_id: 2,
        display_order: 9,
        is_deleted: false,
        version: 2,
        unequiping_hoge_interfaces: Vec::new(),
      },
      HogeInterfaceEntry {
        equipment_id: 4,
        hoge_interface_id: 2,
        display_order: 9,
        is_deleted: false,
        version: 2,
        unequiping_hoge_interfaces: Vec::new(),
      },
    ],
    equipable_hoge_interfaces: vec![
      HogeInterfaceEntry {
        equipment_id: 4,
        hoge_interface_id: 2,
        display_order: 9,
        is_deleted: false,
        version: 2,
        unequiping_hoge_interfaces: vec![
          HogeInterfaceEntry {
            equipment_id: 4,
            hoge_interface_id: 2,
            display_order: 9,
            is_deleted: false,
            version: 2,
            unequiping_hoge_interfaces: Vec::new(),
          },
          HogeInterfaceEntry {
            equipment_id: 4,
            hoge_interface_id: 2,
            display_order: 9,
            is_deleted: false,
            version: 2,
            unequiping_hoge_interfaces: Vec::new(),
          },
        ],
      },
      HogeInterfaceEntry {
        equipment_id: 4,
        hoge_interface_id: 2,
        display_order: 9,
        is_deleted: false,
        version: 2,
        unequiping_hoge_interfaces: vec![
          HogeInterfaceEntry {
            equipment_id: 4,
            hoge_interface_id: 2,
            display_order: 9,
            is_deleted: false,
            version: 2,
            unequiping_hoge_interfaces: Vec::new(),
          },
          HogeInterfaceEntry {
            equipment_id: 4,
            hoge_interface_id: 2,
            display_order: 9,
            is_deleted: false,
            version: 2,
            unequiping_hoge_interfaces: Vec::new(),
          },
        ],
      },
    ],
  };
}

// 装備削除
pub fn delete(_id: u32, // 装備ID
) -> EquipmentEntry {
  // データ取得

  // データ加工
  return EquipmentEntry {
    id: 6,
    name: String::from("AK-74M"),
    ruby: Some(String::from("エーケー-74エム")),
    flavor: Some(String::from("よく聞くやつ")),
    add_socket_count: 3,
    display_order: 1,
    is_deleted: false,
    version: 0,
    having: Some(false),
    statuses: vec![
      StatusEntry {
        equipment_id: 3,
        parameter_id: 2,
        num: Some(3),
        status_version: 4,
        parameter_name: String::from("攻撃力"),
        display_order: 4,
        is_deleted: false,
        parameter_version: 4,
      },
      StatusEntry {
        equipment_id: 3,
        parameter_id: 2,
        num: Some(3),
        status_version: 4,
        parameter_name: String::from("攻撃力"),
        display_order: 4,
        is_deleted: false,
        parameter_version: 4,
      },
    ],
    increasing_hoge_interfaces: vec![
      HogeInterfaceEntry {
        equipment_id: 4,
        hoge_interface_id: 2,
        display_order: 9,
        is_deleted: false,
        version: 2,
        unequiping_hoge_interfaces: Vec::new(),
      },
      HogeInterfaceEntry {
        equipment_id: 4,
        hoge_interface_id: 2,
        display_order: 9,
        is_deleted: false,
        version: 2,
        unequiping_hoge_interfaces: Vec::new(),
      },
    ],
    equipable_hoge_interfaces: vec![
      HogeInterfaceEntry {
        equipment_id: 4,
        hoge_interface_id: 2,
        display_order: 9,
        is_deleted: false,
        version: 2,
        unequiping_hoge_interfaces: vec![
          HogeInterfaceEntry {
            equipment_id: 4,
            hoge_interface_id: 2,
            display_order: 9,
            is_deleted: false,
            version: 2,
            unequiping_hoge_interfaces: Vec::new(),
          },
          HogeInterfaceEntry {
            equipment_id: 4,
            hoge_interface_id: 2,
            display_order: 9,
            is_deleted: false,
            version: 2,
            unequiping_hoge_interfaces: Vec::new(),
          },
        ],
      },
      HogeInterfaceEntry {
        equipment_id: 4,
        hoge_interface_id: 2,
        display_order: 9,
        is_deleted: false,
        version: 2,
        unequiping_hoge_interfaces: vec![
          HogeInterfaceEntry {
            equipment_id: 4,
            hoge_interface_id: 2,
            display_order: 9,
            is_deleted: false,
            version: 2,
            unequiping_hoge_interfaces: Vec::new(),
          },
          HogeInterfaceEntry {
            equipment_id: 4,
            hoge_interface_id: 2,
            display_order: 9,
            is_deleted: false,
            version: 2,
            unequiping_hoge_interfaces: Vec::new(),
          },
        ],
      },
    ],
  };
}
