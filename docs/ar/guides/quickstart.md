# البداية السريعة

## التثبيت

```bash
# من crates.io (بعد النشر)
cargo install seia

# من المصدر
cargo install --path .
```

## أول بحث (CLI)

المحرك الافتراضي هو DuckDuckGo — مجاني، بلا مفتاح، يعمل فورًا:

```bash
seia search "rust async patterns"

# اختر محركًا آخر
seia search "Klein bottle" --engine wikipedia

# مخرجات قابلة للقراءة آليًا
seia search "climate change" --json

# جلب النص الكامل للصفحة لكل نتيجة (أبطأ)
seia search "tokio runtime" --fetch
```

نفّذ `seia engines` لسرد كل محرك ومعرفة ما إذا كان يتطلّب مفتاحًا.

## المحركات التي تتطلّب مفتاحًا

صدّر المفتاح في الصدفة (shell) — يقرأه seia تلقائيًا:

```bash
export TAVILY_API_KEY=tvly-xxxxx
seia search "react server components" --engine tavily

export SEARXNG_URL=http://localhost:8080
seia search "open source licenses" --engine searxng
```

## عبر وكيل (proxy)

يحترم `SearchClient` متغيّرات البيئة القياسية `HTTPS_PROXY` / `HTTP_PROXY` عبر
reqwest — لا حاجة لأي علامة (flag):

```bash
HTTPS_PROXY=http://localhost:7890 seia search "hello world"
```

## استخدامه كمكتبة

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

تابع إلى [المحركات](./engines.md) لمصفوفة المحركات الكاملة، أو
[استخدام المكتبة](./library.md) لواجهة الـ API البرمجية.
