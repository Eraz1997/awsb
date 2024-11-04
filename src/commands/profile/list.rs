use crate::managers::config_file::ConfigFileManager;

pub fn list_profiles(config_file_manager: ConfigFileManager) {
    let names = config_file_manager.get_profile_names();
    let valid_profiles: Vec<String> = names
        .clone()
        .into_iter()
        .filter(|name| config_file_manager.get_profile(name).is_some())
        .collect();
    let invalid_profiles: Vec<String> = names
        .into_iter()
        .filter(|name| !valid_profiles.contains(name))
        .collect();

    println!("{}", valid_profiles.join("\n"));

    if !invalid_profiles.is_empty() {
        eprintln!(
            "\nMalformed or invalid profiles (to manually fix or re-create):\n{}",
            invalid_profiles.join("\n")
        );
    }
}
