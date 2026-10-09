# 执行者

这一步谁做、这一条判据谁判。内核里最底下的一件：它不依赖别的件，`criterion`（判据）与 `error`（错误）反过来依赖它——读这一篇不用先看别的。

## 三个取值

- `agent`——智能体：交给 AI 做、交给 AI 判；
- `rule`——规则引擎：程序当场核，机械比对；
- `human`——人：只在闸门拍板。

常量在 `src/executor.rs`：`AGENT` / `RULE` / `HUMAN`。用的时候比常量，不写字面量——写 `auto`、`ai` 这类同义词，读定义时不认，与别端对表也对不上。

比如一条三步的工作流：第一步 `agent` 起草、第三步 `human` 拍板；每一步挂着的判据里，可能还夹一条 `rule` 当场核报告在不在。

## 两组名单

同样三个取值，在步骤上和判据上能写的不同：

- 步骤的执行者（`EXECUTORS`）——只能 `agent` 或 `human`：能用 AI 都用 AI，人只在闸门；
- 判据的执行者（`CRITERION_TYPES`）——三样都能写：多出来的那个 `rule` 是程序当场核。

名单在内核，拿它校验定义也在内核（读定义的时候核）。真去跑在内核外：`rule` 去查文件、起进程，`agent` 由执行器交给智能体，`human` 停在闸门等人。

## 出处

规格的 `performer/` 一组——导言加 `agent` / `human` / `rule-engine` 三篇，在 quanttide-work 的 `docs/specification/performer/`。取值不因平台而变，改它不是改代码，是改规格。

## 验收

```bash
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
```
