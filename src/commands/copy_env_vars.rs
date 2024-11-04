use crate::error::Error;
use copypasta::{ClipboardContext, ClipboardProvider};
use std::process::Command;

pub fn copy_env_vars() -> Result<(), Error> {
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

    let mut clipboard = ClipboardContext::new().map_err(|_| Error::CouldNotCopyToClipboard)?;
    clipboard
        .set_contents(output)
        .map_err(|_| Error::CouldNotCopyToClipboard)
}
