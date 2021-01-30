// パラメータ
pub struct ParameterEntry {
  pub id: u32,            // パラメータID
  pub name: String,       // パラメータ名
  pub display_order: u32, // 表示順
  pub is_deleted: bool,   // 削除済みかどうか
  pub version: u32,       // バージョン
}

// パラメータ一覧
pub struct ParameterTemplate {
  pub total_count: u32,                // 合計数
  pub parameters: Vec<ParameterEntry>, // パラメータ一覧
}

// パラメータ取得
pub fn find_by_id(_id: u32, // パラメータID
) -> ParameterEntry {
  // データ取得

  // データ加工
  return ParameterEntry {
    id: 4,
    name: String::from("HP"),
    display_order: 1,
    is_deleted: false,
    version: 0,
  };
}

// パラメータ一覧
pub fn find_list(
  _sort_by: Option<u32>, // ソート種別
  _limit: Option<u32>,   // 取得数
  _offset: Option<u32>,  // 取得位置
) -> ParameterTemplate {
  // データ取得

  // データ加工
  return ParameterTemplate {
    total_count: 340,
    parameters: vec![
      ParameterEntry {
        id: 4,
        name: String::from("HP"),
        display_order: 1,
        is_deleted: false,
        version: 0,
      },
      ParameterEntry {
        id: 4,
        name: String::from("HP"),
        display_order: 1,
        is_deleted: false,
        version: 0,
      },
    ],
  };
}

// パラメータ登録
pub fn register(
  _name: String,       // パラメータ名
  _display_order: u32, // 表示順
) -> ParameterEntry {
  // データ登録

  // データ加工
  return ParameterEntry {
    id: 4,
    name: String::from("HP"),
    display_order: 1,
    is_deleted: false,
    version: 0,
  };
}

// パラメータ更新
pub fn update(
  _id: u32,            // パラメータID
  _name: String,       // パラメータ名
  _display_order: u32, // 表示順
  _is_deleted: bool,   // 削除済みかどうか
) -> ParameterEntry {
  // データ更新

  // データ加工
  return ParameterEntry {
    id: 4,
    name: String::from("HP"),
    display_order: 1,
    is_deleted: false,
    version: 0,
  };
}

// パラメータ削除
pub fn delete(_id: u32, // パラメータID
) -> ParameterEntry {
  // データ削除

  // データ加工
  return ParameterEntry {
    id: 4,
    name: String::from("HP"),
    display_order: 1,
    is_deleted: false,
    version: 0,
  };
}
