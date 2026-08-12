use crate::error::Error;
use crate::managers::config_file::ConfigFileManager;
use inquire::{Select, Text};

pub fn provider_name(
    config_file_manager: &ConfigFileManager,
    name: Option<String>,
) -> Result<String, Error> {
    resolve_name(
        "Select a provider:",
        config_file_manager
            .get_provider_names()
            .into_iter()
            .filter(|name| config_file_manager.get_provider(name).is_some())
            .collect(),
        name,
    )
}

pub fn profile_name(
    config_file_manager: &ConfigFileManager,
    name: Option<String>,
) -> Result<String, Error> {
    resolve_name(
        "Select a profile:",
        config_file_manager
            .get_profile_names()
            .into_iter()
            .filter(|name| config_file_manager.get_profile(name).is_some())
            .collect(),
        name,
    )
}

pub fn value(message: &str, value: Option<String>) -> Result<String, Error> {
    match value {
        Some(value) => Ok(value),
        None => Text::new(message).prompt().map_err(|_| Error::Aborted),
    }
}

pub fn edited_value(message: &str, default: &str, value: Option<String>) -> Result<String, Error> {
    match value {
        Some(value) => Ok(value),
        None => Text::new(message)
            .with_default(default)
            .prompt()
            .map_err(|_| Error::Aborted),
    }
}

fn resolve_name(
    message: &str,
    mut names: Vec<String>,
    name: Option<String>,
) -> Result<String, Error> {
    match name {
        Some(name) => Ok(name),
        None => {
            if names.is_empty() {
                return Err(Error::NotFound);
            }
            names.sort();
            Select::new(message, names)
                .with_page_size(10)
                .prompt()
                .map_err(|_| Error::Aborted)
        }
    }
}
