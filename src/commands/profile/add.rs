use crate::commands::prompt::{provider_name, value};
use crate::constants::{VALID_ACCOUNT_ID_REGEX, VALID_NAME_REGEX};
use crate::error::Error;
use crate::managers::config_file::ConfigFileManager;
use regex::Regex;

pub fn add_profile(
    mut config_file_manager: ConfigFileManager,
    name: Option<String>,
    provider: Option<String>,
    role: Option<String>,
    account_id: Option<String>,
) -> Result<(), Error> {
    let name = value("Profile name:", name)?;
    let provider = provider_name(&config_file_manager, provider)?;
    let account_id = value("AWS account ID:", account_id)?;
    let role = value("AWS role name:", role)?;

    Regex::new(VALID_NAME_REGEX)
        .ok()
        .filter(|regex| regex.is_match(name.as_str()))
        .ok_or(Error::InvalidName)?;
    Regex::new(VALID_ACCOUNT_ID_REGEX)
        .ok()
        .filter(|regex| regex.is_match(account_id.as_str()))
        .ok_or(Error::InvalidAccountID)?;

    if let Some(provider) = config_file_manager.get_provider(&provider) {
        config_file_manager
            .add_profile(name, provider, account_id, role)
            .map(|_| ())
            .ok_or(Error::CouldNotEditConfigFile)
    } else {
        Err(Error::ProviderNotFound)
    }
}
