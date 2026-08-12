use crate::commands::prompt::{edited_value, profile_name};
use crate::constants::VALID_ACCOUNT_ID_REGEX;
use crate::error::Error;
use crate::managers::config_file::ConfigFileManager;
use regex::Regex;

pub fn edit_profile(
    mut config_file_manager: ConfigFileManager,
    name: Option<String>,
    provider: Option<String>,
    account_id: Option<String>,
    role: Option<String>,
) -> Result<(), Error> {
    let name = profile_name(&config_file_manager, name)?;
    let current = config_file_manager
        .get_profile(&name)
        .ok_or(Error::NotFound)?;
    let provider_name = edited_value("SSO provider:", &current.provider, provider)?;
    let account_id = edited_value("AWS account ID:", &current.account_id, account_id)?;
    let role = edited_value("AWS role name:", &current.role, role)?;

    Regex::new(VALID_ACCOUNT_ID_REGEX)
        .ok()
        .filter(|regex| regex.is_match(account_id.as_str()))
        .ok_or(Error::InvalidAccountID)?;

    let provider = config_file_manager
        .get_provider(&provider_name)
        .ok_or(Error::ProviderNotFound)?;

    config_file_manager
        .edit_profile(name, provider, account_id, role)
        .map(|_| ())
        .ok_or(Error::CouldNotEditConfigFile)
}
