use crate::error::Error;
use crate::managers::config_file::ConfigFileManager;

pub fn get_provider(config_file_manager: ConfigFileManager, name: String) -> Result<(), Error> {
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
