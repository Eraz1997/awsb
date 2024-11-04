use crate::error::Error;
use crate::managers::config_file::ConfigFileManager;
use std::process::Command;

pub fn sign_in(config_file_manager: ConfigFileManager, name: Option<String>) -> Result<(), Error> {
    let names = config_file_manager.get_provider_names();
    let mut valid_names: Vec<String> = names
        .clone()
        .into_iter()
        .filter(|name| config_file_manager.get_provider(name).is_some())
        .collect();

    if let Some(name) = name {
        if !names.contains(&name) {
            return Err(Error::NotFound);
        } else if !valid_names.contains(&name) {
            return Err(Error::InvalidItem);
        } else {
            valid_names = vec![name];
        }
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
