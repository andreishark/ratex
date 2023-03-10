use crate::json_parser;
use std::io::Error;
use std::{fs, io, path};

extern crate serde;
extern crate serde_json;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ConfigModel {
    pub first_run: bool,
    pub template_path: path::PathBuf,
}

impl Clone for ConfigModel {
    fn clone(&self) -> Self {
        Self {
            first_run: self.first_run,
            template_path: self.template_path.clone(),
        }
    }
}

impl ConfigModel {
    pub fn new(template_path: String, first_run: bool) -> ConfigModel {
        ConfigModel {
            first_run: false,
            template_path: path::PathBuf::from(template_path),
        }
    }

    pub fn first_init(&mut self, template_name: &str) -> Result<(), io::Error> {
        self.first_run = true;
        let file_path = json_parser::get_exec_path(template_name)?;

        fs::create_dir(&file_path)?;

        self.template_path = file_path;

        Ok(())
    }
}
