use crate::constants::{VALID_NAME_REGEX, VALID_REGION_REGEX};
use crate::error::Error;
use crate::managers::config_file::ConfigFileManager;
use regex::Regex;
use url::Url;

pub fn add_provider(
    mut config_file_manager: ConfigFileManager,
    name: String,
    region: String,
    url: String,
) -> Result<(), Error> {
    Url::parse(url.as_str()).map_err(|_| Error::InvalidUrl)?;
    Regex::new(VALID_NAME_REGEX)
        .ok()
        .filter(|regex| regex.is_match(name.as_str()))
        .ok_or(Error::InvalidName)?;
    Regex::new(VALID_REGION_REGEX)
        .ok()
        .filter(|regex| regex.is_match(region.as_str()))
        .ok_or(Error::InvalidRegion)?;

    if config_file_manager.get_provider(&name).is_some() {
        return Err(Error::AlreadyExists);
    }
    if config_file_manager.get_provider_by_url(&url).is_some() {
        return Err(Error::ClashingURL);
    }

    config_file_manager
        .add_provider(name, region, url)
        .ok_or(Error::CouldNotEditConfigFile)?;

    Ok(())
}
