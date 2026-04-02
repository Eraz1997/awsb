use crate::constants::VALID_NAME_REGEX;
use crate::error::Error;
use crate::managers::config_file::ConfigFileManager;
use regex::Regex;

pub fn rename_provider(
    mut config_file_manager: ConfigFileManager,
    name: String,
    new_name: String,
) -> Result<(), Error> {
    Regex::new(VALID_NAME_REGEX)
        .ok()
        .filter(|regex| regex.is_match(new_name.as_str()))
        .ok_or(Error::InvalidName)?;

    if config_file_manager.get_provider(&name).is_none() {
        return Err(Error::NotFound);
    }
    if config_file_manager.get_provider(&new_name).is_some() {
        return Err(Error::AlreadyExists);
    }

    config_file_manager
        .rename_provider(name, new_name)
        .map(|_| ())
        .ok_or(Error::CouldNotEditConfigFile)
}
