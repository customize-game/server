pub struct ParameterEntry {
  pub id: u32,
  pub name: String,
  pub display_order: u32,
  pub is_deleted: bool,
  pub version: u32,
}

pub struct ParameterTemplate {
  pub total_count: u32,
  pub parameters: Vec<ParameterEntry>,
}

pub fn find_by_id(_id: u32) -> ParameterEntry {
  return ParameterEntry {
    id: 4,
    name: String::from("HP"),
    display_order: 1,
    is_deleted: false,
    version: 0,
  };
}

pub fn find_list(
  _sort_by: Option<u32>,
  _limit: Option<u32>,
  _offset: Option<u32>,
) -> ParameterTemplate {
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

pub fn register(_name: String, _display_order: u32) -> ParameterEntry {
  return ParameterEntry {
    id: 4,
    name: String::from("HP"),
    display_order: 1,
    is_deleted: false,
    version: 0,
  };
}

pub fn update(_id: u32, _name: String, _display_order: u32, _is_deleted: bool) -> ParameterEntry {
  return ParameterEntry {
    id: 4,
    name: String::from("HP"),
    display_order: 1,
    is_deleted: false,
    version: 0,
  };
}

pub fn delete(_id: u32) -> ParameterEntry {
  return ParameterEntry {
    id: 4,
    name: String::from("HP"),
    display_order: 1,
    is_deleted: false,
    version: 0,
  };
}
