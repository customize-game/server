pub struct MysetEntry {
  pub id: u32,
  pub name: String,
  pub user_id: u32,
  pub body_id: u32,
  pub version: u32,
}

pub struct MysetTemplate {
  pub total_count: u32,
  pub mysets: Vec<MysetEntry>,
}

pub fn find_by_id(_id: u32) -> MysetEntry {
  return MysetEntry {
    id: _id,
    name: String::from("汎用装備"),
    user_id: 3,
    body_id: 3,
    version: 0,
  };
}

pub fn find_list(
  _user_id: Option<u32>,
  _sort_by: Option<u32>,
  _limit: Option<u32>,
  _offset: Option<u32>,
) -> MysetTemplate {
  return MysetTemplate {
    total_count: 340,
    mysets: vec![
      MysetEntry {
        id: 3,
        name: String::from("汎用装備"),
        user_id: 3,
        body_id: 3,
        version: 0,
      },
      MysetEntry {
        id: 3,
        name: String::from("汎用装備"),
        user_id: 3,
        body_id: 3,
        version: 0,
      },
    ],
  };
}

pub fn register(
  _name: String,
  _user_id: u32,
  _body_id: u32,
) -> MysetEntry {
  return MysetEntry {
    id: 3,
    name: String::from("汎用装備"),
    user_id: 3,
    body_id: 3,
    version: 0,
  };
}

pub fn update(
  _id: u32,
  _name: String,
  _body_id: u32,
) -> MysetEntry {
  return MysetEntry {
    id: 3,
    name: String::from("汎用装備"),
    user_id: 3,
    body_id: 3,
    version: 0,
  };
}

pub fn delete(_id: u32) -> MysetEntry {
  return MysetEntry {
    id: 3,
    name: String::from("汎用装備"),
    user_id: 3,
    body_id: 3,
    version: 0,
  };
}
