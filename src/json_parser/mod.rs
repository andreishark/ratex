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

fn update_json_file(
    config_model: &models::ConfigModel,
    json_path: &path::Path,
) -> Result<(), io::Error> {
    let serialize = serde_json::to_string_pretty(&config_model)?;
    let mut json_file;

    json_file = fs::File::create(&json_path)?;

    json_file.write_all(serialize.as_bytes())?;

    Ok(())
}

fn init_json_file(
    json_path: &path::Path,
    template_name: &str,
    default_repo: &str,
) -> Result<models::ConfigModel, io::Error> {
    let mut config_model =
        models::ConfigModel::new(&template_name.to_owned(), true, default_repo, vec![]);
    config_model.first_init(template_name, default_repo)?;

    update_json_file(&config_model, json_path)?;

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
    let mut changed: bool = false;

    if config_model.repo_path.is_empty() {
        dbg!("Repo path is empty, updating json file");
        config_model = models::ConfigModel::new(
            config_model
                .template_path
                .to_str()
                .expect("Unexpected error occured"),
            config_model.first_run,
            default_repo,
            vec![],
        );

        changed = true;
    }

    if config_model.template_path.to_str().is_none()
        || config_model
            .template_path
            .to_str()
            .expect("Unexpected error")
            .is_empty()
    {
        dbg!("Template path is empty, updating json file");
        let new_template_path = get_exec_path(template_name)?;
        match fs::create_dir(&file_path) {
            Ok(_) => {}
            Err(err) => {
                if err.kind() == io::ErrorKind::AlreadyExists {
                    dbg!("Template directory already exists");
                } else {
                    return Err(err);
                }
            }
        };

        config_model = models::ConfigModel::new(
            new_template_path.to_str().expect("Unexpected error"),
            true,
            &config_model.repo_path,
            vec![],
        );

        changed = true;
    }

    if config_model.first_run {
        dbg!("First run, updating json file");
        config_model = models::ConfigModel::new(
            config_model
                .template_path
                .to_str()
                .expect("Unexpected error occured"),
            true,
            &config_model.repo_path,
            vec![],
        );

        init_json_file(
            &json_path,
            config_model
                .template_path
                .to_str()
                .expect("Unexpected error"),
            &config_model.repo_path,
        )?;

        return Ok(config_model);
    }

    if changed {
        update_json_file(&config_model, &json_path)?;
    }

    Ok(config_model)
}
