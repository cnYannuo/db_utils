use std::{fs::File, io::Read};

use serde_derive::Deserialize;
use toml::de::Error;
use serde::de;

#[derive(Deserialize)]
pub struct Config {
    pub profiles: Profiles
}

impl Config {
    pub fn get_config() -> Self {
        let mut str_val = String::new();
        read_toml("./application.toml", &mut str_val).unwrap()
    }
}

lazy_static::lazy_static! {
    pub static ref CONFIG: Config = Config::get_config();
}

#[derive(Deserialize)]
pub struct Profiles {
    pub db: String,
    pub static_resource: String,
    // pub interface: Vec<String>
    pub limit: u32
}

pub fn read_toml<'de, T>(file_path: &str, str_val:&'de mut String) -> Result<T, Error>
where
    T: de::DeserializeOwned,
{
    let mut file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => panic!("no such file {} exception:{}", file_path, e)
    };
    match file.read_to_string(str_val) {
        Ok(s) => s,
        Err(e) => panic!("Error Reading file: {}", e)
    };
    toml::from_str(str_val)
}

#[cfg(test)]
mod test{

    use serde_derive::Deserialize;

    use crate::{config::read_toml};

    #[derive(Deserialize)]
    struct Timer {
        timer_list: Vec<String>
    }

    #[derive(Deserialize)]
    struct Config {
        pub timer: Timer
    }

    #[test]
    fn test() {
        let mut str_val = String::new();
        let config: Config = read_toml("./test.toml", &mut str_val).unwrap();
        println!("{:?}", config.timer.timer_list);
        // let u: User = serde_json::from_str(&config.timer.timer_list).unwrap();
        // println!("{:?}", u);
    }
}