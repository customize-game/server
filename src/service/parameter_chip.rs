// パラメータチップの効果
pub struct EffectEntry {
  pub parameter_chip_id: u32,     // パラメータチップID
  pub parameter_id: u32,          // パラメータID
  pub num: Option<u32>,           // 増減値
  pub effect_version: u32,        // パラメータチップの効果バージョン
  pub name: String,               // パラメータ名
  pub display_order: u32,         // パラメータ表示順
  pub parameter_is_deleted: bool, // パラメータ削除済みかどうか
  pub parameter_version: u32,     // パラメータバージョン
}

// ソケット
pub struct SocketEntry {
  pub parameter_chip_id: u32, // パラメータチップID
  pub x: u32,                 // X座標
  pub y: u32,                 // Y座標
  pub version: u32,           // ソケットバージョン
}

// パラメータチップ
pub struct ParameterChipEntry {
  pub id: u32,                   // パラメータチップID
  pub name: String,              // パラメータチップ名
  pub display_order: u32,        // 表示順
  pub is_deleted: bool,          // 削除済みかどうか
  pub version: u32,              // バージョン
  pub having_count: Option<u32>,  // 所持数
  pub sockets: Vec<SocketEntry>, // ソケット一覧
  pub effects: Vec<EffectEntry>, // 効果一覧
}

// パラメータチップ一覧
pub struct ParameterChipTemplate {
  pub total_count: u32,                         // 合計数
  pub parameter_chips: Vec<ParameterChipEntry>, // パラメータチップ一覧
}

// パラメータチップ取得
pub fn find_by_id(_id: u32, // パラメータチップID
) -> ParameterChipEntry {
  // データ取得

  // データ加工
  return ParameterChipEntry {
    id: 3,
    name: String::from("攻撃力UP++"),
    display_order: 1,
    is_deleted: false,
    version: 0,
    having_count: Some(3),
    sockets: vec![
      SocketEntry {
        parameter_chip_id: 3,
        x: 3,
        y: 2,
        version: 5,
      },
      SocketEntry {
        parameter_chip_id: 3,
        x: 3,
        y: 2,
        version: 5,
      },
      SocketEntry {
        parameter_chip_id: 3,
        x: 3,
        y: 2,
        version: 5,
      },
    ],
    effects: vec![
      EffectEntry {
        parameter_chip_id: 4,
        parameter_id: 2,
        num: Some(4),
        effect_version: 2,
        name: String::from("攻撃力"),
        display_order: 4,
        parameter_is_deleted: false,
        parameter_version: 3,
      },
      EffectEntry {
        parameter_chip_id: 4,
        parameter_id: 2,
        num: Some(4),
        effect_version: 2,
        name: String::from("攻撃力"),
        display_order: 4,
        parameter_is_deleted: false,
        parameter_version: 3,
      },
      EffectEntry {
        parameter_chip_id: 4,
        parameter_id: 2,
        num: Some(4),
        effect_version: 2,
        name: String::from("攻撃力"),
        display_order: 4,
        parameter_is_deleted: false,
        parameter_version: 3,
      },
    ],
  };
}

// パラメータ一覧取得
pub fn find_list(
  _user_id: Option<u32>,      // ユーザID
  _only_having: Option<bool>, // 取得済みのみ取得するかどうか
  _sort_by: Option<u32>,      // ソート種別
  _limit: Option<u32>,        // 取得数
  _offset: Option<u32>,       // 取得位置
) -> ParameterChipTemplate {
  // データ取得

  // データ加工
  return ParameterChipTemplate {
    total_count: 340,
    parameter_chips: vec![
      ParameterChipEntry {
        id: 3,
        name: String::from("攻撃力UP++"),
        display_order: 1,
        is_deleted: false,
        version: 0,
        having_count: Some(3),
        sockets: vec![
          SocketEntry {
            parameter_chip_id: 3,
            x: 3,
            y: 2,
            version: 5,
          },
          SocketEntry {
            parameter_chip_id: 3,
            x: 3,
            y: 2,
            version: 5,
          },
          SocketEntry {
            parameter_chip_id: 3,
            x: 3,
            y: 2,
            version: 5,
          },
        ],
        effects: vec![
          EffectEntry {
            parameter_chip_id: 4,
            parameter_id: 2,
            num: Some(4),
            effect_version: 2,
            name: String::from("攻撃力"),
            display_order: 4,
            parameter_is_deleted: false,
            parameter_version: 3,
          },
          EffectEntry {
            parameter_chip_id: 4,
            parameter_id: 2,
            num: Some(4),
            effect_version: 2,
            name: String::from("攻撃力"),
            display_order: 4,
            parameter_is_deleted: false,
            parameter_version: 3,
          },
          EffectEntry {
            parameter_chip_id: 4,
            parameter_id: 2,
            num: Some(4),
            effect_version: 2,
            name: String::from("攻撃力"),
            display_order: 4,
            parameter_is_deleted: false,
            parameter_version: 3,
          },
        ],
      },
      ParameterChipEntry {
        id: 3,
        name: String::from("攻撃力UP++"),
        display_order: 1,
        is_deleted: false,
        version: 0,
        having_count: Some(3),
        sockets: vec![
          SocketEntry {
            parameter_chip_id: 3,
            x: 3,
            y: 2,
            version: 5,
          },
          SocketEntry {
            parameter_chip_id: 3,
            x: 3,
            y: 2,
            version: 5,
          },
          SocketEntry {
            parameter_chip_id: 3,
            x: 3,
            y: 2,
            version: 5,
          },
        ],
        effects: vec![
          EffectEntry {
            parameter_chip_id: 4,
            parameter_id: 2,
            num: Some(4),
            effect_version: 2,
            name: String::from("攻撃力"),
            display_order: 4,
            parameter_is_deleted: false,
            parameter_version: 3,
          },
          EffectEntry {
            parameter_chip_id: 4,
            parameter_id: 2,
            num: Some(4),
            effect_version: 2,
            name: String::from("攻撃力"),
            display_order: 4,
            parameter_is_deleted: false,
            parameter_version: 3,
          },
          EffectEntry {
            parameter_chip_id: 4,
            parameter_id: 2,
            num: Some(4),
            effect_version: 2,
            name: String::from("攻撃力"),
            display_order: 4,
            parameter_is_deleted: false,
            parameter_version: 3,
          },
        ],
      },
    ],
  };
}

// パラメータチップ登録
pub fn register(
  _name: String,       // パラメータチップ名
  _display_order: u32, // 表示順
) -> ParameterChipEntry {
  // データ登録

  // データ加工
  return ParameterChipEntry {
    id: 3,
    name: String::from("攻撃力UP++"),
    display_order: 1,
    is_deleted: false,
    version: 0,
    having_count: Some(3),
    sockets: vec![
      SocketEntry {
        parameter_chip_id: 3,
        x: 3,
        y: 2,
        version: 5,
      },
      SocketEntry {
        parameter_chip_id: 3,
        x: 3,
        y: 2,
        version: 5,
      },
      SocketEntry {
        parameter_chip_id: 3,
        x: 3,
        y: 2,
        version: 5,
      },
    ],
    effects: vec![
      EffectEntry {
        parameter_chip_id: 4,
        parameter_id: 2,
        num: Some(4),
        effect_version: 2,
        name: String::from("攻撃力"),
        display_order: 4,
        parameter_is_deleted: false,
        parameter_version: 3,
      },
      EffectEntry {
        parameter_chip_id: 4,
        parameter_id: 2,
        num: Some(4),
        effect_version: 2,
        name: String::from("攻撃力"),
        display_order: 4,
        parameter_is_deleted: false,
        parameter_version: 3,
      },
      EffectEntry {
        parameter_chip_id: 4,
        parameter_id: 2,
        num: Some(4),
        effect_version: 2,
        name: String::from("攻撃力"),
        display_order: 4,
        parameter_is_deleted: false,
        parameter_version: 3,
      },
    ],
  };
}

// パラメータ更新
pub fn update(
  _id: u32,            // パラメータチップID
  _name: String,       // パラメータチップ名
  _display_order: u32, // 表示順
  _is_deleted: bool,   // 削除済みかどうか
) -> ParameterChipEntry {
  // データ更新
  // データ加工
  return ParameterChipEntry {
    id: 3,
    name: String::from("攻撃力UP++"),
    display_order: 1,
    is_deleted: false,
    version: 0,
    having_count: Some(3),
    sockets: vec![
      SocketEntry {
        parameter_chip_id: 3,
        x: 3,
        y: 2,
        version: 5,
      },
      SocketEntry {
        parameter_chip_id: 3,
        x: 3,
        y: 2,
        version: 5,
      },
      SocketEntry {
        parameter_chip_id: 3,
        x: 3,
        y: 2,
        version: 5,
      },
    ],
    effects: vec![
      EffectEntry {
        parameter_chip_id: 4,
        parameter_id: 2,
        num: Some(4),
        effect_version: 2,
        name: String::from("攻撃力"),
        display_order: 4,
        parameter_is_deleted: false,
        parameter_version: 3,
      },
      EffectEntry {
        parameter_chip_id: 4,
        parameter_id: 2,
        num: Some(4),
        effect_version: 2,
        name: String::from("攻撃力"),
        display_order: 4,
        parameter_is_deleted: false,
        parameter_version: 3,
      },
      EffectEntry {
        parameter_chip_id: 4,
        parameter_id: 2,
        num: Some(4),
        effect_version: 2,
        name: String::from("攻撃力"),
        display_order: 4,
        parameter_is_deleted: false,
        parameter_version: 3,
      },
    ],
  };
}

// パラメータチップ削除
pub fn delete(_id: u32, // パラメータチップID
) -> ParameterChipEntry {
  // データ削除

  // データ加工
  return ParameterChipEntry {
    id: 3,
    name: String::from("攻撃力UP++"),
    display_order: 1,
    is_deleted: false,
    version: 0,
    having_count: Some(3),
    sockets: vec![
      SocketEntry {
        parameter_chip_id: 3,
        x: 3,
        y: 2,
        version: 5,
      },
      SocketEntry {
        parameter_chip_id: 3,
        x: 3,
        y: 2,
        version: 5,
      },
      SocketEntry {
        parameter_chip_id: 3,
        x: 3,
        y: 2,
        version: 5,
      },
    ],
    effects: vec![
      EffectEntry {
        parameter_chip_id: 4,
        parameter_id: 2,
        num: Some(4),
        effect_version: 2,
        name: String::from("攻撃力"),
        display_order: 4,
        parameter_is_deleted: false,
        parameter_version: 3,
      },
      EffectEntry {
        parameter_chip_id: 4,
        parameter_id: 2,
        num: Some(4),
        effect_version: 2,
        name: String::from("攻撃力"),
        display_order: 4,
        parameter_is_deleted: false,
        parameter_version: 3,
      },
      EffectEntry {
        parameter_chip_id: 4,
        parameter_id: 2,
        num: Some(4),
        effect_version: 2,
        name: String::from("攻撃力"),
        display_order: 4,
        parameter_is_deleted: false,
        parameter_version: 3,
      },
    ],
  };
}
