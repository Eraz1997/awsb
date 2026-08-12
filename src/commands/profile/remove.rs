use crate::commands::prompt::profile_name;
use crate::error::Error;
use crate::managers::config_file::ConfigFileManager;

pub fn remove_profile(
    mut config_file_manager: ConfigFileManager,
    name: Option<String>,
) -> Result<(), Error> {
    let name = profile_name(&config_file_manager, name)?;

    if config_file_manager.get_profile(&name).is_none() {
        return Err(Error::NotFound);
    }

    config_file_manager
        .remove_profile(name)
        .ok_or(Error::CouldNotEditConfigFile)
}
