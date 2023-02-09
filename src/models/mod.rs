use crate::json_parser;
use std::{fs, io, path};

extern crate serde;
extern crate serde_json;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ConfigModel {
    pub first_run: bool,
    pub template_path: path::PathBuf,
    pub repo_path: String,
    pub templates_name: Vec<String>,
}

impl Clone for ConfigModel {
    fn clone(&self) -> Self {
        Self {
            first_run: self.first_run,
            template_path: self.template_path.clone(),
            repo_path: self.repo_path.clone(),
            templates_name: self.templates_name.clone(),
        }
    }
}

impl ConfigModel {
    pub fn new(
        template_path: &str,
        first_run: bool,
        repo_path: &str,
        templates_name: Vec<String>,
    ) -> ConfigModel {
        ConfigModel {
            first_run: first_run,
            template_path: path::PathBuf::from(template_path),
            repo_path: repo_path.to_string(),
            templates_name,
        }
    }

    pub fn first_init(&mut self, template_name: &str, default_repo: &str) -> Result<(), io::Error> {
        self.first_run = true;
        let file_path = json_parser::get_exec_path(template_name)?;

        fs::create_dir(&file_path)?;

        self.template_path = file_path;
        self.repo_path = default_repo.to_string();

        Ok(())
    }
}
