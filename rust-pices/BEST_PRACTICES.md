# Rust 实际开发最佳实践

> 学完 16 节基础课后，这份清单告诉你"在真实项目里，老 Rust 程序员到底怎么写"。
> 每条都尽量给出"该怎么做 / 为什么 / 反例"。

---

## 一、项目工程化

### 1.1 用 Cargo，不要直接用 rustc
- 任何稍微正经一点的项目都用 `cargo new`，依赖、构建、测试、发布全交给 Cargo。
- `rustc` 只在学习单文件示例（比如本仓库的 `rust-pices/*.rs`）时直接用。

### 1.2 必备三件套：fmt + clippy + test
```bash
cargo fmt --all              # 统一代码风格
cargo clippy --all-targets -- -D warnings   # 把 lint 警告当错误
cargo test                   # 单测 + 文档测试
```
在 CI 里把这三条全部加上，新人 PR 自然就规范了。

### 1.3 善用 workspace 拆分大项目
当一个项目超过 ~5k 行，就考虑拆 workspace：
```toml
# Cargo.toml (根)
[workspace]
members = ["crates/core", "crates/cli", "crates/server"]
```
拆分原则：**按功能边界拆，不按文件大小拆**。

### 1.4 锁定 Rust 版本
项目根目录加 `rust-toolchain.toml`：
```toml
[toolchain]
channel = "1.80.0"
components = ["clippy", "rustfmt"]
```
保证团队成员、CI 用同一个版本。

---

## 二、命名与代码风格

| 类别 | 风格 | 示例 |
|---|---|---|
| 变量、函数、模块 | `snake_case` | `user_name`、`fn read_file()` |
| 类型（struct/enum/trait） | `PascalCase` | `struct UserId`、`trait Summary` |
| 常量、静态变量 | `SCREAMING_SNAKE_CASE` | `const MAX_RETRY: u32 = 3;` |
| 生命周期 | 短小写字母 | `'a`、`'ctx` |
| 泛型参数 | 单大写字母或 PascalCase | `T`、`K`、`Item` |

其它约定：
- 文件名用 `snake_case.rs`，模块文件 `mod.rs` 现在已不推荐，优先 `src/foo/mod.rs` → `src/foo.rs + src/foo/...`。
- 4 空格缩进，行宽 100，由 `rustfmt` 自动处理，**别手动调**。

---

## 三、所有权与借用

### 3.1 函数参数优先用引用，不要随便接受 `String`
```rust
// 好：调用方既能传 &str 也能传 &String
fn greet(name: &str) { println!("hi {name}"); }

// 不好：强制调用方交出所有权，复用性差
fn greet_bad(name: String) { println!("hi {name}"); }
```
经验法则：**只读用 `&T`，要修改用 `&mut T`，需要持有/存储才用 `T`**。

### 3.2 字符串参数：`&str` 优先于 `&String`，`&[T]` 优先于 `&Vec<T>`
切片是更通用的"借用视图"，几乎所有上层类型都能 Deref 成切片。

### 3.3 别用 `.clone()` 治百病
能编译过不代表写得好。看到自己疯狂 `.clone()` 时，先问：
- 是不是引用就够了？
- 是不是该让所有权流转下去（move）而不是复制？
- 数据是不是该用 `Arc<T>` 在多处共享？

### 3.4 不要返回引用某个局部变量（编译器会拦下，但要懂为什么）
返回值要么是拥有所有权的 `T`，要么是引用入参的引用（生命周期由入参决定）。

### 3.5 内部可变性：Cell / RefCell / Mutex 按需选
- 单线程、`Copy` 类型小数据 → `Cell<T>`
- 单线程、运行期借用检查 → `RefCell<T>`
- 多线程共享 → `Mutex<T>` / `RwLock<T>`，外面一般再套 `Arc<...>`

---

## 四、错误处理

### 4.1 库代码用 `thiserror`，应用代码用 `anyhow`
- **库（要被别人 use 的）**：用 `thiserror` 定义具体错误枚举，让上层能精确 `match`。
- **应用（最终二进制）**：用 `anyhow::Result<T>`，统一返回 `anyhow::Error`，能附加 context。
```rust
// 应用代码
use anyhow::{Context, Result};

fn load_config(path: &str) -> Result<Config> {
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("读取配置失败: {path}"))?;
    let cfg = toml::from_str(&text).context("解析配置失败")?;
    Ok(cfg)
}
```

### 4.2 优先用 `?`，少用 `unwrap()`
- 业务代码里出现 `unwrap()` / `expect()` 几乎都是坏味道，留给：
  - 单测 / 示例代码
  - 程序启动期的"不可恢复"配置加载（也建议 `expect("说明为什么 panic")`）
- `?` 自动把错误转换并向上传播，配合 `From` trait 极其顺手。

### 4.3 不要把 `Result` 当流程控制
不要写：
```rust
if let Ok(v) = parse() {
    use_it(v);
} else {
    // 当作 fallback
}
```
要明确：这里到底是"失败应该返回"还是"失败有合理 fallback"？前者用 `?`，后者用 `unwrap_or`/`unwrap_or_else`。

### 4.4 panic! 只用于"代码 bug"
不是"网络断了"、"用户输入错了"用 panic，那些是 `Result`。
panic! 给"理论上不可能、出现就是 bug"的情况，比如索引断言、`unreachable!()`。

---

## 五、类型设计

### 5.1 用 newtype 包裹基础类型，提升语义
```rust
struct UserId(u64);
struct OrderId(u64);
// 编译器现在能阻止你把 UserId 当 OrderId 传过去
```
对所有"业务 ID"、"金额"、"距离"等强烈推荐这么做。

### 5.2 用 enum + match 替代标志位 / 字符串状态
```rust
// 不好
let status = "pending"; // 字符串容易拼错、状态难穷举

// 好
enum OrderStatus { Pending, Paid, Shipped, Refunded }
```
`match` 会强制你处理所有分支，新增状态时编译器会提醒所有遗漏的地方。

### 5.3 把"不可能的状态"在类型里就消除掉
经典例子：
```rust
// 不好：name 是否填写 + 是否登录 是两个 bool，4 种组合里有 2 种是非法的
struct User { name: Option<String>, logged_in: bool }

// 好：用 enum 直接表达合法状态
enum User {
    Guest,
    LoggedIn { name: String },
}
```

### 5.4 builder 模式处理多参数构造
当一个 struct 构造参数超过 ~4 个或有可选项，用 builder：
```rust
let req = HttpRequestBuilder::new("/api")
    .method("POST")
    .header("X-Trace", "abc")
    .timeout_ms(3000)
    .build()?;
```
社区常用 `derive_builder` / `bon` 自动生成。

---

## 六、API 设计

### 6.1 公开 API 尽量"接受最宽，返回最具体"
- 入参接受 `&str`、`&[T]`、`impl AsRef<Path>`、`impl Iterator` 等抽象。
- 返回值给最具体的类型（除非要隐藏实现细节，那时用 `impl Trait`）。

### 6.2 默认实现 `Debug`，需要时实现 `Clone` / `Default` / `PartialEq`
```rust
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Config { /* ... */ }
```
对外发布的类型，缺 `Debug` 会让用户调试体验非常差。

### 6.3 内部辅助函数加 `pub(crate)`，不要无脑 `pub`
Rust 的可见性是显式的，**默认私有**就是好默认。

### 6.4 文档注释要给例子
```rust
/// 计算两个向量的点积。
///
/// # Example
/// ```
/// let r = my_crate::dot(&[1.0, 2.0], &[3.0, 4.0]);
/// assert_eq!(r, 11.0);
/// ```
pub fn dot(a: &[f64], b: &[f64]) -> f64 { /* ... */ }
```
`cargo test` 会顺带执行文档里的代码，文档天然不过时。

---

## 七、并发与异步

### 7.1 单进程内共享状态：`Arc<Mutex<T>>` 是默认选择
- `Arc` 负责跨线程"共享所有权"（引用计数）。
- `Mutex` 负责跨线程"独占写入"。
- 高读低写场景换成 `RwLock`。
- 极简、性能要求高、状态小 → `AtomicXxx`。

### 7.2 async 选用 tokio
生态最大、文档最全、和绝大多数库（reqwest、axum、sqlx、tonic）原生兼容。除非你做嵌入式 / 极小二进制，否则别折腾选型。

### 7.3 async 关键纪律
- 不要在 async 函数里调用阻塞 IO（`std::fs`、`std::thread::sleep`），用 `tokio::fs`、`tokio::time::sleep`。
- CPU 密集任务用 `tokio::task::spawn_blocking` 扔到阻塞线程池。
- 跨 `.await` 持有 `MutexGuard` 是常见坑，改用 `tokio::sync::Mutex` 或缩小锁范围。

### 7.4 慎用 `unsafe`
- 99% 的应用代码完全不需要 `unsafe`。
- 需要时，把 `unsafe` 块缩到尽可能小，外面包装成 **安全 API**，并在注释里写明"为什么这里 unsafe 是合理的（safety invariant）"。

---

## 八、性能小贴士

- **永远先 `cargo build --release` 再说性能问题**，debug 模式慢 10~100 倍是正常的。
- 用 `cargo bench` + `criterion` 做基准测试，凭直觉优化是大忌。
- 大字符串拼接用 `String::with_capacity` 预分配，避免反复 realloc。
- 热点循环里避免不必要的 allocation（`format!`、`Vec::new`、`Box::new`）。
- 迭代器链通常和手写 for 同样快甚至更快，可读性优先。
- 真要榨性能：`cargo flamegraph`、`perf`、`samply` 上场。

---

## 九、测试

- 单测就放在被测模块同文件 `#[cfg(test)] mod tests { ... }`，能访问私有项。
- 集成测试放在 `tests/` 目录，模拟外部用户。
- 文档测试：`///` 注释里的 ```rust``` 代码块会被 `cargo test` 执行。
- 复杂测试用 `rstest`（参数化）、`proptest`（属性测试）、`insta`（快照测试）。
- 异步测试加 `#[tokio::test]`。

---

## 十、常用 crate 速查

| 场景 | 首选 crate |
|---|---|
| 序列化 / JSON | `serde` + `serde_json` |
| 命令行参数 | `clap`（derive 风格） |
| 日志 | `tracing` + `tracing-subscriber` |
| 错误（库） | `thiserror` |
| 错误（应用） | `anyhow` |
| 异步运行时 | `tokio` |
| HTTP 客户端 | `reqwest` |
| HTTP 服务端 | `axum` |
| 数据库 | `sqlx`（异步、编译期校验 SQL） |
| 时间 | `chrono` / `time` / `jiff` |
| UUID | `uuid` |
| 随机数 | `rand` |
| 正则 | `regex` |

---

## 十一、新人最容易栽跟头的 8 个点

1. **到处 `.clone()`**：先思考能否用引用。
2. **`String` vs `&str`**：参数能用 `&str` 就别用 `String`。
3. **借用检查器报错就 `unwrap`**：你绕过的是问题，不是错误。
4. **生命周期标注吓人**：90% 情况下不用手写，编译器要求时再加。
5. **滥用 `Box<dyn Trait>`**：能用泛型 `<T: Trait>` 就别用 trait 对象，性能差且失去单态化优势。
6. **trait object 上调用 `&self`，又想 `Send + Sync`**：会进入复杂 bound 地狱，先想想是不是该用枚举代替。
7. **async 中拿着锁过 `.await`**：编译器不会拦，但容易死锁，要么尽快释放，要么用异步锁。
8. **过度抽象**：Rust 很容易写出 4 层泛型 + 3 层 trait 的"完美设计"，但读起来痛苦。先写朴素版本，有重复再抽象。

---

> 把这些当作"出现偏离时要回到的基线"，不需要一次背完。
> 实际写两三个项目，你会自然回到这份清单上。
