use std::fs::File;
use std::io::Read;

use serde::Deserialize;

use toml;

pub struct DDNSUpdater {
    domains: Vec<DomainConfig>,
}

#[derive(Deserialize)]
pub struct Config {
    domains: Vec<DomainConfig>,
}

#[derive(Deserialize, Debug)]
pub enum DomainType {
    Namecheap,
    Google,
}

#[derive(Deserialize, Debug)]
pub struct DomainConfig {
    pub domain: String,
    pub subdomain: Option<String>,
    pub username: Option<String>,
    pub password: String,
    pub domain_type: DomainType,
}

impl DDNSUpdater {
    pub fn from_config(file: Option<String>) -> DDNSUpdater {
        match DDNSUpdater::read_config(&file.unwrap_or("config.toml".to_string())) {
            Ok(config) => DDNSUpdater {
                domains: config.domains,
            },
            Err(e) => {
                println!("{}", e);
                println!("Failed to read config file, using empty config. This is probably not what you want!");
                DDNSUpdater { domains: vec![] }
            }
        }
    }

    fn read_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let config: Config = toml::from_str(&contents)?;

        Ok(config)
    }

    pub fn get_domains(&self) -> &Vec<DomainConfig> {
        &self.domains
    }

    pub async fn get_ip(&self) -> reqwest::Result<String> {
        let client = reqwest::Client::new();

        let res = client
            .get("https://api.ipify.org")
            .send()
            .await
            .expect("Failed to send request");

        res.text().await
    }

    async fn update_namecheap(
        &self,
        client: &reqwest::Client,
        domain: &DomainConfig,
    ) -> anyhow::Result<bool> {
        let subdomain = domain.subdomain.as_deref().unwrap_or("@");

        let res = client
            .get("https://dynamicdns.park-your-domain.com/update")
            .query(&[
                ("host", subdomain),
                ("domain", &domain.domain),
                ("password", &domain.password),
                ("ip", &self.get_ip().await.unwrap()),
            ])
            .send()
            .await
            .expect("Failed to send request");

        let res = res.text().await.unwrap();

        if res.contains("<ErrCount>0</ErrCount>") {
            Ok(true)
        } else {
            // something went wrong
            anyhow::bail!(
                "Failed to update domain: {}.{} with error: {}",
                subdomain,
                domain.domain,
                res
            );
        }
    }

    async fn update_google(
        &self,
        client: &reqwest::Client,
        domain: &DomainConfig,
    ) -> anyhow::Result<bool> {
        let res = client
            .get("https://domains.google.com/nic/update")
            .query(&[
                ("hostname", &domain.domain),
                ("myip", &self.get_ip().await.unwrap()),
            ])
            .basic_auth(domain.username.as_ref().unwrap(), Some(&domain.password))
            .send()
            .await
            .expect("Failed to send request");

        let res = res.text().await.unwrap();

        if res.contains("good") {
            Ok(true)
        } else if res.contains("nochg") {
            Ok(false)
        } else {
            // something went wrong
            anyhow::bail!(
                "Failed to update domain: {} with error: {}",
                domain.domain,
                res
            );
        }
    }

    pub async fn update_domain(&self, domain: &DomainConfig) -> anyhow::Result<bool> {
        println!("⏳ Updating domain: {}...", &domain.domain);

        let client = reqwest::Client::new();

        match domain.domain_type {
            DomainType::Namecheap => self.update_namecheap(&client, &domain).await,
            DomainType::Google => self.update_google(&client, &domain).await,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_config() -> DDNSUpdater {
        DDNSUpdater::from_config(Some("example.config.toml".to_string()))
    }

    #[test]
    fn test_from_config() {
        let ddns_updater = get_config();

        assert_eq!(ddns_updater.get_domains().len(), 2);
        assert_eq!(ddns_updater.get_domains()[0].domain, "example.com");
        assert_eq!(ddns_updater.get_domains()[1].domain, "example.org");
    }
}
