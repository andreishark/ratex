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

fn init_json_file(
    json_path: &path::Path,
    template_name: &str,
    default_repo: &str,
) -> Result<models::ConfigModel, io::Error> {
    let mut config_model = models::ConfigModel::new(template_name.to_string(), true, default_repo);
    config_model.first_init(template_name, default_repo)?;

    let mut json_file = fs::File::create(json_path)?;
    let serialize = serde_json::to_string_pretty(&config_model)?;

    match json_file.write_all(serialize.as_bytes()) {
        Ok(_) => {}
        Err(_) => {
            let mut file = fs::File::create(json_path)?;
            file.write_all(serialize.as_bytes())?;
        }
    };

    Ok(config_model)
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

    if fs::metadata(&json_path).is_err() {
        dbg!("Json file not found, creating new one");
        let config_model = init_json_file(&json_path, template_name, default_repo)?;
        return Ok(config_model);
    }

    let json_file = fs::File::open(&json_path)?;
    let reader = io::BufReader::new(json_file);

    let mut config_model: models::ConfigModel = serde_json::from_reader(reader)?;

    Ok(config_model)
}
