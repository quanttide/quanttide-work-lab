# quanttide-work-lab-cli

量潮知识工作实验室命令行——[qtcloud-work](https://github.com/quanttide/qtcloud-work) 的实验版。

实验室先试、试成扶正：规格的新写法、工具箱的接入、命令面的改动都先落这儿，验过的再并进 qtcloud-work。本包只做实验，不承诺稳定，也不发布（`publish = false`）。

## 装与跑

```bash
cargo run -- --help      # 本目录下
```

## 依赖与许可

`Cargo.toml` 的每个直接依赖列在这里，写明用途与许可；新增依赖先入表。

| 依赖 | 版本 | 许可 | 用途 |
| :-- | :-- | :-- | :-- |
| `clap` | 4 | MIT OR Apache-2.0 | 命令行解析（derive） |

## 许可

Apache-2.0
