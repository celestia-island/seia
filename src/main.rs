use clap::{Parser, Subcommand};
use tracing_subscriber::EnvFilter;

use seia::{Engine, SearchClient, config::EngineRegistry};

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
        engine: String,

        /// Output as JSON
        #[arg(long)]
        json: bool,

        /// Fetch full page content for each result
        #[arg(long)]
        fetch: bool,

        /// Max results
        #[arg(short, long, default_value = "10")]
        limit: usize,

        /// HTTP/SOCKS proxy (e.g. http://localhost:7890). Overrides SEIA_PROXY / HTTPS_PROXY.
        #[arg(long)]
        proxy: Option<String>,
    },

    /// List available engines
    Engines,

    /// Run the MCP (Model Context Protocol) server on stdio, exposing the
    /// search tools to AI coding assistants.
    #[cfg(feature = "mcp")]
    Mcp,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();
    let registry = EngineRegistry::load().unwrap_or_default();

    match cli.cmd {
        Command::Search {
            query,
            engine,
            json,
            fetch,
            limit,
            proxy,
        } => {
            let eng = parse_engine(&engine);
            let client = if let Some(ref proxy_url) = proxy {
                SearchClient::with_proxy(proxy_url)?.with_registry(registry)
            } else {
                SearchClient::new().with_registry(registry)
            };
            let opts = seia::SearchOptions {
                limit: Some(limit),
                fetch_content: fetch,
                searxng_url: None,
            };

            let result = client.search_with_options(&query, eng, opts).await?;

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
            println!("Free (no key):");
            println!("  duckduckgo         — DuckDuckGo HTML scraping");
            println!("  wikipedia          — Wikipedia API, unlimited");
            println!("  semantic-scholar   — 200M+ academic papers");
            println!("  openalex           — 250M+ scholarly works");
            println!("  arxiv              — Preprints (physics, math, CS)");
            println!();
            println!("Free (self-hosted):");
            println!("  searxng            — Meta-search engine (SEARXNG_URL)");
            println!();
            println!("Paid / key required:");
            println!("  core               — Open-access papers (CORE_API_KEY)");
            println!("  tavily             — AI search API (TAVILY_API_KEY)");
            println!("  bing               — Bing Web Search (BING_SEARCH_API_KEY)");
            println!("  brave              — Brave Search (BRAVE_SEARCH_API_KEY)");
            println!("  zhipu              — 智谱 web_search (ZHIPU_API_KEY)");
            println!("  bocha              — 博查 Web Search (BOCHA_API_KEY)");
            println!("  metaso             — 秘塔 Web Search (METASO_API_KEY)");
            if !registry.engines.is_empty() {
                println!();
                println!("Custom (from config):");
                for (name, def) in &registry.engines {
                    println!("  {name:<20} — {} [{}]", def.label, def.method.to_uppercase());
                }
            }
        }

        #[cfg(feature = "mcp")]
        Command::Mcp => {
            seia::mcp::run().await?;
        }
    }

    Ok(())
}

fn parse_engine(raw: &str) -> Engine {
    let lower = raw.to_ascii_lowercase();
    match lower.as_str() {
        "duckduckgo" | "ddg" => Engine::Duckduckgo,
        "tavily" => Engine::Tavily,
        "searxng" => Engine::Searxng,
        "wikipedia" | "wiki" => Engine::Wikipedia,
        "bing" => Engine::Bing,
        "brave" => Engine::Brave,
        "zhipu" => Engine::Zhipu,
        "bocha" => Engine::Bocha,
        "metaso" => Engine::Metaso,
        "semantic-scholar" | "semanticscholar" | "s2" => Engine::SemanticScholar,
        "openalex" | "oa" => Engine::OpenAlex,
        "arxiv" => Engine::Arxiv,
        "core" => Engine::Core,
        other => Engine::Custom(other.to_string()),
    }
}

fn truncate(s: &str, max: usize) -> String {
    seia::utils::truncate(s, max)
}
