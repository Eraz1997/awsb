use crate::commands::prompt::provider_name;
use crate::error::Error;
use crate::managers::config_file::ConfigFileManager;

pub fn get_provider(
    config_file_manager: ConfigFileManager,
    name: Option<String>,
) -> Result<(), Error> {
    let name = provider_name(&config_file_manager, name)?;
    if !config_file_manager.get_provider_names().contains(&name) {
        return Err(Error::NotFound);
    }
    match config_file_manager.get_provider(&name) {
        Some(provider) => {
            println!("{}", provider);
            Ok(())
        }
        None => Err(Error::InvalidItem),
    }
}
