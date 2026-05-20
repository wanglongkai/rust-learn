// ============================================================
// 第 09 课：常用集合 (Vec / String / HashMap)
// ------------------------------------------------------------
// 学习目标：
//   1. 掌握 Vec<T>：可增长数组，最常用的集合。
//   2. 掌握 String：UTF-8 可增长字符串，区别于 &str。
//   3. 掌握 HashMap<K, V>：键值对集合。
//   4. 学会迭代器的常用模式：iter / iter_mut / into_iter。
// 前置知识：第 05~08 课。
// 本课覆盖：Vec / String / HashMap / iter 系列。
// 运行方式：
//   rustc 09_common_collections.rs && ./09_common_collections
// ============================================================

use std::collections::HashMap;

fn main() {
    // ============================================================
    // 一、Vec<T>：可增长数组，几乎所有"列表"场景都用它
    // ============================================================

    // ---- 1.1 创建 ----
    let mut nums: Vec<i32> = Vec::new();
    nums.push(10);
    nums.push(20);
    nums.push(30);
    println!("nums = {:?}", nums);

    // vec! 宏更常用
    let v = vec![1, 2, 3, 4, 5];
    println!("v = {:?}", v);

    // 已知大小时预分配，避免反复扩容
    let mut big = Vec::with_capacity(1000);
    big.push(1);
    println!("big.len()={}, big.capacity()={}", big.len(), big.capacity());

    // ---- 1.2 访问元素 ----
    // 两种方式：[] 越界 panic，.get() 返回 Option<&T> 更安全
    let first = &v[0]; // panic if v is empty
    let maybe = v.get(100); // Option，安全
    println!("first = {first}, maybe = {:?}", maybe);

    // ---- 1.3 迭代：三种"风味" ----
    let v = vec![1, 2, 3];

    // a) iter()：&T，只读
    for x in v.iter() {
        print!("{x} ");
    }
    println!("  (iter，原 Vec 仍可用)");

    // b) iter_mut()：&mut T，可修改
    let mut v_mut = vec![1, 2, 3];
    for x in v_mut.iter_mut() {
        *x *= 10;
    }
    println!("v_mut = {:?}", v_mut);

    // c) into_iter()：T，move 出元素（消耗 Vec）
    let v2 = vec![String::from("a"), String::from("b")];
    for s in v2.into_iter() {
        println!("拿到 owned String: {s}");
    }
    // println!("{:?}", v2); // 错：v2 已被 move

    // ---- 1.4 迭代器组合子（极常用，先有印象，第 15 课更深入）----
    let v = vec![1, 2, 3, 4, 5];
    let doubled_evens: Vec<i32> = v.iter().filter(|&&x| x % 2 == 0).map(|x| x * 2).collect();
    println!("偶数的两倍 = {:?}", doubled_evens);

    let sum: i32 = v.iter().sum();
    let max = v.iter().max();
    println!("sum = {sum}, max = {:?}", max);

    // ============================================================
    // 二、String：UTF-8 可增长字符串
    // ============================================================
    //   - String：拥有所有权的堆字符串。
    //   - &str  ：字符串切片 / 字符串字面量，不拥有数据。
    //   选择口诀："要存就用 String，参数就用 &str"。
    // ============================================================

    let mut greeting = String::from("hello");
    greeting.push(' ');
    greeting.push_str("Rust");
    let combined = format!("{greeting}, 你好");
    println!("greeting = {greeting}, combined = {combined}");

    // UTF-8 注意事项：不能用整数索引！
    // let c = greeting[0]; // 编译错误
    //
    // 想取"第一个字符"，应该按 chars() 遍历：
    if let Some(first_char) = combined.chars().next() {
        println!("第一个字符 = {first_char}");
    }
    println!("字符数 = {}", combined.chars().count());
    println!("字节数 = {}", combined.len()); // 注意：len 是字节数，不是字符数！

    // 安全切片：必须落在 UTF-8 字符边界，否则 panic
    let s = String::from("hello rust");
    println!("切片: '{}'", &s[..5]);

    // ============================================================
    // 三、HashMap<K, V>：键值对集合
    // ============================================================

    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 读：get 返回 Option<&V>
    if let Some(v) = scores.get("Blue") {
        println!("Blue = {v}");
    }

    // entry API：键不存在则插入，已存在则不动。极地道。
    scores.entry(String::from("Blue")).or_insert(99); // 已存在 → 仍是 10
    scores.entry(String::from("Red")).or_insert(30); // 不存在 → 插入 30

    // 计数模式：把某个 key 的 value 不断 +1
    let text = "the quick brown fox jumps over the lazy dog the fox";
    let mut count: HashMap<&str, i32> = HashMap::new();
    for word in text.split_whitespace() {
        *count.entry(word).or_insert(0) += 1;
    }
    println!("词频 = {:?}", count);

    // 遍历
    for (team, score) in &scores {
        println!("{team}: {score}");
    }

    // ---- HashMap 与所有权 ----
    // 插入 String 这种非 Copy 类型时，键值会被 move 进 HashMap。
    // 插入 i32、&str 这种 Copy / 引用类型，则不会 move。
}

// ============================================================
// 本课小结
//   - Vec<T>：可增长数组，掌握 .iter() / .iter_mut() / .into_iter() 三种迭代。
//   - String/&str：String 拥有，&str 借用；存数据用 String，参数用 &str。
//   - HashMap<K,V>：用 .entry().or_insert() 写出干净的"有就更新，没有就插入"。
//   - 集合迭代器组合子（map/filter/sum/collect/...）是 Rust 的灵魂之一。
//
// 常见陷阱
//   - 用 Vec 索引访问可能 panic，循环遍历更安全或用 .get()。
//   - String 不能下标取字符；用 .chars()、.bytes() 或安全切片。
//   - 把 String 当 key 插 HashMap，之后还想用原 String → 已 move。
//   - HashMap 遍历顺序是随机的，不能依赖（要稳定顺序用 BTreeMap）。
//
// 最佳实践
//   - 性能敏感场景预先 with_capacity()。
//   - 计数 / 累加用 entry API，比手写 if-else 干净。
//   - 函数签名收 &[T] 比 &Vec<T> 通用；收 &str 比 &String 通用。
//   - 想要有序键值对用 BTreeMap；需要插入顺序用 IndexMap（社区 crate）。
//
// 思考题
//   1. let s = String::from("a"); let r = &s[..]; r 的类型是？
//   2. HashMap 怎么换成"稳定输出顺序"的版本？
//   3. 下面两段哪个更地道，为什么？
//        let mut sum = 0; for x in v.iter() { sum += x; }
//        let sum: i32 = v.iter().sum();
// ============================================================
