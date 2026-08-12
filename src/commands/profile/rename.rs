use crate::commands::prompt::{profile_name, value};
use crate::constants::VALID_NAME_REGEX;
use crate::error::Error;
use crate::managers::config_file::ConfigFileManager;
use regex::Regex;

pub fn rename_profile(
    mut config_file_manager: ConfigFileManager,
    name: Option<String>,
    new_name: Option<String>,
) -> Result<(), Error> {
    let name = profile_name(&config_file_manager, name)?;
    let new_name = value("New profile name:", new_name)?;

    Regex::new(VALID_NAME_REGEX)
        .ok()
        .filter(|regex| regex.is_match(new_name.as_str()))
        .ok_or(Error::InvalidName)?;

    if config_file_manager.get_profile(&name).is_none() {
        return Err(Error::NotFound);
    }
    if config_file_manager.get_profile(&new_name).is_some() {
        return Err(Error::AlreadyExists);
    }

    config_file_manager
        .rename_profile(name, new_name)
        .map(|_| ())
        .ok_or(Error::CouldNotEditConfigFile)
}
