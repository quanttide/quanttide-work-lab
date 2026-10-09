# 开发指南

`quanttide-work-lab` 这个包怎么长。仓库整体的分层——core 与 local 两层怎么切、命令行那边有什么——看[仓库开发指南](../../../../docs/dev-guide/index.md)；这里只说内核这半边。

## 这个包是干什么的

把知识工作的规矩写成可执行的正本：工件、过程、场所的模型，判据与判法，进度与凭证的推导。命令行、工作台、后来的端都向它对齐，不各写一份。

给它一张工作流、一件工单、一份「AI 这么答」，它就能算出下一步是哪一步、这一步过没过、进度到哪儿——不用碰文件，也不用起进程。

## 一条界：只进值、只出值

进来的：已经读出来的文本或值、此刻几点、新凭证是什么、当前目录在哪。出去的：算好的结果，或一条读不通的错误。

包里出现 `std::fs`、`std::process`、`std::env`、`now()`、`new_id()` 就是越界；认得命令行那边（`quanttide_work_lab_cli` 或任何入口层的名字）也是越界。位置装载、事件落盘、起 `pi`、探活、飞书这些碰边界的活，全在 `src/cli/`。

这条界是测法，不是洁癖：值给全了就能算出结论，测一条记账规矩不用先搭一个真文件系统。

## 件怎么分

判据是有没有自己的定义：有自己的定义（身份、生命周期、字段规矩）就单开一个聚合目录；没有、只跨聚合做一件事就进服务；谁都用、不属谁的进中立件。

```text
src/
├── lib.rs         出口
├── order/         聚合：工单（model / record / progress / actions）
├── workflow/      聚合：工作流（model / read / check）
├── catalog/       聚合：目录
├── artifact/      聚合：资产表、产物实例、落点规矩
├── material/      聚合：材料
├── workspace/     聚合：工作区身份与字段规矩
├── search/        服务：按名找文档
├── audit/         服务：审计
├── criterion/     中立：判据（模型 / 读法 / 翻成要跑什么）
├── prompts.rs     中立：给智能体的话术
└── outcome.rs error.rs fields.rs executor.rs paths.rs ids.rs sha1.rs
```

名字用能力名，不造 `-er`：`search` 不叫 `searcher`。依赖单向：服务可以认聚合，聚合不认服务。

## 事实源

收进来的每一样都要能在规格里找到出处；规格里没有的，先补规范再写码。规格在 quanttide-work 的 `docs/specification`。

## 现在到哪了

骨架：`lib.rs` 里只有领域名与版本，上面列的件待建。

## 验收

```bash
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
```

三条都绿才算过。
