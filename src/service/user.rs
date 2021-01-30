// ユーザ
pub struct UserEntry {
  pub id: u32,          // ユーザID
  pub exp: u32,         // 経験値
  pub is_deleted: bool, // 削除済みかどうか
  pub version: u32,     // バージョン
}

// ユーザ一覧
pub struct UserTemplate {
  pub total_count: u32,      // 合計数
  pub users: Vec<UserEntry>, // ユーザ一覧
}

// ユーザ取得
pub fn find_by_id(_id: u32, // ユーザID
) -> UserEntry {
  // データ取得

  // データ加工
  return UserEntry {
    id: 4,
    exp: 5,
    is_deleted: false,
    version: 0,
  };
}

// ユーザ一覧取得
pub fn find_list(
  _sort_by: Option<u32>, // ソート種別
  _limit: Option<u32>,   // 取得数
  _offset: Option<u32>,  // 取得位置
) -> UserTemplate {
  // データ取得

  // データ加工
  return UserTemplate {
    total_count: 340,
    users: vec![
      UserEntry {
        id: 4,
        exp: 5,
        is_deleted: false,
        version: 0,
      },
      UserEntry {
        id: 4,
        exp: 5,
        is_deleted: false,
        version: 0,
      },
    ],
  };
}

// ユーザ登録
pub fn register() -> UserEntry {
  // データ登録

  // データ加工
  return UserEntry {
    id: 4,
    exp: 5,
    is_deleted: false,
    version: 0,
  };
}

// ユーザ更新
pub fn update(
  _id: u32,          // ユーザID
  _exp: u32,         // 経験値
  _is_deleted: bool, // 削除済みかどうか
) -> UserEntry {
  // データ更新

  // データ加工
  return UserEntry {
    id: 4,
    exp: 5,
    is_deleted: false,
    version: 0,
  };
}

// ユーザ削除
pub fn delete(_id: u32, // ユーザID
) -> UserEntry {
  // データ削除

  // データ加工
  return UserEntry {
    id: 4,
    exp: 5,
    is_deleted: false,
    version: 0,
  };
}
