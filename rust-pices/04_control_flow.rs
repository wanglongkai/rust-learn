fn main() {
    let number = 7;

    // if 是表达式，可以返回值，但所有分支类型必须一致。
    let size = if number < 5 { "small" } else { "large" };
    println!("{number} is {size}");

    // loop 创建无限循环，可用 break 返回值。
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 3 {
            break counter * 10;
        }
    };
    println!("loop 返回值 = {result}");

    // while 适合条件循环。
    let mut n = 3;
    while n > 0 {
        println!("while: {n}");
        n -= 1;
    }

    // for 常用于遍历集合或范围。
    let items = ["a", "b", "c"];
    for item in items {
        println!("for item = {item}");
    }

    // 1..=3 是闭区间，包含 3。
    for i in 1..=3 {
        println!("range i = {i}");
    }
}
