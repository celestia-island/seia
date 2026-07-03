# البنية

seia هو crate واحد يوفّر كلًّا من مكتبة (`src/lib.rs`) وCLI (`src/main.rs`). الهدف
التصميمي هو **سطح استعلام واحد، خلفيات عديدة**: يختار المُستدعي `Engine` ويحصل على نفس
`SearchResult` بصرف النظر عن أيُّ خلفية أنتجته.

## خريطة الوحدات

```
src/
├── lib.rs          سطح الـ API العام
├── main.rs         clap CLI (search / engines)
├── engines.rs      Engine enum: as_str وapi_key_env وneeds_key
├── engines_impl/   وحدة لكل خلفية
│   ├── duckduckgo.rs   كشط (HTML)
│   ├── wikipedia.rs    API (JSON)
│   ├── tavily.rs       API (JSON، بمفتاح)
│   ├── searxng.rs      API (JSON، مستضاف ذاتيًا)
│   ├── bing.rs         API (JSON، بمفتاح)
│   ├── brave.rs        API (JSON، بمفتاح)
│   ├── zhipu.rs        API (JSON، بمفتاح — 智谱 Web Search)
│   ├── bocha.rs        API (JSON، بمفتاح — 博查 Web Search)
│   └── metaso.rs       API (JSON، بمفتاح — 秘塔 Web Search)
├── client.rs       SearchClient + SearchOptions
├── extractor.rs    جالب محتوى الصفحة الكامل (لِـ --fetch)
└── result.rs       SearchResult / SearchItem / SearchMode
```

## مسارا تنفيذ، نوع نتيجة واحد

تتقارب المسارات كلها على
[`SearchResult`](https://github.com/celestia-island/seia/blob/dev/src/result.rs):

```
query + Engine ─► SearchClient ─► engines_impl/* ─► توحيد ─► SearchResult
```

- **API** — يستدعي `engines_impl::<engine>::search(&http, query, &opts)` المزوّد،
  ويُلغي تسلسل JSON إلى عناصر `SearchItem`.
- **كشط** — نفس التوقيع (signature)، لكنه يحلّل صفحة نتائج HTML.

يسجّل `SearchMode` (`Api` / `Scrape`) أيُّ مسار أنتج النتيجة، فيتمكّن المُستدعي من
التمييز بين إجابة API منظَّمة وصفحة مكتشفة.

## الإرسال (Dispatch)

`SearchClient::search_with_options` هو `match` مسطّح على `Engine`. وإضافة خلفية تعني:
تنفيذ دالة واحدة في `engines_impl/`، وإضافة مُتغيّر `Engine`، وإضافة ذراع `match`. لا
يوجد trait object أو إرسال ديناميكي — مجموعة المحركات مغلقة ومعروفة وقت التجميع، مما
يجعل الـ API متوقّعًا والملف الثنائي صغيرًا.

## بدون متصفح مقطوع الرأس

يأتي seia عمدًا **بلا** أي أتمتة متصفح. كل خلفية هي عميل HTTP عادي. المحركات التي تحجب
بشراسة الطلبات غير الصادرة من متصفح (Google وBaidu وYandex للبحث على الويب) خارج
النطاق — أَدركها عبر واجهات HTTP API الرسمية الخاصة بها أو عبر أداة متصفح مخصَّصة مثل
[shirabe](https://github.com/celestia-island/shirabe) عند توفّرها كـ MCP مستقل.

## إثراء المحتوى

`SearchOptions::fetch_content` شأنٌ متعامد (orthogonal): بعد أن تُرجع المحرّكات
عناصر `SearchItem`، تنزّل `extractor::fetch_content` كل صفحة وتنظّفها. هذا مستقل عن
المحرّك.
