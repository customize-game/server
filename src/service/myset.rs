// マイセット
pub struct MysetEntry {
  pub id: u32,                     // マイセットID
  pub name: String,                // マイセット名
  pub user_id: Option<u32>,        // ユーザID
  pub body_id: u32,                // 素体ID
  pub display_order: u32,          // 表示順
  pub version: u32,                // バージョン
  pub body_name: String,           // 素体名
  pub body_ruby: Option<String>,   // 素体ルビ
  pub body_flavor: Option<String>, // 素体フレーバーテキスト
  pub body_display_order: u32,     // 素体表示順
  pub body_is_deleted: bool,       // 素体削除済みかどうか
  pub body_version: u32,           // 素体バージョン
}

// マイセット一覧
pub struct MysetTemplate {
  pub total_count: u32,        // 合計数
  pub mysets: Vec<MysetEntry>, // マイセット一覧
}

// マイセット取得
pub fn find_by_id(_id: u32, // マイセットID
) -> MysetEntry {
  // データ取得

  // データ加工
  return MysetEntry {
    id: _id,
    name: String::from("汎用装備"),
    user_id: Some(3),
    body_id: 3,
    display_order: 4,
    version: 0,
    body_name: String::from("ZAKU-II"),
    body_ruby: Some(String::from("ザク・ツー")),
    body_flavor: Some(String::from("やられ役")),
    body_display_order: 4,
    body_is_deleted: false,
    body_version: 3,
  };
}

// マイセット一覧取得
pub fn find_list(
  _user_id: Option<u32>, // ユーザID
  _sort_by: Option<u32>, // ソート種別
  _limit: Option<u32>,   // 取得数
  _offset: Option<u32>,  // 取得位置
) -> MysetTemplate {
  // データ取得

  // データ加工
  return MysetTemplate {
    total_count: 340,
    mysets: vec![
      MysetEntry {
        id: 2,
        name: String::from("汎用装備"),
        user_id: Some(3),
        body_id: 3,
        display_order: 4,
        version: 0,
        body_name: String::from("ZAKU-II"),
        body_ruby: Some(String::from("ザク・ツー")),
        body_flavor: Some(String::from("やられ役")),
        body_display_order: 4,
        body_is_deleted: false,
        body_version: 3,
      },
      MysetEntry {
        id: 3,
        name: String::from("汎用装備"),
        user_id: Some(3),
        body_id: 3,
        display_order: 4,
        version: 0,
        body_name: String::from("ZAKU-II"),
        body_ruby: Some(String::from("ザク・ツー")),
        body_flavor: Some(String::from("やられ役")),
        body_display_order: 4,
        body_is_deleted: false,
        body_version: 3,
      },
    ],
  };
}

// マイセット登録
pub fn register(
  _name: String,       // マイセット名
  _user_id: u32,       // ユーザID
  _body_id: u32,       // 素体ID
  _display_order: u32, // 表示順
) -> MysetEntry {
  // データ登録

  // データ加工
  return MysetEntry {
    id: 3,
    name: String::from("汎用装備"),
    user_id: Some(3),
    body_id: 3,
    display_order: 4,
    version: 0,
    body_name: String::from("ZAKU-II"),
    body_ruby: Some(String::from("ザク・ツー")),
    body_flavor: Some(String::from("やられ役")),
    body_display_order: 4,
    body_is_deleted: false,
    body_version: 3,
  };
}

// マイセット更新
pub fn update(
  _id: u32,            // マイセットID
  _name: String,       // マイセット名 ,
  _body_id: u32,       // 素体ID
  _display_order: u32, // 表示順
) -> MysetEntry {
  // データ更新

  // データ加工
  return MysetEntry {
    id: 3,
    name: String::from("汎用装備"),
    user_id: Some(3),
    body_id: 3,
    display_order: 4,
    version: 0,
    body_name: String::from("ZAKU-II"),
    body_ruby: Some(String::from("ザク・ツー")),
    body_flavor: Some(String::from("やられ役")),
    body_display_order: 4,
    body_is_deleted: false,
    body_version: 3,
  };
}

// マイセット削除
pub fn delete(_id: u32, // マイセットID
) -> MysetEntry {
  // データ削除

  // データ加工
  return MysetEntry {
    id: 3,
    name: String::from("汎用装備"),
    user_id: Some(3),
    body_id: 3,
    display_order: 4,
    version: 0,
    body_name: String::from("ZAKU-II"),
    body_ruby: Some(String::from("ザク・ツー")),
    body_flavor: Some(String::from("やられ役")),
    body_display_order: 4,
    body_is_deleted: false,
    body_version: 3,
  };
}
