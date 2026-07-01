# 浏览器模式

部分引擎 —— Google、Baidu、Bing（网页，非 API）、Yandex —— 会激进地拦截非浏览器
请求。seia 通过 [tairitsu](https://github.com/celestia-island/tairitsu) 这个无头浏览器
运行时来驱动它们。seia 说的就是 tairitsu 的 HTTP debug API，因此**没有**原生浏览器
绑定。

## 两种运行 tairitsu 的方式

### 1. 外部守护进程（默认）

在应用之外运行一个 tairitsu debug 服务，然后让 seia 指向它：

```bash
# 一个终端
tairitsu debug --proxy http://localhost:7890

# 另一个终端
seia search "rust 异步" --engine google --browser --tairitsu http://127.0.0.1:3001
```

这样可以把沉重的浏览器进程挡在你的应用二进制之外。

### 2. 内嵌（`embedded-browser` feature）

把 tairitsu 的 debug 服务**编译进** seia，无需单独守护进程：

```toml
[dependencies]
seia = { version = "0.1", features = ["embedded-browser"] }
```

```bash
seia search "rust 异步" --engine google --browser --embedded
```

`embedded` 标志在进程内启动服务（见
[`seia::embedded::start`](https://github.com/celestia-island/seia/blob/dev/src/lib.rs)）。

## 浏览器搜索的工作流程

每次浏览器搜索分三步，全部发往 tairitsu 的 HTTP API：

1. **导航** —— `POST /navigate` 到引擎的搜索 URL。
2. **等待** —— `POST /wait-for-selector` 直到结果容器渲染出来。
3. **提取** —— `POST /evaluate` 运行一段 JS，从 DOM 中读取标题、链接与摘要。

每个引擎的选择器和 URL 模板定义在
[`SearchProfile`](https://github.com/celestia-island/seia/blob/dev/src/profiles.rs) 中：

| Profile | 搜索 URL | 结果容器 |
| --- | --- | --- |
| `google` | `google.com/search?q=` | `div.g` |
| `baidu` | `baidu.com/s?wd=` | `div.result, div.c-container` |
| `bing_web` | `bing.com/search?q=` | `li.b_algo` |
| `yandex` | `yandex.com/search/?text=` | `li.serps-item, div.Organic` |

## 直接使用浏览器客户端

```rust
use seia::{BrowserClient, profiles};

let client = BrowserClient::new("http://127.0.0.1:3001");

if !client.health().await.unwrap_or(false) {
    panic!("tairitsu 浏览器未连接");
}

let profile = profiles::get_profile("google").unwrap();
let result = client.search("rust 异步", profile).await?;
```

CLI 内部把 `--engine <名称> --browser` 映射到对应的 profile（无匹配时回退到
`google` profile）。
