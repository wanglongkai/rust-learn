// ============================================================
// 第 08 课：枚举与模式匹配 (Enums & Pattern Matching)
// ------------------------------------------------------------
// 学习目标：
//   1. 掌握 enum 的多种变体（无数据 / 元组 / 具名字段）。
//   2. 理解 Option<T> 为什么是 Rust 没有 null 的关键。
//   3. 熟练使用 match：穷尽性 + 模式绑定 + 守卫。
//   4. 学会用 if let / while let / let else 写紧凑代码。
// 前置知识：第 07 课 struct。
// 本课覆盖：enum / Option / match / if let / while let / let else。
// 运行方式：
//   rustc 08_enums_and_pattern_matching.rs && ./08_enums_and_pattern_matching
// ============================================================

// ------------------------------------------------------------
// 1. enum 的三种变体写法
//    枚举把"互斥的多种可能"编码进类型系统——比字符串/状态码安全得多。
// ------------------------------------------------------------
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8), // 元组式变体
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,                    // 无数据
    Move { x: i32, y: i32 }, // 具名字段变体
    Write(String),           // 单值元组变体
    ChangeColor(i32, i32, i32),
}

// 枚举也能有方法
impl Message {
    fn describe(&self) -> String {
        match self {
            Message::Quit => String::from("退出"),
            Message::Move { x, y } => format!("移动到 ({x}, {y})"),
            Message::Write(text) => format!("写入 '{text}'"),
            Message::ChangeColor(r, g, b) => format!("颜色 RGB({r}, {g}, {b})"),
        }
    }
}

fn main() {
    // ------------------------------------------------------------
    // 2. 创建并匹配
    // ------------------------------------------------------------
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    describe_ip(&home);
    describe_ip(&loopback);

    let messages = [
        Message::Quit,
        Message::Move { x: 3, y: 4 },
        Message::Write(String::from("hello")),
        Message::ChangeColor(255, 0, 0),
    ];
    for m in &messages {
        println!("{}", m.describe());
    }

    // ------------------------------------------------------------
    // 3. Option<T>：Rust 没有 null，用 Option 表达"也许有值"
    //    enum Option<T> { Some(T), None }
    //    强迫调用方在用值前先处理 None，把空指针错误消灭在编译期。
    // ------------------------------------------------------------
    let some_n: Option<i32> = Some(5);
    let no_n: Option<i32> = None;
    println!("plus_one(Some(5)) = {:?}", plus_one(some_n));
    println!("plus_one(None)    = {:?}", plus_one(no_n));

    // 常用组合子，避免每次都写 match：
    let doubled = some_n.map(|v| v * 2).unwrap_or(0);
    let fallback = no_n.unwrap_or(-1);
    println!("doubled = {doubled}, fallback = {fallback}");

    // ------------------------------------------------------------
    // 4. match：穷尽性 + 模式绑定 + 守卫 (guard)
    //    - 必须覆盖所有可能（编译期检查），改 enum 时编译器会指出所有遗漏的地方。
    //    - 模式可以解构、绑定变量、加条件。
    // ------------------------------------------------------------
    let pair = (1, -1);
    match pair {
        (0, 0) => println!("原点"),
        (x, 0) => println!("X 轴上的点 x={x}"),
        (0, y) => println!("Y 轴上的点 y={y}"),
        (x, y) if x == y => println!("对角线上的点 ({x},{y})"),
        (x, y) if x > 0 && y > 0 => println!("第一象限 ({x},{y})"),
        _ => println!("其它"), // _ 通配符：兜底，必须放最后
    }

    // 范围匹配 + 字面量
    let n = 5;
    let label = match n {
        0 => "zero",
        1..=4 => "small",
        5 | 6 | 7 => "lucky",
        _ => "big",
    };
    println!("{n} -> {label}");

    // ------------------------------------------------------------
    // 5. if let：只关心一种情况的简写
    //    适合"匹配 Some 时做点事，不关心 None"这种场景。
    // ------------------------------------------------------------
    if let Some(v) = some_n {
        println!("if let 拿到: {v}");
    }

    // 配合 else：
    let maybe = Some(10);
    if let Some(v) = maybe {
        println!("拿到 {v}");
    } else {
        println!("没值");
    }

    // ------------------------------------------------------------
    // 6. while let：循环里不断尝试匹配
    //    经典：弹栈直到空。
    // ------------------------------------------------------------
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("pop -> {top}");
    }

    // ------------------------------------------------------------
    // 7. let else：匹配失败就跳走（Rust 1.65+）
    //    特别适合"前置校验：拿不到值就 return / continue"。
    // ------------------------------------------------------------
    fn parse_id(text: &str) -> Option<u32> {
        let Ok(n) = text.parse::<u32>() else {
            return None;
        };
        Some(n * 100)
    }
    println!("parse_id('42') = {:?}", parse_id("42"));
    println!("parse_id('xx') = {:?}", parse_id("xx"));
}

fn plus_one(value: Option<i32>) -> Option<i32> {
    match value {
        Some(x) => Some(x + 1),
        None => None,
    }
}

fn describe_ip(ip: &IpAddr) {
    match ip {
        IpAddr::V4(a, b, c, d) => println!("IPv4 地址: {a}.{b}.{c}.{d}"),
        IpAddr::V6(addr) => println!("IPv6 地址: {addr}"),
    }
}

// ============================================================
// 本课小结
//   - enum 能编码"多种互斥可能"，每个变体可以携带不同形状的数据。
//   - Option<T> 替代了其它语言里的 null，用类型系统消灭空指针。
//   - match 是 Rust 的核心控制流：穷尽 + 强大模式 + 守卫。
//   - if let / while let / let else 是 match 的简化形式。
//
// 常见陷阱
//   - 给 Option 取值时直接 .unwrap()，运行时 panic。
//   - match 漏处理新增的 enum 变体（用 _ 兜底会失去编译器的提醒，慎用）。
//   - 对包含 String / Vec 的 enum 做 match 时，需要用 ref 或 & 引用避免 move。
//
// 最佳实践
//   - 用 enum + match 替代字符串状态（"pending"、"done" 这种最容易拼错）。
//   - 让函数返回 Option / Result 表达"可能没有 / 可能失败"。
//   - Option 优先用组合子（map / and_then / unwrap_or / ok_or）替代 match。
//   - 业务里的非法状态用 enum 排除，比如 enum User { Guest, LoggedIn { name: String } }。
//
// 思考题
//   1. enum 的内存布局是怎样的？为什么 Option<&T> 和 &T 占同样大小？(提示：niche)
//   2. 下面两种写法哪个更地道？
//        match opt { Some(v) => v + 1, None => 0 }
//        opt.map(|v| v+1).unwrap_or(0)
//   3. let else 和 if let ... else { ... } 有什么本质区别？
// ============================================================
