use crate::domains::users::UserRepository;
use config::ConfigError;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use lazy_static::lazy_static;
use serde::Deserialize;

pub type DB = diesel::mysql::Mysql;
pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;

#[derive(Clone)]
pub struct DbServer {
    pub pool: DbPool,
}

impl DbServer {
    pub fn new() -> Self {
        let database_url = format!(
            "mysql://{}:{}@{}:{}/{}",
            CONFIG.db_user, CONFIG.db_password, CONFIG.db_host, CONFIG.db_port, CONFIG.db_name
        );
        let manager = ConnectionManager::<MysqlConnection>::new(database_url);
        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create connection pool.");
        DbServer { pool }
    }

    pub fn user_repository(&self) -> impl UserRepository {
        use crate::infrastructures::repository::users::UserRepositoryImpl;

        UserRepositoryImpl {
            pool: Box::new(self.pool.clone()),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub db_user: String,
    pub db_password: String,
    pub db_name: String,
    pub db_host: String,
    pub db_port: i32,
    pub server_port: i32,
}

impl Default for Config {
    fn default() -> Self {
        Self { 
            db_user: "root".to_string(),
            db_password: "root".to_string(),
            db_name: "player_info".to_string(),
            db_host: "0.0.0.0".to_string(),
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


// TODO: separate SERVER and DB ENV VALUES
lazy_static! {
    pub static ref CONFIG: Config = {
        dotenv().ok();
        Config::get_env_info().unwrap()
    };
}
