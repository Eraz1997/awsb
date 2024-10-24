use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "awsb")]
#[command(about = "A CLI tool which boosts AWS SSO sessions and profiles management")]
#[command(arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: RootCommands,
}

#[derive(Subcommand)]
pub enum RootCommands {
    #[command(subcommand, about = "Manage SSO providers")]
    Provider(ProviderCommands),
    #[command(subcommand, about = "Manage AWS CLI profiles")]
    Profile(ProfileCommands),
    #[command(about = "Switch to a registered profile", )]
    Use {
        #[arg(help = "The name of the profile to use")]
        name: String,
    },
    #[command(about = "Set the shell environment variables for AWS authentication")]
    SetEnvVars,
    #[command(about = "Copy the environment variables for AWS authentication to the clipboard")]
    CopyEnvVars,
}

#[derive(Subcommand)]
pub enum ProviderCommands {
    #[command(about = "Add a provider")]
    Add {
        #[arg(short, long, help = "An arbitrary name for the provider")]
        name: String,
        #[arg(short, long, help = "The AWS region of the provider")]
        region: String,
        #[arg(short, long, help = "The SSO start URL of the provider")]
        url: String,
    },
    #[command(about = "Describe a provider")]
    Get {
        #[arg(help = "The provider name")]
        name: String,
    },
    #[command(about = "List available providers")]
    List,
    #[command(about = "Remove a provider")]
    Remove {
        #[arg(help = "The provider name")]
        name: String,
    },
    #[command(about = "Sign-in and refresh the local AWS credentials")]
    SignIn {
        #[arg(help = "The name of the provider to sign in with, all of them if blank")]
        name: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum ProfileCommands {
    #[command(about = "Add a profile")]
    Add {
        #[arg(short, long, help = "An arbitrary name for the profile")]
        name: String,
        #[arg(short, long, help = "The name of the SSO provider to use to authenticate")]
        provider: String,
        #[arg(short, long, help = "The ID of the AWS account to bind")]
        account_id: String,
        #[arg(short, long, help = "The name of the role to authenticate with")]
        role: String,
    },
    #[command(about = "List available profiles")]
    List,
    #[command(about = "Describe a profile")]
    Get {
        #[arg(help = "The profile name")]
        name: String,
    },
    #[command(about = "Remove a profile")]
    Remove {
        #[arg(help = "The profile name")]
        name: String,
    },
}