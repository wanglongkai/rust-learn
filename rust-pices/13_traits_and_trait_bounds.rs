// ============================================================
// 第 13 课：Trait 与 Trait Bound (接口抽象)
// ------------------------------------------------------------
// 学习目标：
//   1. 理解 trait 是 Rust 的"接口"——定义一组行为契约。
//   2. 学会用 impl Trait for Type 实现 trait，提供默认方法。
//   3. 掌握 trait bound 的多种写法，理解 impl Trait / dyn Trait 的差异。
//   4. 了解几个最常用的"派生 trait"：Debug / Clone / PartialEq / Default。
// 前置知识：第 12 课 泛型。
// 本课覆盖：trait / impl / 默认方法 / impl Trait / dyn Trait / where。
// 运行方式：
//   rustc 13_traits_and_trait_bounds.rs && ./13_traits_and_trait_bounds
// ============================================================

// ------------------------------------------------------------
// 1. 定义 trait：一组方法签名（可附默认实现）
// ------------------------------------------------------------
trait Summary {
    fn summarize(&self) -> String;

    // 默认实现：实现者可以选择重写，不重写就用这个
    fn author(&self) -> String {
        String::from("未知作者")
    }

    // 默认实现也可以调用 trait 里的其它方法
    fn announcement(&self) -> String {
        format!("(来自 {}): {}", self.author(), self.summarize())
    }
}

// ------------------------------------------------------------
// 2. 为具体类型实现 trait
// ------------------------------------------------------------
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} (来自 {})", self.headline, self.location)
    }
    fn author(&self) -> String {
        self.author.clone()
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
    // 不重写 author → 用默认实现 "未知作者"
}

// ------------------------------------------------------------
// 3. 用 trait 做参数：三种等价写法，逐步演进
// ------------------------------------------------------------

// (a) impl Trait 语法：最简洁，单约束首选
fn notify_a(item: &impl Summary) {
    println!("[a] {}", item.summarize());
}

// (b) trait bound 语法：等价于 (a)，多个参数共享同一个泛型类型时必须用这种
fn notify_b<T: Summary>(item: &T) {
    println!("[b] {}", item.summarize());
}

fn notify_pair<T: Summary>(a: &T, b: &T) {
    // a 和 b 必须是"同一种类型"
    println!("[pair] {} / {}", a.summarize(), b.summarize());
}

// (c) where 子句：多约束时可读性最好
fn notify_c<T>(item: &T)
where
    T: Summary + std::fmt::Debug,
{
    println!("[c] {}", item.summarize());
    // 此时既能用 Summary 的方法，也能用 Debug
}

// 多个 trait bound：+ 连接
#[allow(dead_code)]
fn notify_multi<T: Summary + Clone>(item: &T) -> T {
    println!("[multi] {}", item.summarize());
    item.clone()
}

// ------------------------------------------------------------
// 4. 返回值用 impl Trait：函数返回"实现了某 trait 的某个类型"，
//    但不暴露具体类型。常用于返回闭包、迭代器、Future。
//    限制：函数所有返回路径必须是同一具体类型。
// ------------------------------------------------------------
fn make_summary() -> impl Summary {
    Tweet {
        username: String::from("rustacean"),
        content: String::from("Rust 真好用"),
    }
}

// ------------------------------------------------------------
// 5. dyn Trait（trait object）：运行时多态
//    - impl Trait / <T: Trait>：编译期单态化，零开销，但函数只能"一种 T"。
//    - dyn Trait：通过 vtable 在运行时分发，能在容器里混着存不同具体类型。
//    - 使用代价：一次虚函数调用 + 一次指针解引用，几乎可忽略，但失去了内联机会。
// ------------------------------------------------------------
fn notify_dyn(item: &dyn Summary) {
    // dyn 时 item 是"指向某种实现 Summary 的类型的指针"
    println!("[dyn] {}", item.summarize());
}

fn make_box() -> Box<dyn Summary> {
    // 不同分支返回不同具体类型，必须用 Box<dyn Trait>，不能用 impl Trait
    if rand_bool() {
        Box::new(Tweet {
            username: String::from("a"),
            content: String::from("hi"),
        })
    } else {
        Box::new(NewsArticle {
            headline: String::from("h"),
            location: String::from("l"),
            author: String::from("x"),
        })
    }
}

// 简易"随机"——只用标准库
fn rand_bool() -> bool {
    use std::time::SystemTime;
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_nanos() % 2 == 0)
        .unwrap_or(true)
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Rust 学习路线发布"),
        location: String::from("Shanghai"),
        author: String::from("Alice"),
    };
    let tweet = Tweet {
        username: String::from("rustacean"),
        content: String::from("Ownership is powerful!"),
    };

    notify_a(&article);
    notify_b(&tweet);
    notify_c(&NewsArticleDbg {
        headline: String::from("debug 版"),
        author: String::from("Bob"),
    });
    notify_pair(
        &article,
        &NewsArticle {
            headline: String::from("第二条"),
            location: String::from("Beijing"),
            author: String::from("Carol"),
        },
    );

    println!("默认 announcement: {}", tweet.announcement());

    let s = make_summary();
    println!("make_summary -> {}", s.summarize());

    // dyn Trait 的容器：把不同具体类型混着存
    let items: Vec<Box<dyn Summary>> = vec![
        Box::new(Tweet {
            username: "x".into(),
            content: "1".into(),
        }),
        Box::new(NewsArticle {
            headline: "h".into(),
            location: "l".into(),
            author: "y".into(),
        }),
    ];
    for it in &items {
        notify_dyn(it.as_ref());
    }
    let _b = make_box();
}

// 为了演示 Debug + Summary
#[derive(Debug)]
struct NewsArticleDbg {
    headline: String,
    author: String,
}
impl Summary for NewsArticleDbg {
    fn summarize(&self) -> String {
        self.headline.clone()
    }
    fn author(&self) -> String {
        self.author.clone()
    }
}

// ============================================================
// 本课小结
//   - trait 是 Rust 的接口契约，可以提供默认实现。
//   - 静态分发：impl Trait / <T: Trait> → 单态化、零成本、不能混存。
//   - 动态分发：dyn Trait → vtable、有微小开销、可以在 Vec 里混存。
//   - 多约束 + 多参数 → 用 where 子句保持可读性。
//
// 常见陷阱
//   - 把 fn f(x: impl Trait, y: impl Trait) 当作 "x 和 y 必须同一类型"——错，
//     impl Trait 每个位置都是独立的泛型参数。要约束相同类型必须显式 <T: Trait>。
//   - dyn Trait 上调用 generic 方法 (fn foo<U>(&self)) → 编译报错"不是 object-safe"。
//   - 用 dyn Trait 时忘了 Box / &，trait 不是 Sized 不能直接当值。
//
// 最佳实践
//   - 公共行为提炼成 trait，便于将来扩展和测试 (mock)。
//   - 公共类型尽量 derive(Debug, Clone, Default, PartialEq, Eq)；
//     需要排序加 Ord/PartialOrd；需要哈希加 Hash。
//   - 默认用静态分发（性能 + 内联）；只有"容器中混合多种实现"或"插件化扩展"时才用 dyn。
//   - 把短小高频的方法放进 trait，"组合优于继承"。
//
// 思考题
//   1. 为什么 Vec<impl Display> 不合法？而 Vec<Box<dyn Display>> 合法？
//   2. dyn Trait 中的 "object safety" 是什么？哪些 trait 不能做对象？
//   3. fn returns_closure() -> impl Fn(i32) -> i32 { |x| x + 1 } 可行吗？
//      改成在 if 两个分支返回不同闭包还行吗？
// ============================================================
