use std::collections::HashMap;

fn main() {
    // Vec<T>：可增长数组，适合保存同类型多个值。
    let mut numbers = Vec::new();
    numbers.push(10);
    numbers.push(20);
    numbers.push(30);
    println!("numbers = {:?}", numbers);

    // get 返回 Option，避免索引越界导致 panic。
    match numbers.get(1) {
        Some(value) => println!("第二个元素 = {value}"),
        None => println!("没有第二个元素"),
    }

    for value in &mut numbers {
        *value += 1;
    }
    println!("修改后的 numbers = {:?}", numbers);

    // String 是 UTF-8 编码的可增长字符串。
    let mut text = String::from("hello");
    text.push(' ');
    text.push_str("Rust");
    println!("text = {text}");

    // 推荐按 chars 或 bytes 遍历字符串，而不是直接按索引访问。
    for ch in text.chars() {
        print!("{ch} ");
    }
    println!();

    // HashMap<K, V>：键值对集合。
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // entry 可在键不存在时插入默认值。
    scores.entry(String::from("Blue")).or_insert(20);
    scores.entry(String::from("Red")).or_insert(30);

    for (team, score) in &scores {
        println!("{team}: {score}");
    }
}
