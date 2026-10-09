# 开发指南

lab CLI 怎么搭：内核（core）与本机（local）分作两个包——core 出 `packages/rust/`，local 就是 `src/cli/`。命令面与用法另说。

这是 lab 里的设计稿——先在这儿立起来试，验过的再扶正：core 扶正为工具集，local 扶正进 qtcloud-work。

## 为什么分两层

现行 qtcloud-work CLI 把「知识工作的规矩」和「这台机器怎么干活」搅在一起。`order/` 里直接 `std::fs` 读工单、扫目录、写回（`order/mod.rs` 的 `open`、`listing`、`Order::save`）；聚合里直接取时刻、造凭证（`crate::clock::now()`、`crate::ids::new_id()`）；`workspace/local.rs` 一身两职，既装位置又夹在聚合里；起进程（`adapters/pi.rs`）、跑 `run` 判据（`order/rules.rs`）、发 HTTP（`health.rs`）散在四处。想测一条记账规矩，得先搭一个真文件系统；想把账本从本地文件换成远端，得进聚合改。

分两层是把这条线画出来：规矩归 core，机器归 local。

core 还得挪出这个命令——同一套规矩要供 studio 和别的端用。挪出包，它就成了 lab 版的工具集（`quanttide-lab-toolkit`）。

## 层即包

core 是知识工作这件事本身——工件、过程、场所的模型，判据与判法，进度与凭证的推导。它只进值、只出值：进的是已经读出来的文本或值，出的是一份算好的结果或一条读不通的错误。

local 是这台机器——文件在哪、进程怎么起、网怎么通、飞书怎么取、结果打印回哪儿。它把 core 算好的东西落下来，也把落下来的东西读出来交给 core。

一条边界先记住：core 里出现 `std::fs`、`std::process`、`std::env`、`now()`、`new_id()` 就是越界。此刻几点、新凭证是什么、当前目录在哪，由 local 取好递进来。方向也单向：`src/cli` 认 `packages/rust`，反过来不认，core 里出现 `crate::cli` 就是越界。

层名写在包边界上——`packages/rust/` 就是 core，`src/cli/` 就是 local，不另开 `core/`、`local/` 目录。

## 落点

```text
packages/rust/         core：quanttide-lab-toolkit 的实验版（Rust 库）
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── order/         聚合：工单（model / record / progress / actions）
│   ├── workflow/      聚合：工作流（model / read / check）
│   ├── catalog/       聚合：目录
│   ├── artifact/      聚合：资产表、产物实例、落点规矩
│   ├── material/      聚合：材料
│   ├── workspace/     聚合：工作区身份与字段规矩
│   ├── search/        服务：按名找文档
│   ├── audit/         服务：审计
│   ├── criterion/     中立：判据（模型 / 读法 / 翻成要跑什么）
│   ├── prompts.rs     中立：给智能体的话术
│   └── outcome.rs error.rs fields.rs executor.rs paths.rs ids.rs sha1.rs
└── tests/

src/cli/               local：命令行（Rust 二进制，path 依赖 packages/rust）
├── Cargo.toml
└── src/
    ├── main.rs        一行：把命令行交给 cli
    ├── cli.rs cli/    入口：clap 定义、命令树、分派、发射
    ├── help.rs        导览
    ├── fs.rs fs/      文件：位置装载、读定义、读账、写账、事件落盘
    ├── process.rs     进程：起 pi、跑 run 判据
    ├── http.rs        网：provider 探活
    ├── feishu.rs      飞书：起本机 lark-cli，把飞书件取成材料
    └── clock.rs ids.rs  取此刻、发新凭证
```

local 的件数看着少，是按边界分的结果——边界只有文件、进程、网、飞书加一个入口。按行数它占现行全仓（5226 行）约三成：入口与命令面约 690，文件约 440，进程约 134，网 48，时刻与凭证约 35。core 条目多，是知识工作的模型本来就多。

时刻与新凭证是进 core 的输入：`clock.rs` 里取此刻的那半、`ids.rs` 里发新凭证的那半在 local；凭证的派生（给定名字按 uuid5 算 id）是纯的，留 core。

## 飞书模块

飞书模块管一件事：把飞书里的件取成本地材料。它起本机飞书命令行 `lark-cli`，按链接认出是文档、表格还是消息，取回内容，交给 core 的 `material` 认成材料，再由 `fs` 落盘。

飞书的名字、命令、字段名都留在这一件里，core 只认「材料」——材料是领域概念（四字段与阶段），飞书只是它的一条来路。哪天飞书的接口换了，只动 `feishu.rs`。

命令面上建议先接在现有的 `material` 上：材料收录多认一种来源（给一条飞书链接）。等真出现飞书专属动作（回条消息、发份文档）再单起 `feishu` 子命令。

本机飞书命令行是 `lark-cli`：`lark-cli <域> --help` 看命令和快捷方式，`--dry-run` 只预演、`--jq` 收 JSON；标 high-risk-write 的动作要 `--yes`，得先问过人。

## 走一趟：order next

拿现行最全的一条命令当例子：`order next AI冒烟`——把「AI冒烟」这件工单的下一步交给智能体做。敲下命令后，两包这样交替：

1. `src/cli` 打开账本：读 `<账本>/workorders/AI冒烟.yaml`，读它引的那条工作流定义。
2. `packages/rust` 认这张账：YAML 读成工单与工作流，字段合不合规矩在这里判，读不通就吐一条错误回去。
3. `packages/rust` 算下一步：走过哪几步、停在哪儿、下一步是哪一步。这步是 AI 站的，组装现场——这一步要做什么、报告与日志落在哪、前几笔流水是什么、这一步有哪些判据，占位已换成本单的真路径。
4. `src/cli` 起 `pi` 把这段话跑掉，把回答收成文本递回。
5. `packages/rust` 给判据分两路：`agent` 判据翻成「照这条判准审一遍」的话术，`rule` 判据翻成「要查哪个路径、要跑哪条命令」。
6. `src/cli` 真去查、真去跑：文件在不在、命令退出码是不是零，把结果递回。
7. `packages/rust` 定这一笔：过没过、写什么说明、发哪条领域事件。
8. `src/cli` 取此刻、发凭证，把工单与事件落盘。
9. `packages/rust` 算进度行，`src/cli` 把它打到屏幕上。

这一趟里，`packages/rust` 没见过一个路径、没起过一个进程、没问过一次现在几点。

## 从现行 CLI 挪过来

聚合与中立件进 `packages/rust`：`order/`、`workflow/`、`catalog/`、`artifact/`、`material/`、`workspace/`（去掉装载那半）、`search/`、`audit/`、`criterion/`，加 `outcome.rs`、`error.rs`、`fields.rs`、`executor.rs`、`paths.rs`、`sha1.rs`、`prompts.rs`。

碰边界的件进 `src/cli`：`cli.rs`、`cli/`、`help.rs`；`workspace/local.rs`、`events.rs` 进 `fs/`；`adapters/pi.rs` 与 `order/rules.rs` 里真去跑的那半进 `process.rs`；`health.rs` 进 `http.rs`；新写的飞书件进 `feishu.rs`。

聚合里的盘活逐个上缴：`order/mod.rs` 的 `open`、`listing`、`save`、`delete` 改成 local 读、core 认、local 写；`order/execute.rs` 的取时刻、造凭证、跑判据改成收参吐值。

一次挪一条命令，挪完 `cargo test` 全绿，命令面与输出（`ok` / `lines` / `columns` / `rows` / `data`）一字不改。

## 留到下一轮

- 飞书模块的接口面：先只做「取材料」，回写（发文档、回消息）等有真需求再接。
- 两包要不要共一个 Cargo 工作区（根 `Cargo.toml`），还是各留各的锁。
- studio 那侧要不要共用 core：core 是 Rust、studio 是 Dart，真共用要么各留一份薄的、要么 core 出个跨进程的口子。
- `packages/rust/` 的包名与发布线：`quanttide-lab-toolkit` 这个仓库在量潮的子模块里没找到，名字与归属归你定。
