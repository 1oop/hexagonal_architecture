# {{project-name}}

{{description}}

使用 Rust **2024 版本**构建，遵循**六边形架构**（端口与适配器模式）。

## 架构概览

项目结构在 `crates/` 目录下划分为多个独立的 Crate，以强制执行严格的架构边界：

- **`crates/domain` (领域层)**: 
  - **核心**: 包含纯业务逻辑、领域实体和定义的错误类型。
  - **依赖**: 无。完全隔离了框架和外部库。
- **`crates/application` (应用层)**: 
  - **编排**: 定义与外部通信的“端口”（Traits）以及协调领域逻辑的“服务”（用例）。
  - **依赖**: `domain`。
- **`crates/infrastructure` (基础设施层)**: 
  - **适配器 (出站)**: 实现应用层定义的端口（例如通过 SQLx 实现的数据库仓库、Redis 客户端、外部 HTTP 客户端）。
  - **依赖**: `domain`, `application`。
- **`crates/api` (接口/入口层)**: 
  - **适配器 (入站)**: 应用程序的入口。处理交付逻辑（Axum REST API、CLI、任务运行器）以及**依赖注入**（将适配器注入到服务中）。
  - **依赖**: `domain`, `application`, `infrastructure`。

## 依赖原则

`API/Infrastructure` -> `Application` -> `Domain`

核心业务逻辑保持稳定，不感知外部世界的架构细节（如数据库选型、Web 框架等）。

## 环境准备

- [Rust](https://www.rust-lang.org/) (2024 版本需要较新版本的编译器)
- [cargo-generate](https://github.com/cargo-generate/cargo-generate)

## 如何开始

### 1. 使用模板生成项目
```bash
cargo generate --path /path/to/template --name my-awesome-project
```

### 2. 运行 API 服务
```bash
cargo run -p api
```

### 3. 正式环境编译
```bash
cargo build --release
```

## 项目规范

- **Rust 版本**: 2024
- **工作区管理**: 共享依赖在根目录 `Cargo.toml` 中统一管理。
- **错误处理**: 应用层使用 `anyhow`，领域层/库逻辑使用 `thiserror`。
- **异步运行时**: `tokio`。

---
由 [Hexagonal Rust Template] 驱动
作者: {{author}}
