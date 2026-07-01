<p align="center"><img src="../logo.webp" alt="seia" width="240" /></p>

<h1 align="center">seia</h1>

<div align="center">

<strong>المعرفة من كل المصادر</strong>

مكتبة وأداة سطر أوامر للبحث متعدد المحركات.

[![License: SySL-1.0](https://img.shields.io/badge/License-SySL--1.0-blue.svg)](../../LICENSE)
[![Crates.io](https://img.shields.io/crates/v/seia)](https://docs.rs/seia)
[![CI](https://img.shields.io/github/actions/workflow/status/celestia-island/seia/checks.yml)](https://github.com/celestia-island/seia/actions/workflows/checks.yml)
[![Docs](https://img.shields.io/badge/docs-seia.docs.celestia.world-blue)](https://seia.docs.celestia.world)

[English](../en/README.md) · [简体中文](../zhs/README.md) ·
[繁體中文](../zht/README.md) · [日本語](../ja/README.md) ·
[한국어](../ko/README.md) · [Français](../fr/README.md) ·
[Español](../es/README.md) · [Русский](../ru/README.md) ·
**العربية**

</div>

## مقدمة

seia هي مكتبة وأداة سطر أوامر للبحث متعدد المحركات. توفر واجهة موحدة للاستعلام
عن خلفيات بحث متنوعة. المحركات التي لا تتطلب مصادقة تعمل فورًا دون أي إعداد.

## البداية السريعة

### CLI

```bash
# بحث أساسي (بلا مفتاح API)
seia search "rust async patterns"

# اختيار محرك معين
seia search "Klein bottle" --engine wikipedia

# مخرجات JSON
seia search "climate change" --json

# عبر وكيل (proxy)
HTTPS_PROXY=http://localhost:7890 seia search "hello world"

# وضع المتصفح (بدون واجهة، بلا مفتاح API)
seia search "query" --browser --browser-engine google
```

### المكتبة

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let results = client.search("rust async", Engine::Duckduckgo).await?;
```

## التطوير

```bash
just ci          # fmt-check + clippy + test
just test        # cargo test
```

## المحركات المدعومة

### محركات API / الكشط

| المحرك | الموقع الرسمي | الوضع | المصادقة | الحد المجاني | الحالة |
|--------|-------------|------|---------|-----------|--------|
| DuckDuckGo | [duckduckgo.com](https://duckduckgo.com) | كشط | بدون | غير محدود | ✅ |
| Wikipedia | [wikipedia.org](https://www.wikipedia.org) | API | بدون | غير محدود | ✅ |
| SearXNG | [searxng.org](https://searxng.org) | API | `SEARXNG_URL` | استضافة ذاتية | ✅ |
| Tavily | [tavily.com](https://tavily.com) | API | `TAVILY_API_KEY` | 1 000/شهر | ✅ |
| Bing | [bing.com](https://www.bing.com) | API | `BING_SEARCH_API_KEY` | 1 000/شهر | 🔜 |
| Brave | [brave.com/search](https://brave.com/search) | API | `BRAVE_SEARCH_API_KEY` | 2 000/شهر | 🔜 |

> خلفيات Bing و Brave API هي هياكل مؤقتة (لم تُنفَّذ بعد). استخدم ملفات المتصفح كحل
> مؤقت، أو [ساهم](https://github.com/celestia-island/seia).

### محركات المتصفح (CLI فقط)

| المحرك | الموقع الرسمي | المصادقة | الوصف |
|--------|-------------|---------|-------|
| Google | [google.com](https://www.google.com) | بدون (كشط عبر tairitsu) | بحث Google على الويب. |
| Baidu | [baidu.com](https://www.baidu.com) | بدون (كشط عبر tairitsu) | بحث Baidu على الويب. |
| Bing Web | [bing.com](https://www.bing.com) | بدون (كشط عبر tairitsu) | نتائج Bing على الويب. |
| Yandex | [yandex.com](https://yandex.com) | بدون (كشط عبر tairitsu) | بحث Yandex على الويب. |

تستخدم محركات وضع المتصفح [tairitsu](https://github.com/celestia-island/tairitsu)
للعرض بلا واجهة. إمّا شغّل خادمًا خفيًّا مستقلًا، أو فعّل ميزة `embedded-browser`
لتجميع tairitsu داخل العملية.

> تقدم معظم محركات البحث واجهات REST API رسمية. ملفات المتصفح هي حل بديل للمحركات التي
> لم يُنفَّذ backend API الخاص بها بعد، أو عندما لا تكون API متاحة مجانًا. على المدى
> الطويل، سيحصل كل ملف متصفح على متغير `Engine` مطابق مع دعم مفتاح API.

## الترخيص

SySL-1.0 (ترخيص المصدر التركيبي). انظر [LICENSE](../../LICENSE).
