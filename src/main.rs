mod models;

use clap::Parser;
use crate::models::{Cli, ProfileCommands, ProviderCommands, RootCommands};

fn main() {
    let args = Cli::parse();

    match args.command {
        RootCommands::Provider(ProviderCommands::Add { name: _, region: _, url: _ }) => {
            todo!("TODO")
        },
        RootCommands::Provider(ProviderCommands::Get { name: _ }) => {
            todo!("TODO")
        },
        RootCommands::Provider(ProviderCommands::List {}) => {
            todo!("TODO")
        },
        RootCommands::Provider(ProviderCommands::Remove { name: _ }) => {
            todo!("TODO")
        },
        RootCommands::Provider(ProviderCommands::SignIn { name: _ }) => {
            todo!("TODO")
        },
        RootCommands::Profile(ProfileCommands::Add { name: _, provider: _, account_id: _, role: _ }) => {
            todo!("TODO")
        },
        RootCommands::Profile(ProfileCommands::Get { name: _ }) => {
            todo!("TODO")
        },
        RootCommands::Profile(ProfileCommands::List {}) => {
            todo!("TODO")
        },
        RootCommands::Profile(ProfileCommands::Remove { name: _ }) => {
            todo!("TODO")
        },
        RootCommands::Use { name: _ } => {
            todo!("TODO")
        },
        RootCommands::SetEnvVars {} => {
            todo!("TODO")
        },
        RootCommands::CopyEnvVars {} => {
            todo!("TODO")
        },
    }
}
