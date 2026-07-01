# المحركات

يدعم seia 9 خلفيات، تُجلَب جميعها عبر واجهة HTTP API الرسمية الخاصة بها (أو، حيث لا
توجد API، عبر كشط HTML خفيف). لا يوجد متصفح بلا واجهة — seia هو عميل HTTP نقي، لذا
يعمل كل محرك من كلٍّ من الـ CLI والمكتبة عبر نفس الـ enum `Engine`.

تعرض معظم المحركات باقة مجانية؛ والتي تحتاج مفتاحًا تقرؤه من متغيّر بيئة موثَّق، فلا
يظهر أي مفتاح أبدًا في الكود أو وسائط الـ CLI.

## أوضاع التنفيذ

| الوضع | طريقة العمل | المحركات |
| --- | --- | --- |
| **API** | يستدعي HTTP API لمزوّد البحث ويحلّل JSON. | Tavily وSearXNG وWikipedia وBing وBrave و智谱 و博查 و秘塔 |
| **كشط** | يجلب صفحة نتائج HTML خفيفة ويستخرج النتائج. | DuckDuckGo |

## مصفوفة المحركات

### دولي

| المحرك | قيمة الـ Enum | الوضع | المصادقة | الباقة المجانية | الحالة |
| --- | --- | --- | --- | --- | --- |
| DuckDuckGo | `Duckduckgo` | كشط | بدون | غير محدود | ✅ |
| Wikipedia | `Wikipedia` | API | بدون | غير محدود | ✅ |
| SearXNG | `Searxng` | API | `SEARXNG_URL` | استضافة ذاتية | ✅ |
| Tavily | `Tavily` | API | `TAVILY_API_KEY` | 1 000 / شهر | ✅ |
| Bing | `Bing` | API | `BING_SEARCH_API_KEY` | 1 000 / شهر | ✅ |
| Brave | `Brave` | API | `BRAVE_SEARCH_API_KEY` | 2 000 / شهر | ✅ |

### محلي (الصين)

| المحرك | قيمة الـ Enum | الوضع | المصادقة | الحالة |
| --- | --- | --- | --- | --- |
| 智谱 (Zhipu / BigModel) | `Zhipu` | API | `ZHIPU_API_KEY` | ✅ |
| 博查 (Bocha) | `Bocha` | API | `BOCHA_API_KEY` | ✅ |
| 秘塔 (Metaso) | `Metaso` | API | `METASO_API_KEY` | ✅ |

> تستطيع Web Search API الخاصة بـ 智谱 توجيه الطلب عبر إحدى عدّة محركات خلفية —
> 智谱基础版 (`search_std`، الافتراضية)، 智谱高阶版 (`search_pro`)، 搜狗
> (`search_pro_sogou`)، أو 夸克 (`search_pro_quark`). اختر إحداها بمتغيّر البيئة
> `ZHIPU_SEARCH_ENGINE`.

> يُعيد 博查 لكل صفحة كلًّا من `snippet` قصير و`summary` أطول تولّده نماذج لغوية؛
> يُظهر seia أيَّهما أطول باعتباره `snippet` الخاص بالنتيجة.

> يعمل 秘塔 (Metaso) بنطاق بحث افتراضيّ يُساوي `webpage`؛ ولتغييره استعمل متغيّر البيئة
> `METASO_SCOPE`. كما يُحلَّل غلاف الاستجابة (response envelope) تحرّزيًا، فلا يفشل seia
> إن تَغيَّر شكله من جانب المزوّد.

## اختيار المحرك

CLI:

```bash
seia search "query" --engine wikipedia
seia search "查询" --engine zhipu      # يحتاج ZHIPU_API_KEY
```

المكتبة:

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
client.search("query", Engine::Wikipedia).await?;
client.search("查询", Engine::Zhipu).await?;   // يحتاج ZHIPU_API_KEY
```

## فحص البيانات الوصفية للمحرك

يحمل `Engine` بياناته الوصفية الخاصة:

```rust
use seia::Engine;

for engine in [Engine::Duckduckgo, Engine::Tavily, Engine::Bing, Engine::Zhipu] {
    println!("{:?}", engine);                 // duckduckgo / tavily / bing / zhipu
    println!("  يتطلّب مفتاحًا؟ {}", engine.needs_key());
    println!("  متغيّر المفتاح:    {:?}", engine.api_key_env());
}
```
