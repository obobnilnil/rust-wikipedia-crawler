![image](https://github.com/user-attachments/assets/08a14fd6-d630-477c-8c45-3a6590f7ef07)
🕷️ rust_crawler
A simple asynchronous web crawler written in Rust, designed for learning and experimenting with crawling logic on websites like Wikipedia.

✅ By default, the example is configured to crawl Wikipedia (https://en.wikipedia.org)<br>
✅ But it can be adapted to other websites by changing configuration and tweaking the link extraction logic

## Dependencies<br>
reqwest — HTTP client with async support (gzip, brotli, deflate, rustls-tls)

scraper — HTML parsing and CSS selector engine

tokio — Asynchronous runtime

## ⚙️ How it works
✅ Reads configuration from CrawlerConfig struct, including:

base_url: base site (e.g., https://en.wikipedia.org)

seed_path: initial path to crawl from (e.g., /wiki/Main_Page)

respect_robots: whether to honor robots.txt (⚠️ should be true in real usage!)

max_pages: maximum pages to visit

max_depth: how deep to follow links from the seed

delay_ms: delay between requests (milliseconds)

✅ Fetches and parses robots.txt<br>
✅ Extracts links under /wiki/<br>
✅ Skips links like /wiki/Special:, /wiki/Help: etc.<br>
✅ Uses VecDeque as a queue and HashSet to avoid revisiting pages<br>

## 💡 Can it crawl any website?
The current implementation is set up and tested on Wikipedia, using:

if href.starts_with("/wiki/") && !href.contains(':')
This filters links like:

✅ /wiki/Some_Page → crawl

❌ /wiki/Special:Page → skip

If you want to use it on other websites, you need to:

Change base_url and seed_path in the config

Adjust the href filter logic in the code to match that site’s URL structure

## 🚀 How to run
# Clone the project
```bash
git clone https://github.com/obobnilnil/rust-wikipedia-crawler.git
cd rust_crawler

# Build and run
cargo run
```
## 🖥️ Example crawl result
Below is an example output when running the crawler on Wikipedia:
```bash
🌐 Visiting: https://en.wikipedia.org/wiki/Main_Page
🌐 Visiting: https://en.wikipedia.org/wiki/Wikipedia
🌐 Visiting: https://en.wikipedia.org/wiki/Free_content
🌐 Visiting: https://en.wikipedia.org/wiki/Encyclopedia
🌐 Visiting: https://en.wikipedia.org/wiki/English_language
🌐 Visiting: https://en.wikipedia.org/wiki/Goblin_shark
🌐 Visiting: https://en.wikipedia.org/wiki/Shark
🌐 Visiting: https://en.wikipedia.org/wiki/Living_fossil
🌐 Visiting: https://en.wikipedia.org/wiki/Family_(biology)
🌐 Visiting: https://en.wikipedia.org/wiki/Mitsukurinidae
🌐 Visiting: https://en.wikipedia.org/wiki/Ampulla_of_Lorenzini
🌐 Visiting: https://en.wikipedia.org/wiki/Electric_field
🌐 Visiting: https://en.wikipedia.org/wiki/Continental_margin
🌐 Visiting: https://en.wikipedia.org/wiki/Submarine_canyon
🌐 Visiting: https://en.wikipedia.org/wiki/Seamount
🌐 Visiting: https://en.wikipedia.org/wiki/Teleost
🌐 Visiting: https://en.wikipedia.org/wiki/Cephalopod
🌐 Visiting: https://en.wikipedia.org/wiki/Crustacean
🌐 Visiting: https://en.wikipedia.org/wiki/Water_column
🌐 Visiting: https://en.wikipedia.org/wiki/International_Union_for_Conservation_of_Nature
🛑 Reached max pages: 20
✅ Crawl completed. Pages visited: 20
```
<p align="center">
  <img src="https://github.com/user-attachments/assets/2a6eedc8-10c2-48fe-8994-01fb1d4099ef" alt="robots.txt exmaple" />
</p>

And here is a screenshot of the corresponding Wikipedia page:
<p align="center">
  <img src="https://github.com/user-attachments/assets/5574e671-6fbf-461f-a2a9-f293e8347917" alt="robots.txt exmaple" />
</p>
This helps verify that the extracted links match real pages on the site.


## ⚠️ Important notes
⚠️ Respect robots.txt!

This project has a config flag:
```bash
respect_robots: false
**Note:** The default is set to `false` for testing and learning purposes.
```

👉 **In real-world use, you should set this to `true` to comply with site rules and respect `robots.txt`.**

## ❌ Ignoring `robots.txt` or crawling without permission can result in:

- IP bans
- Legal notices (for aggressive scraping)
- Breaking terms of service

This code is intended **for educational purposes only.**

## 🔍 How to check robots.txt of any website
Most websites define their crawling rules in a robots.txt file located at:
```bash
https://<website-domain>/robots.txt
```
✅ Example for Wikipedia, Shopee:
```bash
https://en.wikipedia.org/robots.txt
https://shopee.co.th/robots.txt
```
<p align="center">
  <img src="https://github.com/user-attachments/assets/6ee813c2-ac84-4016-8f4c-2a77bfe969c6" alt="robots.txt example" />
</p>
This file tells crawlers:

Which paths are allowed or disallowed

Crawl-delay or other special rules (if any)

Before running a crawler, you should always:

1️⃣ Visit the website’s robots.txt manually in a browser or using curl

2️⃣ Look for lines like:
```bash
User-agent: *
Disallow: /private/
Disallow: /admin/
```
3️⃣ Make sure your crawler respects these rules
By setting `respect_robots: true` in the config, the crawler will skip disallowed paths automatically.

## ✨ License: MIT License © 2025 Bank Worapon



