fn main() {
    // String 在堆上分配，赋值给另一个变量时会发生 move。
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{s1}"); // 编译错误：s1 的所有权已经移动给 s2。
    println!("s2 = {s2}");

    // clone 会复制堆数据，两个变量都可继续使用。
    let a = String::from("rust");
    let b = a.clone();
    println!("a = {a}, b = {b}");

    // 实现 Copy 的类型（如整数）赋值时会按位复制，不会 move。
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");

    let text = String::from("ownership");
    let length = calculate_length(&text); // 不转移所有权，只借用。
    println!("'{text}' 的长度是 {length}");

    let mut message = String::from("hello");
    change(&mut message); // 可变借用允许被借用函数修改值。
    println!("修改后 message = {message}");
}

fn calculate_length(s: &String) -> usize {
    // s 是不可变引用，不能修改它指向的数据。
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", Rust");
}
