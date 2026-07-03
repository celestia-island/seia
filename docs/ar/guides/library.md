# استخدام المكتبة

كل ما يفعله CLI متاح كمكتبة. تقع الـ API العامة في جذر الـ crate في
[`lib.rs`](https://github.com/celestia-island/seia/blob/dev/src/lib.rs).

## عميل البحث

يملك `SearchClient` عميل `reqwest::Client` واحدًا (مع user agent شبيه بالمتصفح ومهلة
15 ثانية) ويُرسل الطلبات إلى الخلفية الصحيحة وفق `Engine`:

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let result = client.search("rust async", Engine::Duckduckgo).await?;
```

### مع وكيل (proxy)

يُهيّئ `SearchClient::with_proxy` وكيلًا صريحًا. كما يحترم العميل
`HTTPS_PROXY` / `HTTP_PROXY` تلقائيًا عبر reqwest.

```rust
let client = SearchClient::with_proxy("http://localhost:7890");
```

## خيارات البحث

يأخذ `search_with_options` معامل `SearchOptions` لتحديد العدد وجلب المحتوى وعنوان
نسخة SearXNG:

```rust
use seia::{SearchClient, Engine, SearchOptions};

let opts = SearchOptions {
    limit: Some(5),
    fetch_content: true,        // تنزيل نص كل صفحة (أبطأ)
    searxng_url: Some("http://localhost:8080".into()),
};

let result = client
    .search_with_options("query", Engine::Searxng, opts)
    .await?;
```

| الحقل | الافتراضي | المعنى |
| --- | --- | --- |
| `limit` | `Some(10)` | اقتطاع قائمة النتائج. |
| `fetch_content` | `false` | جلب النص الكامل للصفحة إلى `SearchItem::content`. |
| `searxng_url` | `None` | العنوان الأساسي لِـ SearXNG (وإلا يُقرأ من `SEARXNG_URL`). |

## التراجع عبر المحركات

يجرّب `search_fallback` قائمة محركات بالترتيب ويُرجع أول نتيجة غير فارغة — مفيد عندما
تُقيَّد خلفية مجانية بسعة:

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let result = client
    .search_fallback("query", &[Engine::Duckduckgo, Engine::Wikipedia])
    .await?;
```

## أنواع النتائج

```rust
pub struct SearchResult {
    pub engine: String,
    pub query: String,
    pub items: Vec<SearchItem>,
    pub elapsed_ms: u64,
}

pub struct SearchItem {
    pub title: String,
    pub url: String,
    pub snippet: Option<String>,   // ملخّص قصير من المحرك
    pub content: Option<String>,   // النص الكامل للصفحة (فقط مع --fetch)
}
```

كلاهما يشتقّ `Serialize`/`Deserialize`، لذا فإن `serde_json::to_string(&result)`
يمنحك تمامًا ما يطبعه `seia search --json`.

## المُقدِّمة (Prelude)

لاستيراد أكثر إيجازًا:

```rust
use seia::prelude::*;   // SearchClient, Engine, SearchResult, SearchItem, SearchMode
```
