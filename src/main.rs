use std::io;

mod json_parser;
mod models;

static CONFIG_JSON_NAME: &str = "config.json";
static TEMPLATE_DIR_NAME: &str = "templates";

fn run() -> Result<(), io::Error> {
    let config_model = json_parser::parse_json_file(CONFIG_JSON_NAME, TEMPLATE_DIR_NAME)?;
    dbg!("Json file content", &config_model);

    Ok(())
}

fn main() -> Result<(), io::Error> {
    run()
}
