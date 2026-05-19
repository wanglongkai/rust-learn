use std::env;
use std::time::{Duration, Instant};

fn main() {
    // println! / format! 是常用格式化宏。
    let name = "Rust";
    let message = format!("Hello, {name}!");
    println!("{message}");

    // dbg! 会打印表达式和结果，并返回所有权，适合临时调试。
    let value = dbg!(2 + 3);
    println!("value = {value}");

    // vec! 快速创建 Vec。
    let mut numbers = vec![1, 2, 3];
    numbers.push(4);
    println!("numbers = {:?}", numbers);

    // assert! / assert_eq! 常用于测试和运行期检查。
    assert!(numbers.contains(&4));
    assert_eq!(numbers.len(), 4);

    // Option 常用组合子：map、unwrap_or。
    let maybe_name = Some("Alice");
    let greeting = maybe_name.map(|n| format!("Hi, {n}")).unwrap_or(String::from("Hi"));
    println!("{greeting}");

    // Result 常用组合子：map_err、unwrap_or_else。
    let parsed = "42"
        .parse::<i32>()
        .map_err(|error| format!("解析失败: {error}"))
        .unwrap_or_else(|message| {
            println!("{message}");
            0
        });
    println!("parsed = {parsed}");

    // std::env 可读取命令行参数。
    let args: Vec<String> = env::args().collect();
    println!("命令行参数数量 = {}", args.len());

    // std::time 可测量耗时。
    let start = Instant::now();
    std::thread::sleep(Duration::from_millis(10));
    println!("耗时: {:?}", start.elapsed());
}
