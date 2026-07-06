# seia — 项目状态与计划 (PLAN)

> 本文件于 **2026-07-04** 刷新，记录项目当前状态、近期进展与后续计划。

## 1. 项目概述

- **名称**：`seia`
- **简介**：多引擎网页搜索库 / CLI。
- **远程仓库**：git@github.com:celestia-island/seia.git
- **技术栈**：Rust / just
- **类别**：rust-lib

## 2. 当前状态

- **当前分支**：`dev`
- **工作区**：干净（无未提交改动）
- **最近提交时间**：2026-07-04
- **最近提交**：fix: justfile ci recipe syntax (`0b895c3`)
- **分支对比**：`dev` 领先 `master` 65 个提交

## 3. 工作区状态

工作区干净，无未提交改动。此前 §3 列出的 `src/lib.rs` prelude 修复、`res/` PNG 快照移除、README `<details>` 区块移除均已提交（`e3899ff`、`9045372`），本次刷新不再重复描述。

## 4. 近期进展（最近提交）

- fix: justfile ci recipe syntax
- fix: remove bogus res/ snapshots (seia is a CLI, not a terminal renderer)
- fix: prelude use paths + root re-exports
- docs: add PLAN.md current-status snapshot
- docs: large-format search snapshots (110×50)
- docs: standardize License section format across all translations

### 本次完成

- **docs.rs 徽章**：在 `README.md` badge 区添加官方 `[![docs.rs]](https://docs.rs/seia)` 徽章。
- **发布元数据**：在 `Cargo.toml` 补 `[package.metadata.docs.rs]` 配置块（`all-features = true`）。`keywords` / `categories` 此前已存在，确认完整。

## 5. 后续计划

1. 完善 `crates.io` 发布流程（首次 publish 前检查 `cargo publish --dry-run`）。
2. 补充单元/集成测试，保持 `just test` 与 clippy `-D warnings` 通过。
3. **不要**再往 seia 塞终端快照 / PNG；需要快照请去 `kou`。定期刷新本 PLAN.md 以反映最新状态。

## 6. 验证

- `cargo check`：通过。
- `cargo clippy -- -D warnings`：通过。
- `cargo test --lib`：通过（21 passed; 0 failed）。
