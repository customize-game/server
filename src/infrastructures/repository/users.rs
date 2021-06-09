use crate::domains::users::{User, UserId, UserRepository};
use crate::infrastructures::infrastructures::{DbPool, DB};
use diesel::deserialize::QueryableByName;
use diesel::mysql::types::Unsigned;
use diesel::sql_types::BigInt;
use diesel::{sql_query, RunQueryDsl};
use std::error::Error;


pub struct UserRepositoryImpl {
    pub pool: Box<DbPool>,
}


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct UserEntity {
    pub id: u64,
    pub name: String,
}


impl UserEntity {
    fn from(model: &User) -> UserEntity {
        UserEntity {
            id: model.id.get(),
            name: model.name.to_owned(),
        }
    }

    fn of(&self) -> User {
        User {
            id: UserId::new(self.id),
            name: self.name.to_owned(),
        }
    }
}

impl UserRepository for UserRepositoryImpl {
    fn find_by_id(&self, user_id: UserId) -> Result<User, Box<dyn Error + Send + Sync + 'static>> {
        let query = "
            SELECT
                id,
                name,
                email,
                pass
            FROM
                login
            Where
                id
                =
                ?
            ";
        let conn = self
            .pool
            .get()
            .expect("couldn't get db connection from pool");
        let user: UserEntity = sql_query(query)
            .bind::<Unsigned<BigInt>, _>(user_id.get())
            .get_result(&conn)
            .unwrap();
        Ok(user.of())
    }
}

impl QueryableByName<DB> for UserEntity {
    fn build<R: diesel::row::NamedRow<DB>>(row: &R) -> diesel::deserialize::Result<Self> {
        Ok(UserEntity {
            id: row.get("id")?,
            name: row.get("name")?,
        })
    }
}
