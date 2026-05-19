fn main() {
    let mut value = 10;

    // 不可变引用可以同时存在多个。
    let r1 = &value;
    let r2 = &value;
    println!("r1 = {r1}, r2 = {r2}");

    // 可变引用在同一时间只能有一个，并且不能和活跃不可变引用共存。
    let r3 = &mut value;
    *r3 += 5; // 使用 * 解引用，修改引用指向的值。
    println!("value = {value}");

    let sentence = String::from("hello rust world");
    let first = first_word(&sentence);
    println!("第一个单词 = {first}");

    // 字符串切片 &str 是对字符串一部分的引用，不拥有数据。
    let hello = &sentence[0..5];
    let rust = &sentence[6..10];
    println!("切片: {hello}, {rust}");

    // 数组切片也不拥有数据。
    let numbers = [10, 20, 30, 40, 50];
    let middle = &numbers[1..4];
    println!("数组切片 = {:?}", middle);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..index];
        }
    }

    s
}
