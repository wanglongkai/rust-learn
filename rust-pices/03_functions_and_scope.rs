fn main() {
    greet("Rust");

    let sum = add(3, 4);
    println!("3 + 4 = {sum}");

    // 代码块也是表达式，最后一个无分号的表达式会成为返回值。
    let doubled = {
        let value = 10;
        value * 2
    };
    println!("代码块表达式结果 = {doubled}");

    // 作用域控制变量生命周期。inner 只在大括号内部有效。
    {
        let inner = "只在这个作用域中存在";
        println!("{inner}");
    }
}

// 函数参数必须标注类型。
fn greet(name: &str) {
    println!("你好，{name}!");
}

// 使用 -> 标注返回类型。
// 函数体最后一个表达式不写分号，表示返回该值。
fn add(left: i32, right: i32) -> i32 {
    left + right
}
