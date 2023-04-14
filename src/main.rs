mod updater;

const SETTINGS_EMOJI: &str = "⚙️";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;

    let ddns_updater = updater::DDNSUpdater::from_config(None);

    println!("{} Initialized ddns updater!", SETTINGS_EMOJI);

    for domain in ddns_updater.get_domains() {
        let res = ddns_updater.update_domain(&domain).await;

        match res {
            Ok(updated) => {
                if updated {
                    println!("✅ Updated domain: {}", domain.domain);
                } else {
                    println!("⚠️ Domain: {} is already up to date", domain.domain);
                }
            }
            Err(e) => {
                println!("❌ Failed to update domain: {}", domain.domain);
                println!("{} Error: {}", SETTINGS_EMOJI, e);
            }
        }
    }

    Ok(())
}
