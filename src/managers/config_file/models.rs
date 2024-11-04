use std::fmt;
use std::fmt::Formatter;

pub struct Provider {
    pub name: String,
    pub region: String,
    pub url: String,
}

impl fmt::Display for Provider {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        writeln!(formatter, "Provider Name: {}", self.name)?;
        writeln!(formatter, "Region: {}", self.region)?;
        writeln!(formatter, "URL: {}", self.url)
    }
}

pub struct Profile {
    pub name: String,
    pub provider: String,
    pub account_id: String,
    pub role: String,
}

impl fmt::Display for Profile {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        writeln!(formatter, "Profile Name: {}", self.name)?;
        writeln!(formatter, "Provider: {}", self.provider)?;
        writeln!(formatter, "Account ID: {}", self.account_id)?;
        writeln!(formatter, "Role: {}", self.role)
    }
}
