fn main() {
    // Rust 变量默认不可变，这能帮助你减少意外修改。
    let x = 5;
    println!("不可变变量 x = {x}");

    // 如果需要修改变量，必须显式使用 mut。
    let mut count = 0;
    count += 1;
    println!("可变变量 count = {count}");

    // 常量使用 const，必须标注类型，且只能绑定到编译期可确定的值。
    const MAX_POINTS: u32 = 100_000;
    println!("常量 MAX_POINTS = {MAX_POINTS}");

    // shadowing：可以用 let 重新声明同名变量。
    // 它不是修改原变量，而是创建一个新的绑定。
    let spaces = "   ";
    let spaces = spaces.len();
    println!("空格数量 = {spaces}");

    // shadowing 可以改变类型；mut 变量不能改变类型。
    let score = 10;
    let score = score + 5;
    let score = format!("{score} 分");
    println!("shadowing 后的 score = {score}");
}
