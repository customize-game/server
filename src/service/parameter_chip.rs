pub struct ParameterChipEntry {
  pub id: u32,
  pub name: String,
  pub display_order: u32,
  pub is_deleted: bool,
  pub version: u32,
}

pub struct ParameterChipTemplate {
  pub total_count: u32,
  pub parameter_chips: Vec<ParameterChipEntry>,
}

pub fn find_by_id(_id: u32) -> ParameterChipEntry {
  return ParameterChipEntry {
    id: 3,
    name: String::from("攻撃力UP++"),
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
) -> ParameterChipTemplate {
  return ParameterChipTemplate {
    total_count: 340,
    parameter_chips: vec![
      ParameterChipEntry {
        id: 3,
        name: String::from("攻撃力UP++"),
        display_order: 1,
        is_deleted: false,
        version: 0,
      },
      ParameterChipEntry {
        id: 3,
        name: String::from("攻撃力UP++"),
        display_order: 1,
        is_deleted: false,
        version: 0,
      },
    ],
  };
}

pub fn register(_name: String, _display_order: u32) -> ParameterChipEntry {
  return ParameterChipEntry {
    id: 3,
    name: String::from("攻撃力UP++"),
    display_order: 1,
    is_deleted: false,
    version: 0,
  };
}

pub fn update(_id: u32, _name: String, _display_order: u32) -> ParameterChipEntry {
  return ParameterChipEntry {
    id: 3,
    name: String::from("攻撃力UP++"),
    display_order: 1,
    is_deleted: false,
    version: 0,
  };
}

pub fn delete(_id: u32) -> ParameterChipEntry {
  return ParameterChipEntry {
    id: 3,
    name: String::from("攻撃力UP++"),
    display_order: 1,
    is_deleted: false,
    version: 0,
  };
}
