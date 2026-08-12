use crate::commands::prompt::profile_name;
use crate::error::Error;
use crate::managers::config_file::ConfigFileManager;

pub fn get_profile(
    config_file_manager: ConfigFileManager,
    name: Option<String>,
) -> Result<(), Error> {
    let name = profile_name(&config_file_manager, name)?;
    if !config_file_manager.get_profile_names().contains(&name) {
        return Err(Error::NotFound);
    }
    match config_file_manager.get_profile(&name) {
        Some(profile) => {
            println!("{}", profile);
            Ok(())
        }
        None => Err(Error::InvalidItem),
    }
}
