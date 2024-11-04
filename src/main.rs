use crate::commands::copy_env_vars::copy_env_vars;
use crate::commands::print_env_vars::print_env_vars;
use crate::commands::profile::add::add_profile;
use crate::commands::profile::get::get_profile;
use crate::commands::profile::list::list_profiles;
use crate::commands::profile::remove::remove_profile;
use crate::commands::provider::add::add_provider;
use crate::commands::provider::get::get_provider;
use crate::commands::provider::list::list_providers;
use crate::commands::provider::remove::remove_provider;
use crate::commands::provider::sign_in::sign_in;
use crate::commands::use_profile::use_profile;
use crate::error::Error;
use crate::managers::config_file::ConfigFileManager;
use crate::models::{Cli, ProfileCommands, ProviderCommands, RootCommands};
use clap::Parser;
use std::process::exit;

mod commands;
mod constants;
mod error;
mod managers;
mod models;

fn execute_command(args: Cli) -> Result<(), Error> {
    let config_file_manager = ConfigFileManager::new()?;

    match args.command {
        RootCommands::Providers(ProviderCommands::Add { name, region, url }) => {
            add_provider(config_file_manager, name, region, url)
        }
        RootCommands::Providers(ProviderCommands::Get { name }) => {
            get_provider(config_file_manager, name)
        }
        RootCommands::Providers(ProviderCommands::List {}) => {
            list_providers(config_file_manager);
            Ok(())
        }
        RootCommands::Providers(ProviderCommands::Remove { name }) => {
            remove_provider(config_file_manager, name)
        }
        RootCommands::Providers(ProviderCommands::SignIn { name }) => {
            sign_in(config_file_manager, name)
        }
        RootCommands::Profiles(ProfileCommands::Add {
            name,
            provider,
            account_id,
            role,
        }) => add_profile(config_file_manager, name, provider, role, account_id),
        RootCommands::Profiles(ProfileCommands::Get { name }) => {
            get_profile(config_file_manager, name)
        }
        RootCommands::Profiles(ProfileCommands::List {}) => {
            list_profiles(config_file_manager);
            Ok(())
        }
        RootCommands::Profiles(ProfileCommands::Remove { name }) => {
            remove_profile(config_file_manager, name)
        }
        RootCommands::Use { name } => use_profile(config_file_manager, name),
        RootCommands::PrintEnvVars {} => print_env_vars(),
        RootCommands::CopyEnvVars {} => copy_env_vars(),
    }
}

fn main() {
    let args = Cli::parse();
    if let Err(error) = execute_command(args) {
        let message = match error {
            Error::AWSCommandFailed => "AWS command failed.",
            Error::AlreadyExists => "Item already exists.",
            Error::ClashingURL => "URL is already in use.",
            Error::CouldNotCopyToClipboard => "Could not copy to clipboard.",
            Error::CouldNotEditConfigFile => "Could not edit config file.",
            Error::CouldNotReadConfigFile => "Could not read AWS configuration file.",
            Error::HomeDirectoryNotFound => "Home directory not found.",
            Error::InvalidAccountID => "Invalid account ID.",
            Error::InvalidItem => "Item is present but is malformed or invalid.",
            Error::InvalidName => "Please provide a valid name.",
            Error::InvalidRegion => "Please provide a valid region.",
            Error::InvalidUrl => "Please provide a valid URL.",
            Error::NotFound => "Item not found.",
            Error::ProviderNotFound => "Provider not found.",
        };
        eprintln!("{}", message);
        exit(1);
    }
}
