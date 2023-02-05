extern crate serde;
extern crate serde_json;

use std::io::Write;
use std::{env, fs, io, path};

use crate::models;

pub fn get_exec_path(file_path: &str) -> Result<path::PathBuf, io::Error> {
    let mut json_path = env::current_exe()?;
    json_path.pop();
    json_path.push(file_path);

    Ok(json_path)
}

pub fn parse_json_file(
    file_path: &str,
    template_name: &str,
    default_repo: &str,
) -> Result<models::ConfigModel, io::Error> {
    let mut json_path = env::current_exe()?;
    json_path.pop();
    json_path.push(file_path);

    dbg!("Reading json file from: ", json_path.to_str());
    let json_file = fs::File::open(&json_path).expect("Failed");
    let reader = io::BufReader::new(json_file);

    let mut config_model: models::ConfigModel = serde_json::from_reader(reader)?;

    if !config_model.first_run {
        let mut json_file = fs::File::open(&json_path)?;
        config_model.first_init(template_name)?;
        let serialize = serde_json::to_string(&config_model)?;
        match json_file.write_all(serialize.as_bytes()) {
            Ok(_) => {}
            Err(_) => {
                let mut file = fs::File::create(&json_path)?;
                file.write_all(serialize.as_bytes())?;
            }
        };
    }

    Ok(config_model)
}
