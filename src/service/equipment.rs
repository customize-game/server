pub struct EquipmentEntry {
  pub id: u32,
  pub name: String,
  pub ruby: String,
  pub flavor: String,
  pub add_socket_count: u32,
  pub display_order: u32,
  pub is_deleted: bool,
  pub version: u32,
}

pub struct EquipmentTemplate {
  pub total_count: u32,
  pub equipments: Vec<EquipmentEntry>,
}

pub fn find_by_id(_id: u32) -> EquipmentEntry {
  return EquipmentEntry {
    id: _id,
    name: String::from("AK-74M"),
    ruby: String::from("エーケー-74エム"),
    flavor: String::from("よく聞くやつ"),
    add_socket_count: 3,
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
) -> EquipmentTemplate {
  return EquipmentTemplate {
    total_count: 340,
    equipments: vec![
      EquipmentEntry {
        id: 0,
        name: String::from("AK-74M"),
        ruby: String::from("エーケー-74エム"),
        flavor: String::from("よく聞くやつ"),
        add_socket_count: 3,
        display_order: 1,
        is_deleted: false,
        version: 0,
      },
      EquipmentEntry {
        id: 1,
        name: String::from("AK-74M"),
        ruby: String::from("エーケー-74エム"),
        flavor: String::from("よく聞くやつ"),
        add_socket_count: 3,
        display_order: 1,
        is_deleted: false,
        version: 0,
      },
    ],
  };
}

pub fn register(
  _name: String,
  _ruby: String,
  _flavor: String,
  _add_socket_count: u32,
  _display_order: u32,
) -> EquipmentEntry {
  return EquipmentEntry {
    id: 999,
    name: String::from("AK-74M"),
    ruby: String::from("エーケー-74エム"),
    flavor: String::from("よく聞くやつ"),
    add_socket_count: 3,
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
  _add_socket_count: u32,
  _display_order: u32,
  _is_deleted: bool,
) -> EquipmentEntry {
  return EquipmentEntry {
    id: _id,
    name: String::from("AK-74M"),
    ruby: String::from("エーケー-74エム"),
    flavor: String::from("よく聞くやつ"),
    add_socket_count: 3,
    display_order: 1,
    is_deleted: false,
    version: 0,
  };
}

pub fn delete(_id: u32) -> EquipmentEntry {
  return EquipmentEntry {
    id: _id,
    name: String::from("AK-74M"),
    ruby: String::from("エーケー-74エム"),
    flavor: String::from("よく聞くやつ"),
    add_socket_count: 3,
    display_order: 1,
    is_deleted: false,
    version: 0,
  };
}
