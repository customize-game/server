use crate::domains::users::User;
use serde::Serialize;

// Domain モデルから API Response 用のデータに変換しています．
// Json でSerialize できるデータ型になります
#[derive(Debug, Clone, Serialize)]
pub struct UserListResponse {
    user: Vec<UserDTO>,
}

impl UserListResponse {
    pub fn new(users: Vec<User>) -> UserListResponse {
        UserListResponse {
            user: users.iter().map(|u| UserDTO::new(&u)).collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct UserDTO {
    id: u64,
    name: String,
}

impl UserDTO {
    pub fn new(model: &User) -> UserDTO {
        UserDTO {
            id: model.id.get(),
            name: model.name.to_owned(),
        }
    }
}
