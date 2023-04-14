mod updater;

const SETTINGS_EMOJI: &str = "⚙️";

#[tokio::main]
async fn main() {
    let ddns_updater = updater::DDNSUpdater::from_env();

    println!("{} Initialized ddns updater!", SETTINGS_EMOJI);

    for domain in ddns_updater.get_domains() {
        let res = ddns_updater.update_domain(&domain).await;

        match res {
            Ok(updated) => {
                if updated {
                    println!("{} Handling domain: {}", SETTINGS_EMOJI, domain);
                } else {
                    println!(
                        "{} Domain: {} is already up to date",
                        SETTINGS_EMOJI, domain
                    );
                }
            }
            Err(e) => {
                println!("{} Failed to update domain: {}", SETTINGS_EMOJI, domain);
                println!("{} Error: {}", SETTINGS_EMOJI, e);
            }
        }
    }
}
