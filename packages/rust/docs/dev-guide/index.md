# 开发指南

`quanttide-work-lab` 这个包怎么长。仓库整体的分层——core 与 local 两层、命令行与飞书——看[仓库开发指南](../../../docs/dev-guide/index.md)；这里只说内核这半边。

## 是什么

知识工作这件事本身——工件、过程、场所的模型，判据与判法，进度与凭证的推导。它是 lab 版的工具集内核，将来扶正为 quanttide-lab-toolkit。

## 界在哪

只进值、只出值。包里出现 `std::fs`、`std::process`、`std::env`、`now()`、`new_id()` 就是越界——那些由命令行那边取好递进来。也不认命令行：出现入口层的名字就是越界。

碰边界的活——位置装载、事件落盘、起 `pi`、探活、飞书——全在 `src/cli/`。件怎么摆见[仓库开发指南](../../../docs/dev-guide/index.md)·落点。

## 现在到哪了

骨架：`src/lib.rs` 只有领域名与版本，模型待建。

## 验收

```bash
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
```
