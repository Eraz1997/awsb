use crate::managers::config_file::ConfigFileManager;

pub fn list_providers(config_file_manager: ConfigFileManager) {
    let names = config_file_manager.get_provider_names();
    let valid_providers: Vec<String> = names
        .clone()
        .into_iter()
        .filter(|name| config_file_manager.get_provider(name).is_some())
        .collect();
    let invalid_providers: Vec<String> = names
        .into_iter()
        .filter(|name| !valid_providers.contains(name))
        .collect();

    println!("{}", valid_providers.join("\n"));

    if !invalid_providers.is_empty() {
        eprintln!(
            "\nMalformed or invalid providers (to manually fix or re-create):\n{}",
            invalid_providers.join("\n")
        );
    }
}
