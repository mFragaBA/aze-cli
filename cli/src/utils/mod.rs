use std::path::PathBuf;
use serde::Deserialize;
use figment::{
    providers::{ Format, Toml },
    Figment,
};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub player_ids: Vec<u64>,
    pub small_blind: u8,
    pub buy_in: u64,
}

pub fn load_config(config_file: &PathBuf) -> Result<Config, String> {
    Figment::from(Toml::file(config_file))
        .extract()
        .map_err(|err| format!("Failed to load {} config file: {err}", config_file.display()))
}