use serde_derive::Deserialize;

use crate::config::{CONFIG, read_toml};

#[derive(Deserialize)]
pub struct Db {
    pub mysql: Mysql,
    pub redis: Redis,
}

#[derive(Deserialize)]
pub struct Mysql {
    ip: String,
    port: u32,
    username: String,
    password: String,
    db_name: String
}

#[derive(Deserialize)]
pub struct Redis {
    ip: String,
    port: u32,
    token_timeout: usize,
    timeout: usize,
}

impl Db {

    // 目前只支持mysql
    pub fn get_db_url() -> String {
        let db_config = Self::get_db_config(&CONFIG.profiles.db);
        format!("mysql://{}:{}@{}:{}/{}", 
            db_config.mysql.username, 
            db_config.mysql.password, 
            db_config.mysql.ip, 
            db_config.mysql.port, 
            db_config.mysql.db_name, 
        )
    }

    // redis url
    pub fn get_redis_url() -> String {
        let db_config = Self::get_db_config(&CONFIG.profiles.db);
        format!("redis://{}:{}/", 
            db_config.redis.ip, 
            db_config.redis.port
        )
    }

    /// redis token超时时间
    pub fn get_redis_token_timeout() -> usize {
        let db_config = Self::get_db_config(&CONFIG.profiles.db);
        db_config.redis.token_timeout
    }

    /// redis 超时时间
    pub fn get_redis_timeout() -> usize {
        let db_config = Self::get_db_config(&CONFIG.profiles.db);
        db_config.redis.timeout
    }

    fn get_db_config(db_name: &str) -> Self {
        let mut str_val = String::new();
        read_toml(&format!("./db_{}.toml", db_name), &mut str_val).unwrap()
    }
    

}