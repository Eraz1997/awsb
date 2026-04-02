use crate::error::Error;
use crate::managers::config_file::ConfigFileManager;
use inquire::Select;

fn resolve_profile_name(
    config_file_manager: &ConfigFileManager,
    name: Option<String>,
) -> Result<String, Error> {
    match name {
        Some(name) => Ok(name),
        None => {
            let names = config_file_manager.get_profile_names();
            if names.is_empty() {
                return Err(Error::NotFound);
            }
            Select::new("Select a profile:", names)
                .with_page_size(10)
                .prompt()
                .map_err(|_| Error::Aborted)
        }
    }
}

pub fn use_profile(
    mut config_file_manager: ConfigFileManager,
    name: Option<String>,
) -> Result<(), Error> {
    let name = resolve_profile_name(&config_file_manager, name)?;

    if !config_file_manager.get_profile_names().contains(&name) {
        return Err(Error::NotFound);
    }
    if let Some(profile) = config_file_manager.get_profile(&name) {
        if let Some(provider) = config_file_manager.get_provider(&profile.provider) {
            config_file_manager
                .overwrite_default_profile(profile, provider)
                .map(|_| ())
                .ok_or(Error::CouldNotEditConfigFile)
        } else {
            Err(Error::InvalidItem)
        }
    } else {
        Err(Error::InvalidItem)
    }
}
