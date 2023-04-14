use std::env;

pub struct DDNSUpdater {
    namecheap_pass: String,
    domains: Vec<String>,
}

impl DDNSUpdater {
    pub fn from_env() -> DDNSUpdater {
        DDNSUpdater {
            namecheap_pass: env::var("NAMECHEAP_PASS").expect("NAMECHEAP_PASS not set"),
            domains: env::var("NAMECHEAP_DOMAINS")
                .expect("NAMECHEAP_DOMAINS not set")
                .split(',')
                .map(|s| s.to_string())
                .collect(),
        }
    }

    pub fn get_domains(&self) -> &Vec<String> {
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

    fn get_split_domain(&self, domain: &str) -> anyhow::Result<(String, String)> {
        match (domain.contains('.'), domain.matches('.').count()) {
            (false, _) => anyhow::bail!("Invalid domain: {}", domain),
            (true, 1) => Ok((String::from("@"), domain.into())),
            _ => {
                let mut split = domain.splitn(2, '.');
                let subdomain = split.next().unwrap_or("@");
                let domain = split.next().unwrap();

                Ok((subdomain.into(), domain.into()))
            }
        }
    }

    async fn update_subdomain(
        &self,
        client: &reqwest::Client,
        subdomain: &str,
        domain: &str,
    ) -> anyhow::Result<bool> {
        let res = client
            .get("https://dynamicdns.park-your-domain.com/update")
            .query(&[
                ("host", subdomain),
                ("domain", domain),
                ("password", &self.namecheap_pass),
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
                domain,
                res
            );
        }
    }

    pub async fn update_domain(&self, domain: &str) -> anyhow::Result<bool> {
        println!("Updating domain: {}...", domain);

        let client = reqwest::Client::new();

        let (subdomain, domain) = self.get_split_domain(domain)?;

        self.update_subdomain(&client, &subdomain, &domain).await
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_env() {
        env::set_var("NAMECHEAP_PASS", "test");
        env::set_var("NAMECHEAP_DOMAINS", "test.com,test2.com");
    }

    #[test]
    fn test_from_env() {
        test_env();

        let ddns_updater = DDNSUpdater::from_env();

        assert_eq!(ddns_updater.namecheap_pass, "test");
        assert_eq!(ddns_updater.domains, vec!["test.com", "test2.com"]);
    }

    #[test]
    fn test_get_domain_only() {
        test_env();

        let ddns_updater = DDNSUpdater::from_env();

        assert_eq!(
            ddns_updater.get_split_domain("test.com").unwrap(),
            (String::from("@"), String::from("test.com"))
        );
    }

    #[test]
    fn test_get_subdomain() {
        test_env();

        let ddns_updater = DDNSUpdater::from_env();

        assert_eq!(
            ddns_updater.get_split_domain("test.test.com").unwrap(),
            (String::from("test"), String::from("test.com"))
        );
    }
}
