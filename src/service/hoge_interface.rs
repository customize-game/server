pub struct HogeInterfaceEntry {
  pub id: u32,
  pub name: String,
  pub display_order: u32,
  pub is_deleted: bool,
  pub version: u32,
}

pub struct HogeInterfaceTemplate {
  pub total_count: u32,
  pub hoge_interfaces: Vec<HogeInterfaceEntry>,
}

pub fn find_by_id(_id: u32) -> HogeInterfaceEntry {
  return HogeInterfaceEntry {
    id: _id,
    name: String::from("右手"),
    display_order: 1,
    is_deleted: false,
    version: 0,
  };
}

pub fn find_list(
  _sort_by: Option<u32>,
  _limit: Option<u32>,
  _offset: Option<u32>,
) -> HogeInterfaceTemplate {
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

pub fn register(_name: String, _display_order: u32) -> HogeInterfaceEntry {
  return HogeInterfaceEntry {
    id: 2,
    name: String::from("右手"),
    display_order: 1,
    is_deleted: false,
    version: 0,
  };
}

pub fn update(
  _id: u32,
  _name: String,
  _display_order: u32,
  _is_deleted: bool,
) -> HogeInterfaceEntry {
  return HogeInterfaceEntry {
    id: 2,
    name: String::from("右手"),
    display_order: 1,
    is_deleted: false,
    version: 0,
  };
}

pub fn delete(_id: u32) -> HogeInterfaceEntry {
  return HogeInterfaceEntry {
    id: 2,
    name: String::from("右手"),
    display_order: 1,
    is_deleted: false,
    version: 0,
  };
}
