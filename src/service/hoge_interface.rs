// hogeインタフェース
pub struct HogeInterfaceEntry {
  pub id: u32,            // hogeインタフェースID
  pub name: String,       // hogeインタフェース名
  pub display_order: u32, // 表示順
  pub is_deleted: bool,   // 削除済みかどうか
  pub version: u32,       // バージョン
}

// hogeインタフェース一覧
pub struct HogeInterfaceTemplate {
  pub total_count: u32,                         // 合計数
  pub hoge_interfaces: Vec<HogeInterfaceEntry>, // hogeインタフェース一覧
}

// hogeインタフェース取得
pub fn find_by_id(_id: u32, // hogeインタフェースID
) -> HogeInterfaceEntry {
  // データ取得

  // データ加工
  return HogeInterfaceEntry {
    id: _id,
    name: String::from("右手"),
    display_order: 1,
    is_deleted: false,
    version: 0,
  };
}

// hogeインタフェース一覧取得
pub fn find_list(
  _sort_by: Option<u32>, // ソート種別
  _limit: Option<u32>,   // 取得数
  _offset: Option<u32>,  // 取得位置
) -> HogeInterfaceTemplate {
  // データ取得

  // データ加工
  return HogeInterfaceTemplate {
    total_count: 340,
    hoge_interfaces: vec![
      HogeInterfaceEntry {
        id: 1,
        name: String::from("右手"),
        display_order: 1,
        is_deleted: false,
        version: 0,
      },
      HogeInterfaceEntry {
        id: 2,
        name: String::from("右手"),
        display_order: 1,
        is_deleted: false,
        version: 0,
      },
    ],
  };
}

// hogeインタフェース登録
pub fn register(
  _name: String,       // hogeインタフェース名
  _display_order: u32, // 表示順
) -> HogeInterfaceEntry {
  // データ登録

  // データ加工
  return HogeInterfaceEntry {
    id: 2,
    name: String::from("右手"),
    display_order: 1,
    is_deleted: false,
    version: 0,
  };
}

// hogeインタフェース更新
pub fn update(
  _id: u32,            // hogeインタフェースID
  _name: String,       // hogeインタフェース名
  _display_order: u32, // 表示順
  _is_deleted: bool,   // 削除済みかどうか
) -> HogeInterfaceEntry {
  // データ更新

  // データ加工
  return HogeInterfaceEntry {
    id: 2,
    name: String::from("右手"),
    display_order: 1,
    is_deleted: false,
    version: 0,
  };
}

// hogeインタフェース削除
pub fn delete(_id: u32, // hogeインタフェースID
) -> HogeInterfaceEntry {
  return HogeInterfaceEntry {
    id: 2,
    name: String::from("右手"),
    display_order: 1,
    is_deleted: false,
    version: 0,
  };
}
