use crate::commands::prompt::provider_name;
use crate::error::Error;
use crate::managers::config_file::ConfigFileManager;

pub fn remove_provider(
    mut config_file_manager: ConfigFileManager,
    name: Option<String>,
) -> Result<(), Error> {
    let name = provider_name(&config_file_manager, name)?;

    if config_file_manager.get_provider(&name).is_none() {
        return Err(Error::NotFound);
    }

    config_file_manager
        .remove_provider(name)
        .ok_or(Error::CouldNotEditConfigFile)
}
