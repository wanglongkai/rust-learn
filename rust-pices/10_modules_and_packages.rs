// ============================================================
// 第 10 课：模块与包 (Modules, Crates & Packages)
// ------------------------------------------------------------
// 学习目标：
//   1. 理解 package / crate / module 三层概念。
//   2. 掌握 mod / pub / use 的用法，写出有清晰边界的代码。
//   3. 知道 Cargo 工程的典型目录结构与常用命令。
// 前置知识：第 07、08 课。
// 本课覆盖：mod / pub / use / crate / super / Cargo 工程结构。
// 运行方式：
//   rustc 10_modules_and_packages.rs && ./10_modules_and_packages
//   (本文件用 inline mod 演示模块系统；真实项目中模块通常拆成多个文件)
// ============================================================
//
// === 三层概念 ===
//   package（包）   ：一个 Cargo.toml 管辖范围，包含一个或多个 crate。
//   crate（编译单元）：一个二进制 (bin) 或库 (lib)。一个 package 至少有一个 crate。
//   module（模块）   ：crate 内部的命名空间，用来组织代码。
//
//   ┌────────────────── package ──────────────────┐
//   │ Cargo.toml                                   │
//   │ ┌─── crate (lib) ───┐  ┌── crate (bin) ──┐  │
//   │ │  src/lib.rs       │  │  src/main.rs    │  │
//   │ │   ├─ mod a        │  │   uses lib...   │  │
//   │ │   └─ mod b        │  └─────────────────┘  │
//   │ └───────────────────┘                       │
//   └──────────────────────────────────────────────┘

// ------------------------------------------------------------
// 1. 用 mod 在当前文件内联定义模块
// ------------------------------------------------------------
mod front_of_house {
    // 默认私有！没加 pub 的子模块外部访问不到。
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("[hosting] 加入等位列表");
        }

        pub fn seat_at_table() {
            println!("[hosting] 安排入座");
            // 同模块内部不需要 pub 也能互相调用
            internal_check();
        }

        // 没有 pub：外部访问不到，只在 hosting 模块内可用
        fn internal_check() {
            println!("[hosting] (内部) 检查桌位");
        }
    }
}

mod back_of_house {
    // ------------------------------------------------------------
    // 2. struct 的可见性：struct 本身 pub，字段默认仍是私有。
    //    要让某个字段对外可写/可读，需要单独加 pub。
    // ------------------------------------------------------------
    pub struct Breakfast {
        pub toast: String,          // 对外公开
        seasonal_fruit: String,     // 私有
    }

    impl Breakfast {
        // 由于 seasonal_fruit 私有，外部无法直接构造 Breakfast，
        // 必须通过这种"构造函数"获得，从而保证不变量。
        pub fn summer(toast: &str) -> Self {
            Self {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }

        pub fn describe(&self) {
            println!(
                "[back_of_house] 早餐: toast={}, fruit={}",
                self.toast, self.seasonal_fruit
            );
        }
    }

    // ------------------------------------------------------------
    // 3. enum 的可见性：enum 加 pub 后，所有变体都自动 pub。
    // ------------------------------------------------------------
    #[allow(dead_code)]
    pub enum Appetizer {
        Soup,
        Salad,
    }

    // ------------------------------------------------------------
    // 4. super：访问父模块；crate：访问 crate 根。
    // ------------------------------------------------------------
    pub fn fix_incorrect_order() {
        cook_order();
        // 调用兄弟模块的函数：
        super::front_of_house::hosting::seat_at_table();
        // 也能写成绝对路径：
        // crate::front_of_house::hosting::seat_at_table();
    }

    fn cook_order() {
        println!("[back_of_house] 重新做菜");
    }
}

// ------------------------------------------------------------
// 5. use：把长路径引入当前作用域，让代码更短。
// ------------------------------------------------------------
use crate::front_of_house::hosting;
// use as：处理同名冲突
use std::io::Result as IoResult;
// 集合导入
use std::collections::{HashMap, HashSet};

fn _example_signature() -> IoResult<()> {
    let _m: HashMap<i32, i32> = HashMap::new();
    let _s: HashSet<i32> = HashSet::new();
    Ok(())
}

fn main() {
    // ---- 路径写法的三种风格 ----
    // 绝对路径（从 crate 根开始）
    crate::front_of_house::hosting::add_to_waitlist();
    // 相对路径
    front_of_house::hosting::add_to_waitlist();
    // 通过 use 引入后的短路径
    hosting::seat_at_table();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat"); // 字段 pub，可改
    // meal.seasonal_fruit = String::from("apple"); // 错：字段私有
    meal.describe();

    back_of_house::fix_incorrect_order();
}

// ============================================================
// 真实项目的目录与 Cargo.toml 示例（仅文字示意，不在此运行）
// ------------------------------------------------------------
// my_project/
// ├── Cargo.toml
// ├── src/
// │   ├── main.rs                 ← bin crate 的入口 (fn main)
// │   ├── lib.rs                  ← lib crate 的入口（可选）
// │   ├── config.rs               ← 一个独立模块（mod config;）
// │   └── http/
// │       ├── mod.rs (或 http.rs) ← mod http 的入口
// │       ├── client.rs           ← pub mod client
// │       └── server.rs
// ├── tests/                      ← 集成测试，每个文件一个 crate
// │   └── api.rs
// └── examples/                   ← 示例程序
//     └── hello.rs
//
// Cargo.toml 关键字段：
//   [package]
//   name = "my_project"
//   version = "0.1.0"
//   edition = "2021"
//
//   [dependencies]
//   serde = { version = "1", features = ["derive"] }
//   anyhow = "1"
//
//   [dev-dependencies]
//   pretty_assertions = "1"
//
// 常用 Cargo 命令：
//   cargo new my_app          # 创建二进制项目
//   cargo new my_lib --lib    # 创建库项目
//   cargo build               # 编译
//   cargo run                 # 编译并运行（默认 main bin）
//   cargo test                # 跑所有测试
//   cargo check               # 只做类型检查，不生成可执行（更快）
//   cargo fmt                 # 格式化
//   cargo clippy              # lint
//   cargo add anyhow          # 添加依赖（Cargo 1.62+）
//   cargo update              # 升级依赖到 Cargo.lock 允许的范围
//   cargo doc --open          # 生成并打开文档
// ============================================================

// ============================================================
// 本课小结
//   - package = Cargo.toml 管辖；crate = 一个编译单元；module = crate 内命名空间。
//   - 默认私有，pub 才公开；struct 字段、enum 变体的可见性需单独控制。
//   - use 让长路径变短，as 处理重名，{} 批量导入。
//   - 真实项目按文件 / 文件夹拆模块，配合 mod 声明组装成树。
//
// 常见陷阱
//   - mod foo; 没在 main.rs / lib.rs / 父 mod 里声明 → 找不到模块。
//   - 把所有东西都 pub → 失去封装，重构时改动牵一发动全身。
//   - struct 加了 pub 就以为字段也公开 → 字段需要单独 pub。
//   - 路径混乱时用绝对路径 (crate::...) 最稳。
//
// 最佳实践
//   - 默认私有，最小化 pub 范围；仅对真正需要对外的项加 pub。
//   - 内部跨模块复用用 pub(crate)；只让父模块用 pub(super)。
//   - 模块层次反映领域边界，不要按"工具/常量/类型"机械划分。
//   - 把公开类型集中 re-export，比如在 lib.rs 写 pub use crate::http::Client;
//
// 思考题
//   1. lib crate 和 bin crate 在工程上有什么区别？为什么常常同时存在？
//   2. pub(crate) / pub(super) / pub(in path) 三者的差别？
//   3. 为什么 enum 加 pub 后变体就全 pub，而 struct 的字段不是？
// ============================================================
