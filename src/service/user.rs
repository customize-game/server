pub struct UserEntry {
  pub id: u32,
  pub exp: u32,
  pub is_deleted: bool,
  pub version: u32,
}

pub struct UserTemplate {
  pub total_count: u32,
  pub users: Vec<UserEntry>,
}

pub fn find_by_id(_id: u32) -> UserEntry {
  return UserEntry {
    id: 4,
    exp:5,
    is_deleted: false,
    version:0,
  };
}

pub fn find_list(
  _sort_by: Option<u32>,
  _limit: Option<u32>,
  _offset: Option<u32>,
) -> UserTemplate {
  return UserTemplate {
    total_count: 340,
    users: vec![
      UserEntry {
        id: 4,
        exp:5,
        is_deleted: false,
        version:0,
      },
      UserEntry {
        id: 4,
        exp:5,
        is_deleted: false,
        version:0,
      },
    ],
  };
}

pub fn register() -> UserEntry {
  return UserEntry {
    id: 4,
    exp:5,
    is_deleted: false,
    version:0,
  };
}

pub fn update(
  _id: u32,
  _exp: u32,
  _is_deleted: bool,
) -> UserEntry {
  return UserEntry {
    id: 4,
    exp:5,
    is_deleted: false,
    version:0,
  };
}

pub fn delete(_id: u32) -> UserEntry {
  return UserEntry {
    id: 4,
    exp:5,
    is_deleted: false,
    version:0,
  };
}
