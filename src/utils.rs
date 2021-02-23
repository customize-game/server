use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use config::Environment;
use config::ConfigError;
use serde::Deserialize;
use lazy_static::lazy_static;

#[derive(Deserialize, Debug)]
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

impl Config {
    pub fn get_env_info() -> Result<Self, ConfigError> {
        let mut config = config::Config::new();
        config.merge(config::Environment::new())?;
        config.try_into()
    }
}

lazy_static! {
    static ref CONFIG: Config = {
        dotenv().ok();
        Config::get_env_info().unwrap()
    };
}


pub fn establish_connection() -> MysqlConnection {
    let database_url = format!("mysql://{}:{}@localhost:{}/{}", CONFIG.db_user, CONFIG.db_password, CONFIG.db_port, CONFIG.db_name);
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
