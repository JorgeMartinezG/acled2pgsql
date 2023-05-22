use crate::acled::Params;
use serde::Deserialize;
use std::fs::read_to_string;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub acled: Params,
    //pub countries: HashMap<String, u16>,
    //pub database: Database,
}

impl Config {
    pub fn new(config_file: &str) -> Self {
        let content = read_to_string(config_file).unwrap();

        let config: Config = toml::from_str(content.as_str()).expect("Cannot read config file");

        config
    }
}
