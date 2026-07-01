use clap::{Parser, Subcommand};
use tracing_subscriber::EnvFilter;

use seia::{Engine, SearchClient};

#[derive(Parser)]
#[command(name = "seia", about = "One query, every search engine.")]
struct Cli {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Search the web
    Search {
        query: String,

        #[arg(short, long, value_enum, default_value = "duckduckgo")]
        engine: Engine,

        /// Browser engine name (when --browser is set): google, baidu, bing_web, yandex.
        #[arg(long)]
        browser_engine: Option<String>,

        /// Output as JSON
        #[arg(long)]
        json: bool,

        /// Fetch full page content for each result
        #[arg(long)]
        fetch: bool,

        /// Max results
        #[arg(short, long, default_value = "10")]
        limit: usize,

        /// Browser mode: search via embedded tairitsu headless browser.
        /// Zero external dependencies — browser runs in-process.
        #[arg(long)]
        browser: bool,

        /// Proxy server for browser (e.g. http://localhost:7890)
        #[arg(long)]
        proxy: Option<String>,
    },

    /// List available engines
    Engines,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    match cli.cmd {
        Command::Search {
            query,
            engine,
            browser_engine,
            json,
            fetch,
            limit,
            browser,
            proxy,
        } => {
            if browser {
                // Start embedded browser server
                let port = 3001u16;
                let endpoint = seia::embedded::start(port, proxy.as_deref())
                    .map_err(|e| anyhow::anyhow!("embedded server failed: {}", e))?;

                // Wait for browser to connect
                tokio::time::sleep(std::time::Duration::from_secs(15)).await;

                let client = seia::BrowserClient::new(&endpoint);
                if !client.health().await.unwrap_or(false) {
                    eprintln!("Warning: browser not ready at {}", endpoint);
                }

                // Select profile
                let engine_name = browser_engine.as_deref().unwrap_or("google");
                let profile = seia::profiles::get_profile(engine_name).ok_or_else(|| {
                    anyhow::anyhow!(
                        "unknown browser engine '{}'. Options: google, baidu, bing_web, yandex",
                        engine_name
                    )
                })?;

                let result = client.search(&query, profile).await?;

                if json {
                    println!("{}", serde_json::to_string_pretty(&result)?);
                } else {
                    println!(
                        "Engine: {} (browser) | {} results | {}ms\n",
                        result.engine,
                        result.items.len(),
                        result.elapsed_ms
                    );
                    for (i, item) in result.items.iter().enumerate() {
                        println!("{}. {}", i + 1, item.title);
                        println!("   {}", item.url);
                        if let Some(snippet) = &item.snippet {
                            println!("   {}", truncate(snippet, 120));
                        }
                        println!();
                    }
                }
                return Ok(());
            }

            // API / scrape mode
            let client = SearchClient::new();
            let opts = seia::SearchOptions {
                limit: Some(limit),
                fetch_content: fetch,
                searxng_url: None,
            };

            let result = client.search_with_options(&query, engine, opts).await?;

            if json {
                println!("{}", serde_json::to_string_pretty(&result)?);
            } else {
                println!(
                    "Engine: {} | {} results | {}ms\n",
                    result.engine,
                    result.items.len(),
                    result.elapsed_ms
                );
                for (i, item) in result.items.iter().enumerate() {
                    println!("{}. {}", i + 1, item.title);
                    println!("   {}", item.url);
                    if let Some(snippet) = &item.snippet {
                        println!("   {}", truncate(snippet, 120));
                    }
                    if let Some(content) = &item.content {
                        println!("   [content: {} chars]", content.len());
                    }
                    println!();
                }
            }
        }

        Command::Engines => {
            println!("Available engines:");
            println!("  duckduckgo  — Free, HTML scraping, no key needed");
            println!("  tavily      — AI-optimized API, free 1K/month (TAVILY_API_KEY)");
            println!("  searxng     — Self-hosted meta-search (SEARXNG_URL)");
            println!("  wikipedia   — Free, unlimited, academic knowledge");
            println!("  bing        — Bing Search API (BING_SEARCH_API_KEY) [planned]");
            println!("  brave       — Brave Search API (BRAVE_SEARCH_API_KEY) [planned]");
            println!();
            println!("Browser engines (--browser flag, zero setup):");
            println!("  google      — via embedded tairitsu browser");
            println!("  baidu       — via embedded tairitsu browser");
            println!("  bing_web    — via embedded tairitsu browser");
            println!("  yandex      — via embedded tairitsu browser");
        }
    }

    Ok(())
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}...", &s[..max])
    }
}
