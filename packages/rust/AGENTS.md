# AGENTS.md - quanttide-work-lab Rust 库

## 项目结构

```
packages/rust/
├── AGENTS.md          # 本文件
├── Cargo.toml         # Rust 包配置
├── README.md          # 项目说明
├── docs/dev-guide/    # 本包自己的开发指南
├── src/
│   └── lib.rs         # 库入口
└── tests/
    └── package.rs     # 集成测试
```

## 事实源

领域模型与规格以 quanttide-work 的 `docs/specification` 为准；本库只做表达，不定义领域。收进来的每一样都要能在规格里找到出处。

## 提交约定

Conventional Commits（`feat:` / `fix:` / `docs:` / `chore:`）；破坏性变更标 `!` 并在 body 说明迁移方式。

## 测试

```bash
cargo fmt --check && cargo clippy --all-targets --locked -- -D warnings && cargo test --locked
```
