pub struct EquipmentDataEntry {
    pub id: u32,
    pub name: String,
    pub ruby: String,
    pub flavor: String,
    pub display_order: u32,
    pub is_deleted: bool,
    pub version: u32,
}

pub fn find_by_id(_id: u32) -> EquipmentDataEntry {
  return EquipmentDataEntry {
    id: _id,
    name: String::from("ZAKU"),
    ruby: String::from("ザク"),
    flavor: String::from("ジオンの量産機"),
    display_order: 1,
    is_deleted: false,
    version:0
  };
}
