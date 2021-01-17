pub struct RobotEntry {
  pub id: u32,
  pub name: String,
  pub ruby: String,
  pub flavor: String,
  pub display_order: u32,
  pub is_deleted: bool,
  pub version: u32,
}

pub struct RobotTemplate {
  pub total_count: u32,
  pub robots: Vec<RobotEntry>,
}

pub fn find_by_id(_id: u32) -> RobotEntry {
  return RobotEntry {
    id: 4,
    name: String::from("ZAKU-II"),
    ruby: String::from("ザク・ツー"),
    flavor: String::from("ジオンで量産されるヤツ"),
    display_order: 1,
    is_deleted: false,
    version: 0,
  };
}

pub fn find_list(
  _user_id: Option<u32>,
  _only_having: Option<bool>,
  _sort_by: Option<u32>,
  _limit: Option<u32>,
  _offset: Option<u32>,
) -> RobotTemplate {
  return RobotTemplate {
    total_count: 340,
    robots: vec![
      RobotEntry {
        id: 4,
        name: String::from("ZAKU-II"),
        ruby: String::from("ザク・ツー"),
        flavor: String::from("ジオンで量産されるヤツ"),
        display_order: 1,
        is_deleted: false,
        version: 0,
      },
      RobotEntry {
        id: 4,
        name: String::from("ZAKU-II"),
        ruby: String::from("ザク・ツー"),
        flavor: String::from("ジオンで量産されるヤツ"),
        display_order: 1,
        is_deleted: false,
        version: 0,
      },
    ],
  };
}

pub fn register(_name: String, _ruby: String, _flavor: String, _display_order: u32) -> RobotEntry {
  return RobotEntry {
    id: 4,
    name: String::from("ZAKU-II"),
    ruby: String::from("ザク・ツー"),
    flavor: String::from("ジオンで量産されるヤツ"),
    display_order: 1,
    is_deleted: false,
    version: 0,
  };
}

pub fn update(
  _id: u32,
  _name: String,
  _ruby: String,
  _flavor: String,
  _display_order: u32,
  _is_deleted: bool,
) -> RobotEntry {
  return RobotEntry {
    id: 4,
    name: String::from("ZAKU-II"),
    ruby: String::from("ザク・ツー"),
    flavor: String::from("ジオンで量産されるヤツ"),
    display_order: 1,
    is_deleted: false,
    version: 0,
  };
}

pub fn delete(_id: u32) -> RobotEntry {
  return RobotEntry {
    id: 4,
    name: String::from("ZAKU-II"),
    ruby: String::from("ザク・ツー"),
    flavor: String::from("ジオンで量産されるヤツ"),
    display_order: 1,
    is_deleted: false,
    version: 0,
  };
}
