use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use config::ConfigError;

pub struct Config {
    pub db_user: String,
    pub db_password: String,
    pub db_name: String,
    pub db_port: i32,
    pub server_port: i32,
}

impl Default for Config {
    fn default() -> Self {
        Self { 
            db_user: "root".to_string(),
            db_password: "root".to_string(),
            db_name: "player_info".to_string(),
            db_port: 3306,
            server_port: 5000
        }
    }
}


pub fn establish_connection() -> MysqlConnection {
    let database_url = "mysql://root:root@localhost:3306/player_info";
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
