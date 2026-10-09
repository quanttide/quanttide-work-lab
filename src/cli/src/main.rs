//! quanttide-work-lab-cli：qtcloud-work 的实验版。
//!
//! 实验室先试、试成扶正：规格的新写法、工具箱的接入、命令面的改动都先落这儿。
//! 本包只做实验，不承诺稳定；命令面待建。

use clap::Parser;

/// 量潮知识工作实验室命令行——qtcloud-work 的实验版。
#[derive(Parser)]
#[command(name = "quanttide-work-lab", version, about)]
struct Cli {}

fn main() {
    Cli::parse();
    println!("quanttide-work-lab：qtcloud-work 的实验版，命令面待建。");
}
