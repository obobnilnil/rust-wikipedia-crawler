mod config;
mod robots;
mod crawler;

use config::CrawlerConfig;
use reqwest::Client;
use robots::parse_robots_txt;
use crawler::run_crawler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = CrawlerConfig {
    base_url: "https://en.wikipedia.org".to_string(),
    seed_path: "/wiki/Main_Page".to_string(),
    respect_robots: false,
    max_pages: 20,
    max_depth: 2,
    delay_ms: 400,
    };

    let client = Client::builder()
        .user_agent("BankBot/0.1 (for learning; contact: worapon@example.com)")
        .build()?;

    let robots_txt = client
        .get(format!("{}/robots.txt", config.base_url))
        .send()
        .await?
        .text()
        .await?;

    let disallowed_paths = parse_robots_txt(&robots_txt);

    run_crawler(&config, &client, &disallowed_paths).await;

    Ok(())
}
