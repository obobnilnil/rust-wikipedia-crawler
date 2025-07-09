pub fn parse_robots_txt(content: &str) -> Vec<String> {
    let mut disallows = Vec::new();
    let mut apply = false;

    for line in content.lines() {
        let line = line.trim();

        if line.starts_with("User-agent:") {
            apply = line.contains('*');
        } else if apply && line.starts_with("Disallow:") {
            if let Some(path) = line.split(':').nth(1) {
                let p = path.trim().to_string();
                if !p.is_empty() {
                    disallows.push(p);
                }
            }
        } else if line.starts_with("User-agent:") {
            apply = false;
        }
    }

    disallows
}
