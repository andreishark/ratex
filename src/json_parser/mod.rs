extern crate serde;
extern crate serde_json;

use std::{
    env, fs, io,
    path::{self, PathBuf},
};

use crate::models;

static JSON_NAME: &str = "config.json";

pub fn parse_json_file(file_path: &str) -> Result<models::ConfigModel, io::Error> {
    let mut json_path = env::current_exe()?;
    json_path.push(JSON_NAME);

    let json_file = fs::File::open(json_path)?;
    let reader = io::BufReader::new(json_file);

    let config_model: models::ConfigModel = serde_json::from_reader(reader)?;

    Ok(config_model)
}
