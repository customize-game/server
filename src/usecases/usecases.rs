use crate::domains::users::{User, UserId, UserRepository};

use std::error::Error;

pub fn find_by_id(
    pool: impl UserRepository,
    user_id: UserId,
) -> Result<User, Box<dyn Error + Send + Sync + 'static>> {
    pool.find_by_id(user_id)
}
