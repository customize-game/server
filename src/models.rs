use crate::schema::users;

#[derive(Insertable)]
#[table_name = "bodies"]
pub struct Body {
  pub name: String,
  pub id: Integer,
  pub name: String,
  pub ruby: Nullable<String>,
  pub flavor: Nullable<Text>,
  pub display_order: Integer,
  pub is_deleted: Tinyint,
  pub created_datetime: Datetime,
  pub updated_datetime: Datetime,
  pub version: Integer,
}

#[derive(Insertable)]
#[table_name = "bodies_hoge_interfaces"]
pub struct BodiesHogeInterface {
  body_id: Integer,
  hoge_interface_id: Integer,
  created_datetime: Datetime,
  updated_datetime: Datetime,
  version: Integer,
}

#[derive(Insertable)]
#[table_name = "body_free_sockets"]
pub struct BodyFreeSocket {
  body_id: Integer,
  x: Integer,
  y: Integer,
  operator: Nullable<String>,
  num: Nullable<Integer>,
  created_datetime: Datetime,
  updated_datetime: Datetime,
  version: Integer,
}

#[derive(Insertable)]
#[table_name = "body_statuses"]
pub struct BodyStatus {
  body_id: Integer,
  parameter_id: Integer,
  num: Integer,
  created_datetime: Datetime,
  updated_datetime: Datetime,
  version: Integer,
}

#[derive(Insertable)]
#[table_name = "designated_place_to_equipment_by_effects"]
pub struct DesignatedPlaceToEquipmentByEffect {
  equipment_id: Integer,
  hoge_intarface_id: Integer,
  parameter_id: Integer,
  num: Nullable<Integer>,
  created_datetime: Datetime,
  updated_datetime: Datetime,
  version: Integer,
}

#[derive(Insertable)]
#[table_name = "equipment_mysets"]
pub struct EquipmentMyset {
  myset_id: Integer,
  hoge_interface_id: Integer,
  equipment_id: Integer,
  created_datetime: Datetime,
  updated_datetime: Datetime,
  version: Integer,
}

#[derive(Insertable)]
#[table_name = "equipment_status"]
pub struct EquipmentStatus {
  equipment_id: Integer,
  parameter_id: Integer,
  num: Nullable<Integer>,
  created_datetime: Datetime,
  updated_datetime: Datetime,
  version: Integer,
}

#[derive(Insertable)]
#[table_name = "equipments"]
pub struct Equipment {
  id: Integer,
  name: String,
  ruby: Nullable<String>,
  flavor: Nullable<Text>,
  add_socket_count: Nullable<Integer>,
  display_order: Nullable<Integer>,
  is_deleted: Nullable<Tinyint>,
  created_datetime: Datetime,
  updated_datetime: Datetime,
  version: Integer,
}

#[derive(Insertable)]
#[table_name = "equipments_equipable_in_hoge_interfaces"]
pub struct EquipmentsEquipableInHogeInterface {
  equipment_id: Integer,
  hoge_interface_id: Integer,
  created_datetime: Datetime,
  updated_datetime: Datetime,
  version: Integer,
}

#[derive(Insertable)]
#[table_name = "equipped_when_increasing_hoge_interfaces"]
pub struct EquippedWhenIncreasingHogeInterface {
  equipment_id: Integer,
  hoge_interface_id: Integer,
  created_datetime: Datetime,
  updated_datetime: Datetime,
  version: Integer,
}

#[derive(Insertable)]
#[table_name = "equipped_when_unequipping_hoge_interfaces"]
pub struct EquippedWhenUnequippingHogeInterface {
  equipment_id: Integer,
  equipped_hoge_intarface_id: Integer,
  unequipping_hoge_intarface_id: Integer,
  created_datetime: Datetime,
  updated_datetime: Datetime,
  version: Integer,
}

#[derive(Insertable)]
#[table_name = "having_bodies"]
pub struct HavingBody {
  user_id: Integer,
  body_id: Integer,
  created_datetime: Datetime,
  updated_datetime: Datetime,
  version: Integer,
}

#[derive(Insertable)]
#[table_name = "having_equipments"]
pub struct HavingEquipment {
  user_id: Integer,
  equipment_id: Integer,
  count: Integer,
  created_datetime: Datetime,
  updated_datetime: Datetime,
  version: Integer,
}

#[derive(Insertable)]
#[table_name = "having_parameter_chips"]
pub struct HavingParameterChip {
  user_id: Integer,
  parameter_chip_id: Integer,
  count: Integer,
  created_datetime: Datetime,
  updated_datetime: Datetime,
  version: Integer,
}

#[derive(Insertable)]
#[table_name = "hoge_interfaces"]
pub struct HogeInterface {
  id: Integer,
  name: String,
  display_order: Integer,
  is_deleted: Tinyint,
  created_datetime: Datetime,
  updated_datetime: Datetime,
  version: Integer,
}

#[derive(Insertable)]
#[table_name = "hoge_sockets"]
pub struct HogeSocket {
  parameter_chip_id: Integer,
  x: Integer,
  y: Integer,
  created_datetime: Datetime,
  updated_datetime: Datetime,
  version: Integer,
}

#[derive(Insertable)]
#[table_name = "mysets"]
pub struct Myset {
  id: Integer,
  name: String,
  user_id: Integer,
  body_id: Integer,
  created_datetime: Datetime,
  updated_datetime: Datetime,
  version: Integer,
}

#[derive(Insertable)]
#[table_name = "parameter_chip_effects"]
pub struct ParameterChipEffect {
  parameter_chip_id: Integer,
  parameter_id: Integer,
  num: Nullable<Integer>,
  created_datetime: Datetime,
  updated_datetime: Datetime,
  version: Integer,
}

#[derive(Insertable)]
#[table_name = "parameter_chip_mysets"]
pub struct ParameterChipMyset {
  myset_id: Integer,
  parameter_chip_id: Integer,
  x: Integer,
  y: Integer,
  angle: Integer,
  created_datetime: Datetime,
  updated_datetime: Datetime,
  version: Integer,
}

#[derive(Insertable)]
#[table_name = "parameter_chips"]
pub struct ParameterChip {
  id: Integer,
  name: String,
  display_order: Integer,
  is_deleted: Tinyint,
  created_datetime: Datetime,
  updated_datetime: Datetime,
  version: Integer,
}

#[derive(Insertable)]
#[table_name = "parameters"]
pub struct Parameter {
  id: Integer,
  name: String,
  display_order: Integer,
  is_deleted: Tinyint,
  created_datetime: Datetime,
  updated_datetime: Datetime,
  version: Integer,
}

#[derive(Insertable)]
#[table_name = "participants"]
pub struct Participant {
  result_id: Integer,
  user_id: Integer,
  created_datetime: Datetime,
  updated_datetime: Datetime,
  version: Integer,
}

#[derive(Insertable)]
#[table_name = "results"]
pub struct Result {
  id: Integer,
  start_date: Datetime,
  end_date: Datetime,
  created_datetime: Datetime,
  updated_datetime: Datetime,
  version: Integer,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct User {
  id: Integer,
  exp: Integer,
  created_datetime: Datetime,
  updated_datetime: Datetime,
  version: Integer,
}
