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
        #[arg(long)]
        json: bool,
        #[arg(long)]
        fetch: bool,
        #[arg(short, long, default_value = "10")]
        limit: usize,
        #[arg(long)]
        proxy: Option<String>,
    },

    /// List available engines
    Engines,

    /// Write built-in engine configs to ~/.seia/engines/
    #[command(name = "engines-install")]
    EnginesInstall,

    /// Reset built-in engine configs to defaults, overwriting user changes
    #[command(name = "engines-reset")]
    EnginesReset,

    /// Run the HTTP search service (`seia_search_v1` protocol)
    #[cfg(feature = "serve")]
    Serve {
        /// Bind address (default: $SEIA_BIND or 127.0.0.1:3013)
        #[arg(long)]
        bind: Option<String>,
    },

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
            let mut builtin = Vec::new();
            let mut custom = Vec::new();
            for (name, def) in &registry.engines {
                if def.builtin {
                    builtin.push((name.as_str(), def));
                } else {
                    custom.push((name.as_str(), def));
                }
            }
            builtin.sort_by_key(|(n, _)| *n);
            custom.sort_by_key(|(n, _)| *n);

            println!("Built-in ({}):", builtin.len());
            for (name, def) in &builtin {
                let key = Engine::from_name(name).and_then(|e| e.api_key_env());
                let key_note = if key.is_some() { " [key]" } else { "" };
                println!(
                    "  {name:<22} — {}{}",
                    def.help.as_deref().unwrap_or(&def.label),
                    key_note
                );
            }
            if !custom.is_empty() {
                println!();
                println!("Custom ({}):", custom.len());
                for (name, def) in &custom {
                    println!(
                        "  {name:<22} — {} [{}]",
                        def.label,
                        def.method.to_uppercase()
                    );
                }
            }
            if builtin.is_empty() {
                println!("Run `seia engines-install` to restore built-in configs.");
            }
        }

        Command::EnginesInstall => match EngineRegistry::install(false) {
            Ok(dir) => println!("Installed built-in engine configs to {}", dir.display()),
            Err(e) => eprintln!("Error: {e}"),
        },

        Command::EnginesReset => match EngineRegistry::reset() {
            Ok(dir) => println!("Reset built-in engine configs in {}", dir.display()),
            Err(e) => eprintln!("Error: {e}"),
        },

        #[cfg(feature = "serve")]
        Command::Serve { bind } => {
            seia::serve::run(bind).await?;
        }

        #[cfg(feature = "mcp")]
        Command::Mcp => {
            seia::mcp::run().await?;
        }
    }

    Ok(())
}

fn parse_engine(raw: &str) -> Engine {
    Engine::from_name(raw).unwrap_or_else(|| Engine::Custom(raw.to_string()))
}

fn truncate(s: &str, max: usize) -> String {
    seia::utils::truncate(s, max)
}
