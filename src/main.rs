use std::{fmt, io};

mod json_parser;
mod models;
mod ratex_lib;
mod template_handler;

#[derive(thiserror::Error, Debug)]
pub enum RatexError {
    IoError(#[from] io::Error),
    JsonError(#[from] serde_json::Error),
    GitError(#[from] git2::Error),
    InvalidUri(#[from] http::uri::InvalidUri),
}

impl fmt::Display for RatexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)?;
        Ok(())
    }
}

static CONFIG_JSON_NAME: &str = "config.json";
static TEMPLATE_DIR_NAME: &str = "templates";
static DEFAULT_REPO_PATH: &str = "https://github.com/andreishark/ratex";

fn run() -> Result<(), RatexError> {
    let config_model =
        json_parser::parse_json_file(CONFIG_JSON_NAME, TEMPLATE_DIR_NAME, DEFAULT_REPO_PATH)?;
    dbg!("Json file content", &config_model);

    template_handler::download_template(config_model, "test")?;

    Ok(())
}

fn main() -> Result<(), RatexError> {
    run()
}
