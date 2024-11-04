use crate::error::Error;
use crate::managers::config_file::ConfigFileManager;

pub fn remove_profile(
    mut config_file_manager: ConfigFileManager,
    name: String,
) -> Result<(), Error> {
    if config_file_manager.get_profile(&name).is_none() {
        return Err(Error::NotFound);
    }

    config_file_manager
        .remove_profile(name)
        .ok_or(Error::CouldNotEditConfigFile)
}
