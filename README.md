# AWS SSO Booster 🚀

A CLI tool which boosts AWS SSO sessions and profiles management.

## Install ⚙️

> [!IMPORTANT]  
> Rust and Cargo are required to install AWS SSO Booster 

```shell
cargo install --git https://github.com/Eraz1997/awsb
```

## Usage 🎸

```shell
# Show available commands and print help message
awsb --help
awsb <COMMAND> <SUBCOMMAND> --help
awsb <COMMAND> --help

# Set profile as current
awsb use <PROFILE_NAME>

# Manage SSO providers
awsb providers add --name <NAME> --region <REGION> --url <URL>
awsb providers list
awsb providers get <NAME>
awsb providers remove <NAME>
awsb providers sign-in

# Manage profiles
awsb profiles add --name <NAME> --provider <PROVIDER> --account-id <ACCOUNT_ID> --role <ROLE>
awsb profiles list
awsb profiles get <NAME>
awsb profiles remove <NAME>

# Get AWS access environment variables
awsb print-env-vars
awsb copy-env-vars
```