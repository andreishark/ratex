use std::path;
extern crate serde;
extern crate serde_json;

#[derive(Debug, serde::Deserialize)]
pub struct ConfigModel {
    template_path: path::PathBuf,
}


impl ConfigModel {
    fn new(template_path: String) -> ConfigModel {
        ConfigModel {
            template_path: path::PathBuf::from(template_path),
        }
    }
}
