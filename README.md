# rust-learn · Rust 学习路线

一份"对初学者最友好"的 Rust 自学仓库：

- 用 16 节单文件小程序，从变量讲到 async，循序渐进。
- 配套两份导引文档，帮你建立**整体认知**与**实战最佳实践**。
- 全部代码可独立用 `rustc` 编译运行，无需 Cargo 工程。

---

## 从这里开始

1. **先读** [`rust-pices/00_overview.md`](./rust-pices/00_overview.md)
   - Rust 是什么、擅长什么、设计哲学、配套工具链
   - 16 节课的完整路线图
2. **再跟着课**：`rust-pices/01_variables_and_mutability.rs` → ... → `rust-pices/16_async_await.rs`
3. **学完后翻一遍** [`rust-pices/BEST_PRACTICES.md`](./rust-pices/BEST_PRACTICES.md)
   - 工程化 / 命名 / 所有权 / 错误处理 / API 设计 / 异步 / 并发等实战清单

---

## 目录结构

```
rust-learn/
├── README.md                ← 本文件，学习入口
├── Cargo.toml               ← 顶层 Cargo 工程（演示用，可忽略）
├── src/main.rs              ← Cargo 工程的入口，对应第 01 课内容
└── rust-pices/              ← 16 节核心教程
    ├── 00_overview.md                       Rust 整体认知 + 路线图
    ├── BEST_PRACTICES.md                    实战最佳实践清单
    ├── 01_variables_and_mutability.rs       变量、mut、const、shadowing
    ├── 02_data_types_scalar_and_compound.rs 标量 / 元组 / 数组
    ├── 03_functions_and_scope.rs            函数 / 表达式 / 作用域
    ├── 04_control_flow.rs                   if / loop / while / for / range
    ├── 05_ownership_and_borrowing.rs        所有权三定律 / move / clone / Copy
    ├── 06_references_and_slices.rs          引用规则 / &str / &[T]
    ├── 07_structs_and_methods.rs            struct / impl / 关联函数 / newtype
    ├── 08_enums_and_pattern_matching.rs     enum / Option / match / if let
    ├── 09_common_collections.rs             Vec / String / HashMap
    ├── 10_modules_and_packages.rs           mod / pub / use / Cargo 工程
    ├── 11_error_handling.rs                 Result / ? / 自定义错误
    ├── 12_generics.rs                       泛型 / trait bound / 单态化
    ├── 13_traits_and_trait_bounds.rs        trait / impl Trait / dyn Trait
    ├── 14_lifetimes.rs                      生命周期标注 / 省略规则
    ├── 15_standard_library_and_macros.rs    宏 / 迭代器 / Option Result 组合子
    └── 16_async_await.rs                    async / await / Future / 运行时
```

---

## 怎么运行每节课

每节课都是**单文件可独立编译**的。

任选其一：

```bash
# 方法 1：用 rustc 单文件编译（轻量，不需要 Cargo 工程）
cd rust-pices
rustc 01_variables_and_mutability.rs
./01_variables_and_mutability        # macOS / Linux
.\01_variables_and_mutability.exe    # Windows

# 方法 2：用顶层 Cargo 工程运行 src/main.rs
cargo run
```

---

## 推荐学习节奏

| 节奏 | 用时 | 适合谁 |
|---|---|---|
| 一节课 30 分钟，每天 2 节 | 8 天 | 已有任何一门系统级语言（C/C++/Go）经验 |
| 一节课 1 小时，每天 1 节 | 16 天 | 之前只写过 Python / JS / Java |
| 周末集中 2 节 + 周中复盘 | 2 个月 | 业余学习者 |

每节课的"思考题"和"常见陷阱"区域，建议读完代码后回头再过一遍。

---

## 学完之后

- **CLI 工具**：clap + anyhow + indicatif，做一个像样的命令行工具。
- **Web 后端**：axum + tokio + sqlx，写一个有数据库的 REST API。
- **WebAssembly**：wasm-bindgen + wasm-pack，把 Rust 跑进浏览器。
- **进阶资料**：The Rust Programming Language（中文版译作《Rust 程序设计语言》）、Rust by Example、《Rust 异步编程》。

祝旅途愉快。
