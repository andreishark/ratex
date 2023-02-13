use std::fs;

use crate::{models, RatexError};

pub fn download_template(
    config_model: models::ConfigModel,
    template_name: &str,
) -> Result<(), RatexError> {
    let mut template_path = config_model.template_path.clone();
    template_path.push(template_name);

    let mut repo_path = config_model.repo_path.to_owned();
    // repo_path.push_str("/");
    // repo_path.push_str(template_name);

    let repo_path = repo_path.as_str().parse::<http::Uri>()?;

    dbg!("Downloading to template path", &template_path);
    dbg!("Repo path", &repo_path);

    if fs::metadata(&template_path).is_ok() {
        return Err(RatexError::GitError(git2::Error::from_str(
            "Template already exists. If it is empty, delete it and try again.",
        )));
    }
    fs::create_dir(&template_path)?;

    let repo = match git2::Repository::clone(repo_path.to_string().as_str(), &template_path) {
        Ok(_) => {}
        Err(err) => {
            fs::remove_dir_all(&template_path)?;
            return Err(RatexError::GitError(git2::Error::from_str(
                format!(
                    "Failed to clone repo. Check if the repo path is correct. Error {}",
                    err
                )
                .as_str(),
            )));
        }
    };

    Ok(())
}
