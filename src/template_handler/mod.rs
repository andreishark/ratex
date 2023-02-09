use std::fs;

use crate::{models, RatexError};

pub fn download_template(
    config_model: models::ConfigModel,
    template_name: &str,
) -> Result<(), RatexError> {
    let mut template_path = config_model.template_path.clone();
    template_path.push(template_name);

    fs::create_dir(&config_model.template_path)?;
    let repo = git2::Repository::clone(&config_model.repo_path, template_path)?;

    Ok(())
}
