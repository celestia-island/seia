# 快速开始

## 安装

```bash
# 从 crates.io（发布后）
cargo install seia

# 从源码
cargo install --path .
```

## 第一次搜索（CLI）

默认引擎是 DuckDuckGo —— 免费、无需密钥、立即可用：

```bash
seia search "rust 异步模式"

# 选择其它引擎
seia search "克莱因瓶" --engine wikipedia

# 机器可读输出
seia search "气候变化" --json

# 拉取每条结果的完整页面文本（较慢）
seia search "tokio 运行时" --fetch
```

运行 `seia engines` 列出全部引擎及是否需要密钥。

## 需要密钥的引擎

在 shell 中导出密钥，seia 会自动读取：

```bash
export TAVILY_API_KEY=tvly-xxxxx
seia search "react server components" --engine tavily

export SEARXNG_URL=http://localhost:8080
seia search "开源许可证" --engine searxng
```

## 通过代理

```bash
HTTPS_PROXY=http://localhost:7890 seia search "hello world"

# 或显式指定
seia search "hello world" --proxy http://localhost:7890
```

## 作为库使用

```rust
use seia::{SearchClient, Engine};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = SearchClient::new();
    let result = client.search("rust 异步", Engine::Duckduckgo).await?;

    for item in &result.items {
        println!("{} — {}", item.title, item.url);
    }
    Ok(())
}
```

继续阅读 [引擎](./engines.md) 查看完整引擎矩阵，或
[库的用法](./library.md) 查看编程接口。
