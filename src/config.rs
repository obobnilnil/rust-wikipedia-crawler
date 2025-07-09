pub struct CrawlerConfig {
    pub base_url: String,
    pub seed_path: String,
    pub respect_robots: bool,
    pub max_pages: usize,
    pub max_depth: usize,
    pub delay_ms: u64,
}
