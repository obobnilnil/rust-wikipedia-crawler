use reqwest::Client;
use scraper::{Html, Selector};
use tokio::time::{sleep, Duration};
use std::collections::{HashSet, VecDeque};
use crate::config::CrawlerConfig;

pub async fn run_crawler(config: &CrawlerConfig, client: &Client, disallowed_paths: &[String]) {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((config.seed_path.to_string(), 0));

    while let Some((path, depth)) = queue.pop_front() {
        if visited.len() >= config.max_pages {
            println!("🛑 Reached max pages: {}", config.max_pages);
            break;
        }

        if depth > config.max_depth || visited.contains(&path) {
            continue;
        }

        if config.respect_robots && disallowed_paths.iter().any(|dis| path.starts_with(dis)) {
            println!("🚫 Blocked by robots.txt: {}", path);
            continue;
        }

        let full_url = format!("{}{}", config.base_url, path);
        println!("🌐 Visiting: {}", full_url);
        visited.insert(path.clone());

        let html = match client.get(&full_url).send().await {
            Ok(resp) => resp.text().await.unwrap_or_default(),
            Err(err) => {
                eprintln!("⚠️ Failed to fetch {}: {}", full_url, err);
                continue;
            }
        };

        let document = Html::parse_document(&html);
        let selector = Selector::parse("a").unwrap();

        for element in document.select(&selector) {
            if let Some(href) = element.value().attr("href") {
                if href.starts_with("/wiki/") && !href.contains(':') && !visited.contains(href) {
                    queue.push_back((href.to_string(), depth + 1));
                }
            }
        }

        sleep(Duration::from_millis(config.delay_ms)).await;
    }

    println!("✅ Crawl completed. Pages visited: {}", visited.len());
}
