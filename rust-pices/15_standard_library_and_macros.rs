// ============================================================
// 第 15 课：标准库与宏 (std & macros)
// ------------------------------------------------------------
// 学习目标：
//   1. 掌握最常用的格式化宏：println! / format! / write! / eprintln! / dbg!。
//   2. 熟悉迭代器组合子：map / filter / collect / fold / zip / chain。
//   3. 熟悉 Option / Result 上的组合子。
//   4. 了解 std::env / std::time / std::fs 等常用模块。
// 前置知识：第 09 课集合迭代器、第 11 课 Result。
// 本课覆盖：宏 / 迭代器 / Option/Result 组合子 / env / time / fs。
// 运行方式：
//   rustc 15_standard_library_and_macros.rs && ./15_standard_library_and_macros
// ============================================================

use std::env;
use std::time::{Duration, Instant};

fn main() {
    // ============================================================
    // 一、格式化与日志宏
    // ============================================================

    // 1.1 println! / format!：标准输出 vs 返回 String
    let name = "Rust";
    let s = format!("Hello, {name}!");
    println!("{s}");

    // 1.2 各种格式化写法
    let n = 42;
    println!(
        "十进制 {}, 十六进制 {:x}, 二进制 {:b}, 八进制 {:o}",
        n, n, n, n
    );
    println!("宽度填充 |{:>6}|{:<6}|{:^6}|", 7, 7, 7); // 右/左/居中
    println!("浮点保留小数 {:.3}", std::f64::consts::PI);
    println!(
        "具名参数 {who} 在 {place}",
        who = "Alice",
        place = "Shanghai"
    );
    println!("Debug 打印 {:?}", vec![1, 2, 3]);
    println!("漂亮 Debug:\n{:#?}", vec![1, 2, 3]);

    // 1.3 eprintln! 输出到 stderr（错误日志和正常日志要分流）
    eprintln!("[err-stream] 这条会出现在 stderr");

    // 1.4 dbg! 调试神器：打印表达式 + 位置 + 值，并返回所有权
    let value = dbg!(2 + 3);
    println!("value = {value}");

    // ============================================================
    // 二、迭代器组合子（"零成本抽象"的最佳代言）
    // ============================================================

    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // 2.1 map / filter / collect：经典流水线
    let evens_squared: Vec<i32> = v.iter().filter(|&&x| x % 2 == 0).map(|x| x * x).collect();
    println!("偶数的平方 = {:?}", evens_squared);

    // 2.2 sum / count / max / min
    let sum: i32 = v.iter().sum();
    let max = v.iter().max().copied();
    println!("sum = {sum}, max = {:?}", max);

    // 2.3 fold：通用累积
    let product: i32 = v.iter().fold(1, |acc, &x| acc * x);
    println!("product = {product}");

    // 2.4 zip / enumerate / chain
    let names = ["Alice", "Bob", "Carol"];
    let ages = [30, 25, 40];
    let pairs: Vec<(&&str, &i32)> = names.iter().zip(ages.iter()).collect();
    println!("pairs = {:?}", pairs);

    for (i, n) in v.iter().enumerate().take(3) {
        println!("idx={i} val={n}");
    }

    let combined: Vec<i32> = [1, 2].iter().chain([3, 4].iter()).copied().collect();
    println!("combined = {:?}", combined);

    // 2.5 find / any / all：搜索/断言
    let first_big = v.iter().find(|&&x| x > 5);
    let has_zero = v.iter().any(|&x| x == 0);
    let all_pos = v.iter().all(|&x| x > 0);
    println!("first_big = {first_big:?}, has_zero = {has_zero}, all_pos = {all_pos}");

    // 2.6 collect 到不同集合：HashMap / String
    use std::collections::HashMap;
    let map: HashMap<&&str, &i32> = names.iter().zip(ages.iter()).collect();
    println!("map = {:?}", map);

    let joined: String = ["rust", "is", "fast"]
        .iter()
        .copied()
        .collect::<Vec<_>>()
        .join(" ");
    println!("joined = {joined}");

    // ============================================================
    // 三、Option / Result 上的组合子
    // ============================================================

    let maybe_name: Option<&str> = Some("Alice");
    let greeting = maybe_name
        .map(|n| format!("Hi, {n}"))
        .unwrap_or_else(|| String::from("Hi, stranger"));
    println!("{greeting}");

    let parsed: i32 = "42"
        .parse::<i32>()
        .map_err(|e| format!("解析失败: {e}"))
        .unwrap_or_else(|msg| {
            eprintln!("{msg}");
            0
        });
    println!("parsed = {parsed}");

    // and_then：链式调用，前一步失败/None 就短路
    let chained: Option<i32> = Some("10")
        .and_then(|s| s.parse::<i32>().ok())
        .map(|n| n * 2);
    println!("chained = {:?}", chained);

    // ok_or：Option -> Result
    let r: Result<&str, &str> = Some("abc").ok_or("没值");
    println!("r = {:?}", r);

    // ============================================================
    // 四、常用 std 模块速览
    // ============================================================

    // 4.1 std::env：命令行参数 / 环境变量
    let args: Vec<String> = env::args().collect();
    println!("命令行参数共 {} 个: {:?}", args.len(), args);
    if let Ok(home) = env::var("HOME").or_else(|_| env::var("USERPROFILE")) {
        println!("HOME / USERPROFILE = {home}");
    }

    // 4.2 std::time：测量耗时
    let start = Instant::now();
    std::thread::sleep(Duration::from_millis(10));
    println!("耗时: {:?}", start.elapsed());

    // 4.3 std::fs（这里只演示读，写在第 11 课已经见过）
    if let Ok(text) = std::fs::read_to_string("Cargo.toml") {
        println!(
            "Cargo.toml 前 60 字符: {}",
            text.chars().take(60).collect::<String>()
        );
    } else {
        println!("(运行目录下没有 Cargo.toml，跳过)");
    }

    // ============================================================
    // 五、断言宏：测试与运行期检查
    // ============================================================

    let mut numbers = vec![1, 2, 3];
    numbers.push(4);
    assert!(numbers.contains(&4));
    assert_eq!(numbers.len(), 4);
    assert_ne!(numbers.first(), numbers.last());

    // debug_assert! 系列：仅在 debug 编译下检查，release 下消失（零开销）
    debug_assert!(!numbers.is_empty());

    // unreachable! / todo! / unimplemented!：宣告"这里不该到"
    // let _: i32 = match 1 { 1 => 1, _ => unreachable!() };
}

// ============================================================
// 本课小结
//   - 格式化宏：println! / eprintln! / format! / dbg! —— 调试 + 输出全靠它们。
//   - 迭代器是 Rust 写代码的"主旋律"：先 .iter()，再链式组合，最后 collect 或聚合。
//   - Option / Result 大量用组合子（map / and_then / unwrap_or_else），避免到处 match。
//   - 标准库非常丰富：env / time / fs / process / thread / sync / collections 都值得翻一遍 docs.rs。
//
// 常见陷阱
//   - {} 找不到 Display 实现 → 用 {:?} (Debug) 调试，或给类型实现 Display。
//   - .unwrap() 用在 demo 没事，写到生产 → panic 等着你。
//   - collect 不写类型 → Rust 不知道收成什么集合；写成 collect::<Vec<_>>()。
//   - 把 &str 当 String 累加：每次 + 都 allocate；用 String::with_capacity + push_str。
//
// 最佳实践
//   - 调试用 dbg!()；正式日志用 tracing crate（生产首选）。
//   - 数据流式处理优先迭代器链，不要先 collect 成 Vec 再处理（浪费内存）。
//   - Option/Result 处理优先组合子，match 只在"必须区分多个分支"时再用。
//   - 不熟悉的方法立刻打开 docs.rs/std，标准库文档极其详尽。
//
// 思考题
//   1. iter() / iter_mut() / into_iter() 三者产出的元素类型分别是？
//   2. collect 是怎么知道要收集成 Vec<T> 还是 HashMap<K, V> 的？
//   3. fold(init, f) 和 reduce(f) 有什么区别？
// ============================================================
