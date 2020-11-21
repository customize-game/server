use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;

pub fn establish_connection() -> MysqlConnection {
    let database_url = "mysql://root:root@localhost:3306/player_info";
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
