# Inicio rápido

## Instalación

```bash
# Desde crates.io (cuando se publique)
cargo install seia

# Desde el código fuente
cargo install --path .
```

## Primera búsqueda (CLI)

El motor por defecto es DuckDuckGo: gratuito, sin clave y funciona de inmediato:

```bash
seia search "rust async patterns"

# Elegir otro motor
seia search "Klein bottle" --engine wikipedia

# Salida legible por máquina
seia search "climate change" --json

# Obtener el texto completo de cada página de resultado (más lento)
seia search "tokio runtime" --fetch
```

Ejecuta `seia engines` para listar todos los motores y si necesitan una clave.

## Motores que necesitan una clave

Exporta la clave en tu shell: seia la leerá automáticamente:

```bash
export TAVILY_API_KEY=tvly-xxxxx
seia search "react server components" --engine tavily

export SEARXNG_URL=http://localhost:8080
seia search "open source licenses" --engine searxng
```

## A través de un proxy

```bash
HTTPS_PROXY=http://localhost:7890 seia search "hello world"
```

## Usarlo como biblioteca

```rust
use seia::{SearchClient, Engine};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = SearchClient::new();
    let result = client.search("rust async", Engine::Duckduckgo).await?;

    for item in &result.items {
        println!("{} — {}", item.title, item.url);
    }
    Ok(())
}
```

Continúa con [Motores](./engines.md) para ver la matriz completa de motores, o con
[Uso como biblioteca](./library.md) para la API programática.
