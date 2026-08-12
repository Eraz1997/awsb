use crate::commands::prompt::provider_name;
use crate::error::Error;
use crate::managers::config_file::ConfigFileManager;
use std::process::Command;

pub fn sign_in(
    config_file_manager: ConfigFileManager,
    name: Option<String>,
    select: bool,
) -> Result<(), Error> {
    let mut valid_names: Vec<String> = config_file_manager
        .get_provider_names()
        .into_iter()
        .filter(|name| config_file_manager.get_provider(name).is_some())
        .collect();

    match name {
        Some(name) => {
            if !valid_names.contains(&name) {
                if config_file_manager.get_provider_names().contains(&name) {
                    return Err(Error::InvalidItem);
                }
                return Err(Error::NotFound);
            }
            valid_names = vec![name];
        }
        None if select => {
            valid_names = vec![provider_name(&config_file_manager, None)?];
        }
        None => {}
    }

    for name in valid_names {
        println!("Signing in with {}\n", name);
        Command::new("aws")
            .args(["sso", "login", "--sso-session", &name])
            .status()
            .ok()
            .filter(|status| status.success())
            .ok_or(Error::AWSCommandFailed)?;
    }

    Ok(())
}
