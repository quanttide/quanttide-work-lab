//! quanttide-work-lab：知识工作内核的实验版。
//!
//! 实验室先试、试成扶正：内核的模型与规矩先落这儿，验过的再并进
//! quanttide-lab-toolkit。本包只做实验，不承诺稳定。
//!
//! 界在这里：只进值、只出值——不碰文件、不起进程、不发 HTTP、不取时刻、
//! 不读环境。碰边界的活（位置装载、事件落盘、起 pi、探活、飞书）留在命令行
//! 那边（`src/cli/`），由它取好递进来。见 `docs/dev-guide/index.md`。
//!
//! 现在只有骨架，模型待建。

/// 领域英文名。
pub const DOMAIN: &str = "knowledge-work";

/// 包版本（取自 `Cargo.toml`）。
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
