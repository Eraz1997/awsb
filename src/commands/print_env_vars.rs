use crate::error::Error;
use std::process::Command;

pub fn print_env_vars() -> Result<(), Error> {
    let raw_output = Command::new("aws")
        .args([
            "configure",
            "export-credentials",
            "--format",
            "env-no-export",
        ])
        .output()
        .ok()
        .ok_or(Error::AWSCommandFailed)?;
    let output = String::from_utf8(raw_output.stdout).map_err(|_| Error::AWSCommandFailed)?;
    println!("{}", output);
    println!("# Use with \"eval\" to set environment variables in your current shell session.");

    Ok(())
}
