# quanttide-work-lab

量潮知识工作内核——quanttide-lab-toolkit 的实验版。

实验室先试、试成扶正：内核的模型与规矩先落这儿，验过的再并进 quanttide-lab-toolkit。本包只做实验，不承诺稳定，也不发布（`publish = false`）。

## 是什么

知识工作这件事本身——工件、过程、场所的模型，判据与判法，进度与凭证的推导。只进值、只出值：不碰文件、不起进程、不发 HTTP、不取时刻。碰边界的活留在命令行那边（`src/cli/`）。

现在只有骨架，模型待建。

## 文档

- [本包开发指南](docs/dev-guide/index.md)——界在哪、结构、验收；
- [仓库开发指南](../../docs/dev-guide/index.md)——core 与 local 两层、cli 与飞书。

## 装与跑

```bash
cargo test      # 本目录下
```

## 依赖与许可

`Cargo.toml` 的每个直接依赖列在这里，写明用途与许可；新增依赖先入表。暂无直接依赖。

## 许可

Apache-2.0
