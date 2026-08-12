---
name: awsb
description: Use when the user needs to authenticate to AWS, switch AWS profiles or accounts, manage AWS SSO providers, or run any `awsb` command. Triggers include "switch to AWS profile", "use AWS account", "sign in to AWS", "AWS SSO", "awsb", "list profiles", "add provider", or any request involving AWS credentials or profile switching.
---

# AWS SSO Booster (awsb) Skill

`awsb` is a CLI tool for managing AWS SSO sessions and profiles. It manages `~/.aws/config` directly and wraps the AWS CLI for credential export.

## Core concepts

- **Provider** — an AWS IAM Identity Center (SSO) tenant: has a name, a region, and a start URL (`sso_start_url`). Stored as `[sso-session <name>]` sections in `~/.aws/config`.
- **Profile** — an AWS account + role pair linked to a provider. Stored as `[profile <name>]` sections in `~/.aws/config`.
- **Default profile** — `awsb use <name>` copies a profile's settings into the `[default]` section of `~/.aws/config`, making it the active profile for all AWS CLI / SDK calls.

## When to use each command

| Goal | Command |
|---|---|
| Switch active AWS profile / account (default, human-driven) | `awsb use [PROFILE_NAME]` |
| Use a profile in agent-driven sessions (no default change) | `AWS_PROFILE=<NAME>` + profile name from `awsb profiles list` |
| Authenticate / refresh SSO credentials | `awsb providers sign-in [PROVIDER_NAME]` |
| List all profiles | `awsb profiles list` |
| List all providers | `awsb providers list` |
| Show current credentials as env vars | `awsb print-env-vars` |
| Copy credentials to clipboard | `awsb copy-env-vars` |
| Add a new SSO provider | `awsb providers add --name <NAME> --region <REGION> --url <URL>` |
| Add a new profile | `awsb profiles add --name <NAME> --provider <PROVIDER> --account-id <ACCOUNT_ID> --role <ROLE>` |
| Describe a profile | `awsb profiles get <NAME>` |
| Describe a provider | `awsb providers get <NAME>` |
| Remove a profile | `awsb profiles remove <NAME>` |
| Remove a provider | `awsb providers remove <NAME>` |
| Edit a profile | `awsb profiles edit [NAME]` |
| Edit a provider | `awsb providers edit [NAME]` |
| Rename a profile | `awsb profiles rename [NAME] [NEW_NAME]` |
| Rename a provider | `awsb providers rename [NAME] [NEW_NAME]` |

## Typical workflows

### Agent-driven sessions: use `AWS_PROFILE` instead of `awsb use`

When an agent needs to run AWS commands as a specific profile, do **not** use `awsb use <PROFILE_NAME>`: it rewrites the `[default]` section in `~/.aws/config`, which is a global change that can clobber the profile a human is actively using in a parallel session.

Instead, set the `AWS_PROFILE` environment variable for the agent's child processes:

```shell
awsb profiles list  # find the profile name
AWS_PROFILE=<PROFILE_NAME> aws sts get-caller-identity
# or, to scope it to the whole session:
export AWS_PROFILE=<PROFILE_NAME>
```

This selects the profile per-process only and leaves the user's `[default]` profile untouched.

### Switch the default profile (interactive / human-driven)

Use `awsb use` only when the *default* profile itself should change for the whole machine/session:

1. List available profiles to find the right name:
   ```shell
   awsb profiles list
   ```
2. Switch to the desired profile:
   ```shell
   awsb use <PROFILE_NAME>
   ```
   This overwrites the `[default]` section in `~/.aws/config`. All subsequent AWS CLI and SDK calls will use this profile.

### First-time setup: add a provider and profile

1. Add the SSO provider (get the start URL and region from your AWS admin):
   ```shell
   awsb providers add --name <NAME> --region <REGION> --url <SSO_START_URL>
   ```
2. Sign in to refresh local credentials:
   ```shell
   awsb providers sign-in <NAME>
   ```
   This opens a browser for SSO authentication.
3. Add a profile for a specific account and role:
   ```shell
   awsb profiles add --name <NAME> --provider <PROVIDER_NAME> --account-id <ACCOUNT_ID> --role <ROLE_NAME>
   ```
4. For agent-driven work, set the new profile without changing `[default]`:
   ```shell
   export AWS_PROFILE=<NAME>
   ```

### Refresh expired SSO credentials

```shell
awsb providers sign-in
```
Omit the provider name to sign in with all registered providers. Add `-s` to pick a single provider interactively (`awsb providers sign-in -s`).

### Export credentials as environment variables

```shell
eval $(awsb print-env-vars)
```
This sets `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, and `AWS_SESSION_TOKEN` in the current shell session.

## Important notes

- `awsb use` modifies the `[default]` section in `~/.aws/config`, not `~/.aws/credentials`. Prefer `AWS_PROFILE` over `awsb use` in agent-driven sessions so the user's default profile in parallel sessions is never changed.
- Profiles are only valid if their linked provider exists and the provider fields match exactly. If `awsb profiles get <NAME>` returns nothing, the profile config may be inconsistent.
- `awsb providers sign-in` requires the AWS CLI (`aws`) to be installed and available on PATH.
- `awsb print-env-vars` / `awsb copy-env-vars` call `aws configure export-credentials --format env-no-export` under the hood, so valid cached SSO credentials must already exist.
- All profile and provider names are arbitrary strings with no special characters; keep them lowercase and hyphenated for clarity.
