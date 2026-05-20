// ============================================================
// 第 11 课：错误处理 (Error Handling)
// ------------------------------------------------------------
// 学习目标：
//   1. 区分"可恢复错误 (Result)"与"不可恢复错误 (panic!)"。
//   2. 熟练使用 ? 运算符传播错误。
//   3. 学会用 From + 自定义错误类型让错误自动转换。
//   4. 了解库代码 (thiserror) 和应用代码 (anyhow) 的不同最佳实践。
// 前置知识：第 08 课 enum/Option/match。
// 本课覆盖：panic! / Result<T, E> / ? / From / 错误转换。
// 运行方式：
//   rustc 11_error_handling.rs && ./11_error_handling
// ============================================================

use std::fs::File;
use std::io::{self, ErrorKind, Read};
use std::num::ParseIntError;

fn main() {
    // ------------------------------------------------------------
    // 1. panic!：不可恢复错误，会终止当前线程
    //    适合"代码 bug、不变量被破坏"，不要用来处理业务上的"用户输入错"。
    //    用 RUST_BACKTRACE=1 cargo run 可看完整调用栈。
    // ------------------------------------------------------------
    // panic!("不可恢复错误"); // 别真的取消注释 :)

    // .unwrap() / .expect() 本质就是"失败就 panic"
    // 写产品代码尽量避免，写 demo / 测试可以放心用。
    let n: i32 = "42".parse().unwrap(); // 成功
    let n2: i32 = "42".parse().expect("应是数字"); // 失败时打印自定义信息
    println!("n = {n}, n2 = {n2}");

    // ------------------------------------------------------------
    // 2. Result<T, E>：可恢复错误
    //    enum Result<T, E> { Ok(T), Err(E) }
    //    所有"可能失败"的函数都应该返回 Result，让调用方决定怎么处理。
    // ------------------------------------------------------------
    let greeting = File::open("hello.txt");
    let _file = match greeting {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // 文件不存在 → 我们自己创建
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(created) => created,
                Err(create_error) => panic!("创建文件也失败了: {create_error:?}"),
            },
            // 其它错误（权限不足等）→ 没法恢复，直接 panic
            other => panic!("打开文件失败: {other:?}"),
        },
    };

    // ------------------------------------------------------------
    // 3. ? 运算符：错误传播的语法糖
    //    `x?` 展开后近似为：
    //      match x {
    //          Ok(v) => v,
    //          Err(e) => return Err(e.into()),
    //      }
    //    要求：当前函数返回类型也是 Result（或 Option / ControlFlow 等实现了 Try 的）。
    // ------------------------------------------------------------
    match read_username_from_file("hello.txt") {
        Ok(name) if !name.trim().is_empty() => println!("用户名: {name}"),
        Ok(_) => println!("文件存在但内容为空"),
        Err(error) => println!("读取失败: {error}"),
    }

    // ------------------------------------------------------------
    // 4. ? 在 Option 里也能用：Some 继续走，None 直接 return None
    // ------------------------------------------------------------
    println!("first_digit('abc12') = {:?}", first_digit("abc12"));
    println!("first_digit('abc')   = {:?}", first_digit("abc"));

    // ------------------------------------------------------------
    // 5. 组合子：避免到处写 match
    // ------------------------------------------------------------
    let parsed = "42"
        .parse::<i32>()
        .map_err(|e| format!("解析失败: {e}")) // 转换错误类型
        .unwrap_or_else(|msg| {
            // 失败时给个默认值
            println!("{msg}");
            0
        });
    println!("parsed = {parsed}");

    let chained: Result<i32, ParseIntError> = "10"
        .parse::<i32>()
        .and_then(|n| "20".parse::<i32>().map(|m| n + m));
    println!("chained = {:?}", chained);

    // ------------------------------------------------------------
    // 6. 自定义错误 + From：让 ? 自动转换异质错误
    //    这是"库代码"里最常见的做法，免去手写 .map_err(...)。
    // ------------------------------------------------------------
    match parse_and_double("12") {
        Ok(v) => println!("parse_and_double('12') = {v}"),
        Err(e) => println!("err = {e}"),
    }
    match parse_and_double("oops") {
        Ok(v) => println!("ok = {v}"),
        Err(e) => println!("err = {e}"), // ParseIntError 被 From 转成 MyError::Parse
    }
}

// ------------------------------------------------------------
// 函数返回 Result + ?，让错误"顺着调用栈"自动向上传播
// ------------------------------------------------------------
fn read_username_from_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?; // ? 失败立即返回
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}

// 在 Option 上的 ?
fn first_digit(s: &str) -> Option<u32> {
    let c = s.chars().find(|c| c.is_ascii_digit())?; // 没找到就直接返回 None
    c.to_digit(10)
}

// ------------------------------------------------------------
// 自定义错误：标准做法
// ------------------------------------------------------------
#[derive(Debug)]
enum MyError {
    Parse(ParseIntError),
    OutOfRange,
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::Parse(e) => write!(f, "解析错误: {e}"),
            MyError::OutOfRange => write!(f, "数字超出业务允许范围"),
        }
    }
}

impl std::error::Error for MyError {} // 标准 Error trait（生态期望）

// 关键：实现 From<ParseIntError> → 让 `?` 能自动把 ParseIntError 转成 MyError
impl From<ParseIntError> for MyError {
    fn from(e: ParseIntError) -> Self {
        MyError::Parse(e)
    }
}

fn parse_and_double(s: &str) -> Result<i32, MyError> {
    let n: i32 = s.parse()?; // ParseIntError 自动转为 MyError::Parse
    if !(0..=1000).contains(&n) {
        return Err(MyError::OutOfRange);
    }
    Ok(n * 2)
}

// ============================================================
// 本课小结
//   - 可恢复错误 → Result<T, E>；不可恢复 (代码 bug) → panic!。
//   - `?` 是错误传播的核心语法糖，依赖 From trait 自动转换。
//   - 自定义错误三件套：enum + Display + Error trait（+ From 实现）。
//
// 常见陷阱
//   - 业务代码到处 .unwrap()：上线就崩。
//   - 把所有错误一锅 panic：失去恢复机会。
//   - 自定义错误不实现 Display / Error：和生态库（anyhow、thiserror、tracing）不兼容。
//   - 在 main 里写一堆 match：试试把 main 返回 Result<(), E>，直接用 `?`。
//
// 最佳实践
//   - 库代码：用 `thiserror` 派生 enum 错误，错误信息精确、稳定。
//   - 应用代码：用 `anyhow::Result<T>` 统一返回 + `.context("说明")` 附加上下文。
//   - panic! 留给"理论上不可能"的分支，并配合 unreachable!() / debug_assert!()。
//   - 想看堆栈：环境变量 RUST_BACKTRACE=1。
//
// 思考题
//   1. ? 和 try!() 宏（已废弃）有什么联系？
//   2. fn main() -> Result<(), Box<dyn std::error::Error>> 的意义是什么？
//   3. 为什么"库用 thiserror、应用用 anyhow"是社区共识？
// ============================================================
