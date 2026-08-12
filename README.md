# AWS SSO Booster 🚀

A CLI tool which boosts AWS SSO sessions and profiles management.

## Install ⚙️

> [!IMPORTANT]  
> Rust and Cargo are required to install AWS SSO Booster 

```shell
cargo install --git https://github.com/Eraz1997/awsb
```

## AI Skill 🤖

```shell
mkdir -p ~/.config/<agent>/skills/awsb
curl -fsSL https://raw.githubusercontent.com/Eraz1997/awsb/main/skills/awsb/SKILL.md \
  -o ~/.config/<agent>/skills/awsb/SKILL.md
```

Replace `<agent>` with your agent's config directory (e.g. `claude`, `codex`, `opencode`). Once installed, any agent will automatically use the skill when you ask it to switch AWS profiles, authenticate with SSO, or manage providers.

## Usage 🎸

```shell
# Show available commands and print help message
awsb --help
awsb <COMMAND> <SUBCOMMAND> --help
awsb <COMMAND> --help

# Set profile as current
awsb use [PROFILE_NAME] # if you don't set PROFILE_NAME, an interactive selection menu is shown

# Manage SSO providers
awsb providers add [--name NAME] [--region REGION] [--url URL]
awsb providers list
awsb providers get [NAME]
awsb providers edit [NAME] # optionally pass --region / --url to skip the prompts
awsb providers remove [NAME]
awsb providers rename [NAME] [NEW_NAME]
awsb providers sign-in [NAME] # signs in with all providers if blank, add -s to pick one interactively

# Manage profiles
awsb profiles add [--name NAME] [--provider PROVIDER] [--account-id ACCOUNT_ID] [--role ROLE]
awsb profiles list
awsb profiles get [NAME]
awsb profiles edit [NAME] # optionally pass --provider / --account-id / --role to skip the prompts
awsb profiles remove [NAME]
awsb profiles rename [NAME] [NEW_NAME]

# Get AWS access environment variables
awsb print-env-vars
awsb copy-env-vars
```
