use crate::domains::domains::Id;
use std::error::Error;
use std::fmt;


pub type UserId = Id<User>;

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub name: String,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid type")
    }
}

impl User {
    pub fn create(name: String, email: String, password: String) -> Self {
        Self {
            id: Default::default(),
            name: name,
        }
    }
}

impl Error for User {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

pub trait UserRepository {
    fn find_by_id(&self, user_id: UserId) -> Result<User, Box<dyn Error + Send + Sync + 'static>>;
}
