# seia — 项目状态与计划 (PLAN)

> 本文件由自动化扫描于 **2026-07-04** 生成，记录项目当前状态、近期进展与后续计划。

## 1. 项目概述

- **名称**：`seia`
- **简介**：多引擎网页搜索库。
- **远程仓库**：git@github.com:celestia-island/seia.git
- **技术栈**：Rust / just
- **类别**：rust-lib

## 2. 当前状态

- **当前分支**：`dev`
- **工作区**：有改动（见下）
- **最近提交时间**：2026-07-04
- **最近提交**：fix: remove bogus res/ snapshots (seia is a CLI, not a terminal renderer)
- **分支对比**：`dev` 领先 `master` 61 个提交

## 3. 未提交改动

- `src/lib.rs`：修复 `prelude` 中残留的 Rust 2015 风格 `use`（`cargo build` 失败的根因），并在 crate 根重新导出 `SearchClient` / `SearchOptions` / `Engine` 等，使 `seia::Engine` 等对外路径可用。
- 删除 `res/search_campbell.png`、`res/search_solarized_dark.png`：seia 是多引擎搜索库 / CLI，**不具备终端渲染能力**，不应附带 PNG 快照。真正能渲染终端快照的是同仓群下的 `kou`（VT 引擎）。此前由自动化脚本误把快照提交进了 seia。
- 移除 `README.md` 中对应的 `<details>` Screenshots 区块。

## 4. 近期进展（最近提交）

- fix: remove bogus res/ snapshots (seia is a CLI, not a terminal renderer)
- fix: prelude `use` paths + root re-exports (build was broken)
- docs: add PLAN.md current-status snapshot
- docs: standardize License section format across all translations
- docs: standardize language bar to markdown format

## 5. 后续计划

1. 完善文档示例与 `crates.io` 发布元数据（rust-version / metadata / docs.rs badge）。
2. 补充单元/集成测试，保持 `just test` 与 clippy `-D warnings` 通过。
3. **不要**再往 seia 塞终端快照 / PNG；需要快照请去 `kou`。定期刷新本 PLAN.md 以反映最新状态。

