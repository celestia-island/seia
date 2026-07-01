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

        /// Output as JSON
        #[arg(long)]
        json: bool,

        /// Fetch full page content for each result
        #[arg(long)]
        fetch: bool,

        /// Max results
        #[arg(short, long, default_value = "10")]
        limit: usize,

        /// Browser mode: search via tairitsu headless browser.
        /// Use with engines that need a real browser (google, baidu, bing_web, yandex).
        /// Requires either an external tairitsu daemon or the `embedded-browser` feature.
        #[arg(long)]
        browser: bool,

        /// tairitsu debug server endpoint (default: http://127.0.0.1:3001)
        #[arg(long, default_value = "http://127.0.0.1:3001")]
        tairitsu: String,

        /// Start embedded tairitsu server (requires `embedded-browser` feature).
        /// If set, ignores --tairitsu and spawns in-process.
        #[cfg(feature = "embedded-browser")]
        #[arg(long)]
        embedded: bool,

        /// Proxy server for the embedded browser (e.g. http://localhost:7890)
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
            json,
            fetch,
            limit,
            browser,
            tairitsu,
            #[cfg(feature = "embedded-browser")]
            embedded,
            proxy,
        } => {
            if browser {
                // Determine endpoint
                #[cfg(feature = "embedded-browser")]
                let endpoint = if embedded {
                    let port = 3001u16;
                    seia::embedded::start(port, proxy.as_deref())
                        .map_err(|e| anyhow::anyhow!("embedded server failed: {}", e))?;
                    // Wait for browser to connect
                    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                    format!("http://127.0.0.1:{}", port)
                } else {
                    tairitsu.clone()
                };
                #[cfg(not(feature = "embedded-browser"))]
                let endpoint = {
                    let _ = &proxy;
                    tairitsu.clone()
                };

                // Select profile
                let profile = match engine.as_str() {
                    "duckduckgo" => seia::profiles::get_profile("google")
                        .ok_or_else(|| anyhow::anyhow!("no profile"))?,
                    _ => seia::profiles::get_profile(engine.as_str())
                        .or_else(|| seia::profiles::get_profile("google"))
                        .ok_or_else(|| anyhow::anyhow!("no profile for engine {}", engine))?,
                };

                let client = seia::BrowserClient::new(&endpoint);

                if !client.health().await.unwrap_or(false) {
                    eprintln!("Warning: tairitsu browser not connected at {}", endpoint);
                    eprintln!("Start with: tairitsu debug --proxy http://localhost:7890");
                    #[cfg(feature = "embedded-browser")]
                    eprintln!("Or use: seia search '...' --browser --embedded");
                }

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
            let opts = seia::client::SearchOptions {
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
