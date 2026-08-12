use crate::commands::prompt::{edited_value, provider_name};
use crate::constants::VALID_REGION_REGEX;
use crate::error::Error;
use crate::managers::config_file::ConfigFileManager;
use regex::Regex;
use url::Url;

pub fn edit_provider(
    mut config_file_manager: ConfigFileManager,
    name: Option<String>,
    region: Option<String>,
    url: Option<String>,
) -> Result<(), Error> {
    let name = provider_name(&config_file_manager, name)?;
    let current = config_file_manager
        .get_provider(&name)
        .ok_or(Error::NotFound)?;
    let region = edited_value("AWS region:", &current.region, region)?;
    let url = edited_value("SSO start URL:", &current.url, url)?;

    Regex::new(VALID_REGION_REGEX)
        .ok()
        .filter(|regex| regex.is_match(region.as_str()))
        .ok_or(Error::InvalidRegion)?;
    Url::parse(url.as_str()).map_err(|_| Error::InvalidUrl)?;

    if let Some(provider) = config_file_manager.get_provider_by_url(&url) {
        if provider.name != name {
            return Err(Error::ClashingURL);
        }
    }

    config_file_manager
        .edit_provider(name, region, url)
        .map(|_| ())
        .ok_or(Error::CouldNotEditConfigFile)
}
