use crate::commands::prompt::profile_name;
use crate::error::Error;
use crate::managers::config_file::ConfigFileManager;

pub fn use_profile(
    mut config_file_manager: ConfigFileManager,
    name: Option<String>,
) -> Result<(), Error> {
    let name = profile_name(&config_file_manager, name)?;

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
