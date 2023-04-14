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

        // try to split the domain into subdomain and domain
        // if there is no subdomain, use @ as the subdomain

        if domain.matches('.').count() > 1 {
            let mut split = domain.splitn(2, '.');
            let subdomain = split.next().unwrap();
            let domain = split.next().unwrap();

            let res = self.update_subdomain(&client, subdomain, domain).await?;
            Ok(res)
        } else {
            let res = self.update_subdomain(&client, "@", domain).await?;
            Ok(res)
        }
    }
}
