<p align="center"><img src="https://raw.githubusercontent.com/celestia-island/docs.celestia.world/dev/res/logo/seia.webp" alt="Seia" width="240" /></p>

<h1 align="center">Seia</h1>

<p align="center"><strong>بحث ويب متعدد المحركات</strong></p>

<div align="center">

[![License: SySL-1.0](https://img.shields.io/badge/License-SySL--1.0-blue.svg)](https://sysl.celestia.world)
[![GitHub](https://img.shields.io/badge/github-celestia--island%2Fseia-blue.svg)](https://github.com/celestia-island/seia)
[![Checks](https://img.shields.io/github/actions/workflow/status/celestia-island/seia/checks.yml)](https://github.com/celestia-island/seia/actions/workflows/checks.yml)
[![Docs](https://img.shields.io/badge/docs-seia.docs.celestia.world-blue)](https://seia.docs.celestia.world)
[![docs.rs](https://docs.rs/seia/badge.svg)](https://docs.rs/seia)

</div>

<div align="center">

[English](../en/README.md) ·
[简体中文](../zhs/README.md) ·
[繁體中文](../zht/README.md) ·
[日本語](../ja/README.md) ·
[한국어](../ko/README.md) ·
[Français](../fr/README.md) ·
[Español](../es/README.md) ·
[Русский](../ru/README.md) ·
**العربية**

</div>

## مقدمة

seia هي مكتبة وأداة سطر أوامر للبحث متعدد المحركات. توفر واجهة موحدة للاستعلام
عن خلفيات بحث متنوعة. المحركات التي لا تتطلب مصادقة تعمل فورًا دون أي إعداد.

## البداية السريعة

### CLI

```bash
# Basic search (no API key required)
seia search "rust async patterns"

# Choose a specific engine
seia search "Klein bottle" --engine wikipedia

# JSON output
seia search "climate change" --json

# Through a proxy
HTTPS_PROXY=http://localhost:7890 seia search "hello world"
```

### المكتبة

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let results = client.search("rust async", Engine::Wikipedia).await?;
```

## خادم MCP

ابنِ seia بميزة `mcp` وشغّل خادم stdio — فهو يعرض عميل البحث متعدد المحركات لمساعدي الترميز بالذكاء الاصطناعي عبر بروتوكول سياق النموذج (Model Context Protocol):

```bash
seia mcp
```

يُعلن الخادم عن ثلاث أدوات: `seia_search` (محرك واحد، افتراضي duckduckgo بدون مفتاح)، `seia_search_multi` (يجرب سلسلة محركات، يُعيد الأول بنتائج)، و`seia_list_engines` (المحركات التسعة ومتغيرات بيئة مفاتيح API). وصله بعميل MCP:

```json
{
  "mcpServers": {
    "seia": { "command": "seia", "args": ["mcp"] }
  }
}
```

عيّن `SEIA_PROXY` لتوجيه طلبات البحث عبر وكيل (مثلاً `http://localhost:7890`)؛ `HTTPS_PROXY` / `HTTP_PROXY` مدعومة أيضاً.

## التطوير

```bash
just ci          # fmt-check + clippy + test
just test        # cargo test
just test-proxy  # run tests through localhost:7890 proxy (see tests/README)
```

## المحركات المدعومة

| المحرك | المصادقة |
|--------|---------|
| [DuckDuckGo](https://duckduckgo.com/) | بدون |
| [Wikipedia](https://www.mediawiki.org/wiki/API:Search) | بدون |
| [SearXNG](https://docs.searxng.org/) | `SEARXNG_URL` |
| [Tavily](https://docs.tavily.com/) | `TAVILY_API_KEY` |
| [Bing](https://learn.microsoft.com/en-us/bing/search-apis/bing-web-search/) | `BING_SEARCH_API_KEY` |
| [Brave](https://api.search.brave.com/app/documentation) | `BRAVE_SEARCH_API_KEY` |
| [秘塔 (MetaSo)](https://metaso.cn/search-api/playground) | `METASO_API_KEY` |
| [智谱 (Zhipu)](https://docs.bigmodel.cn/cn/guide/tools/web-search) | `ZHIPU_API_KEY` |
| [博查 (Bocha)](https://open.bochaai.com/docs) | `BOCHA_API_KEY` |

## الترخيص

SySL-1.0（Synthetic Source License）。انظر [LICENSE](https://sysl.celestia.world)。
