use crate::error::Error;
use crate::managers::config_file::models::{Profile, Provider};
use home::home_dir;
use ini::configparser::ini::Ini;

pub mod models;

pub struct ConfigFileManager {
    path: String,
    document: Ini,
}

impl ConfigFileManager {
    pub fn new() -> Result<ConfigFileManager, Error> {
        let home_dir = home_dir().ok_or(Error::HomeDirectoryNotFound)?;
        let path = home_dir
            .join(".aws/config")
            .into_os_string()
            .into_string()
            .map_err(|_| Error::HomeDirectoryNotFound)?;
        let mut document = Ini::new();
        document.set_default_section("");
        document
            .load(path.as_str())
            .map_err(|_| Error::CouldNotReadConfigFile)?;

        Ok(ConfigFileManager { document, path })
    }

    pub fn get_provider_names(&self) -> Vec<String> {
        self.document
            .sections()
            .iter()
            .filter_map(|key| key.strip_prefix("sso-session "))
            .map(|name| name.to_string())
            .collect()
    }

    pub fn get_provider(&self, name: &String) -> Option<Provider> {
        let key = format!("sso-session {}", name);
        let region = self.document.get(key.as_str(), "sso_region")?.to_string();
        let url = self
            .document
            .get(key.as_str(), "sso_start_url")?
            .to_string();

        Some(Provider {
            name: name.clone(),
            region,
            url,
        })
    }

    pub fn get_provider_by_url(&self, url: &String) -> Option<Provider> {
        self.get_provider_names()
            .iter()
            .filter_map(|name| self.get_provider(name))
            .find(|provider| provider.url == *url)
    }

    pub fn add_provider(&mut self, name: String, region: String, url: String) -> Option<Provider> {
        let key = format!("sso-session {}", name);
        self.document
            .set(key.as_str(), "sso_region", region.clone().into());
        self.document
            .set(key.as_str(), "sso_start_url", url.clone().into());
        self.document.set(
            key.as_str(),
            "sso_registration_scopes",
            "sso:account:access".to_string().into(),
        );

        self.document.write(self.path.as_str()).ok()?;

        Some(Provider { name, region, url })
    }

    pub fn remove_provider(&mut self, name: String) -> Option<()> {
        let key = format!("sso-session {}", name);
        self.document.remove_section(key.as_str());
        self.document.write(self.path.as_str()).ok()
    }

    pub fn get_profile_names(&self) -> Vec<String> {
        self.document
            .sections()
            .iter()
            .filter_map(|key| key.strip_prefix("profile "))
            .map(|name| name.to_string())
            .collect()
    }

    pub fn get_profile(&self, name: &String) -> Option<Profile> {
        let key = format!("profile {}", name);
        let account_id = self
            .document
            .get(key.as_str(), "sso_account_id")?
            .to_string();
        let role = self
            .document
            .get(key.as_str(), "sso_role_name")?
            .to_string();
        let url = self
            .document
            .get(key.as_str(), "sso_start_url")?
            .to_string();
        let provider_name = self.document.get(key.as_str(), "sso_session")?.to_string();
        let sso_region = self.document.get(key.as_str(), "sso_region")?.to_string();
        let region = self.document.get(key.as_str(), "region")?.to_string();

        let provider = self.get_provider_by_url(&url)?;
        if provider_name != provider.name
            || sso_region != provider.region
            || region != provider.region
        {
            return None;
        }

        Some(Profile {
            name: name.clone(),
            provider: provider_name,
            account_id,
            role,
        })
    }

    pub fn add_profile(
        &mut self,
        name: String,
        provider: Provider,
        account_id: String,
        role: String,
    ) -> Option<Profile> {
        let key = format!("profile {}", name);
        self.document
            .set(key.as_str(), "sso_role_name", role.clone().into());
        self.document
            .set(key.as_str(), "sso_session", provider.name.clone().into());
        self.document
            .set(key.as_str(), "sso_account_id", account_id.clone().into());
        self.document
            .set(key.as_str(), "sso_region", provider.region.clone().into());
        self.document
            .set(key.as_str(), "region", provider.region.clone().into());
        self.document
            .set(key.as_str(), "sso_start_url", provider.url.clone().into());

        self.document.write(self.path.as_str()).ok()?;

        Some(Profile {
            name,
            provider: provider.name,
            role,
            account_id,
        })
    }

    pub fn remove_profile(&mut self, name: String) -> Option<()> {
        let key = format!("profile {}", name);
        self.document.remove_section(key.as_str());
        self.document.write(self.path.as_str()).ok()
    }

    pub fn overwrite_default_profile(
        &mut self,
        profile: Profile,
        provider: Provider,
    ) -> Option<Profile> {
        self.document
            .set("default", "sso_role_name", profile.role.clone().into());
        self.document
            .set("default", "sso_session", provider.name.clone().into());
        self.document.set(
            "default",
            "sso_account_id",
            profile.account_id.clone().into(),
        );
        self.document
            .set("default", "sso_region", provider.region.clone().into());
        self.document
            .set("default", "region", provider.region.clone().into());
        self.document
            .set("default", "sso_start_url", provider.url.clone().into());

        self.document.write(self.path.as_str()).ok()?;

        Some(profile)
    }
}
