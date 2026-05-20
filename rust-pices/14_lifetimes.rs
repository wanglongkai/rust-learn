// ============================================================
// 第 14 课：生命周期 (Lifetimes)
// ------------------------------------------------------------
// 学习目标：
//   1. 理解"生命周期"到底是什么：编译器追踪的引用有效期。
//   2. 学会读懂 <'a> 标注，知道什么时候必须手写它。
//   3. 了解生命周期省略规则——多数函数不需要写 'a。
//   4. 理解 'static 的含义。
// 前置知识：第 06 课引用与切片。
// 本课覆盖：'a 标注 / struct 中的引用 / 省略规则 / 'static。
// 运行方式：
//   rustc 14_lifetimes.rs && ./14_lifetimes
// ============================================================
//
// 核心概念：
//   每一个引用都有一个"生命周期"——它必须有效的代码区间。
//   编译器（借用检查器）确保引用始终指向"还活着"的数据。
//   多数情况下，编译器能自动推断；当存在歧义时，需要你显式标注。
//
//   生命周期标注 <'a> 不会改变实际的活跃时长，它只是"告诉编译器各引用之间的关系"。

// ------------------------------------------------------------
// 1. 经典例子：返回两个引用中较长的那个。
//    问题：编译器不知道返回的引用来自 x 还是 y，
//          也就不知道它必须"至少活多久"才合法。
//    解法：用 'a 表达 "x、y、返回值都活着同样长（取它们的交集）"。
// ------------------------------------------------------------
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// 反例：下面会编译失败 —— result 想活到 main 末尾，
// 但 string2 在内层 } 后就 drop 了。
fn dangle_demo() {
    let _string1 = String::from("abcd");
    // let result;
    // {
    //     let string2 = String::from("xy");
    //     result = longest(&_string1, &string2); // result 借了 string2
    // } // string2 在这里 drop
    // println!("{}", result); // 错：result 现在悬垂
}

// ------------------------------------------------------------
// 2. struct 里存引用 → 必须给 struct 标注生命周期
// ------------------------------------------------------------
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // 生命周期省略规则：&self 的生命周期会被自动赋给输出引用
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("请注意: {announcement}");
        self.part
    }
}

// ------------------------------------------------------------
// 3. 省略规则（compiler elision rules）——为什么大多数函数不用写 'a
//
//   规则 1：每个引用参数得到独立的生命周期。
//     fn foo(x: &T, y: &U)  ≈  fn foo<'a, 'b>(x: &'a T, y: &'b U)
//   规则 2：只有一个输入引用时，输出引用与之相同。
//     fn foo(x: &T) -> &U  ≈  fn foo<'a>(x: &'a T) -> &'a U
//   规则 3：有 &self 时，输出引用的生命周期与 &self 相同。
//
//   不满足以上三条 → 必须手写 'a。
// ------------------------------------------------------------

// 满足规则 2：不需要写 'a
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

// 满足规则 3：&self → 输出 &str 自动跟 self 一样
// (见上面的 announce_and_return_part)

// 不满足三条规则：必须显式标注（就是 longest 的例子）

// ------------------------------------------------------------
// 4. 'static：贯穿整个程序运行期间的生命周期
//    - 字符串字面量类型是 &'static str：存于二进制只读段。
//    - 不是"魔法续命卡"！用 &'static 时小心限制——通常意味着数据必须全局存在。
// ------------------------------------------------------------
fn static_demo() -> &'static str {
    "我活到程序结束"
}

// ------------------------------------------------------------
// 5. 综合：泛型 + trait bound + 生命周期 同时存在的写法
//    顺序约定：生命周期参数写在前，类型参数写在后。
// ------------------------------------------------------------
use std::fmt::Display;

fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("通知: {ann}");
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let s1 = String::from("abcd");
    let s2 = "xyz";
    let result = longest(s1.as_str(), s2);
    println!("较长字符串 = {result}");

    let novel = String::from("第一句话。第二句话。");
    let first_sentence = novel.split('。').next().expect("至少要有一句话");
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };
    println!("摘录: {}", excerpt.announce_and_return_part("注意"));

    println!("第一个单词: '{}'", first_word(s1.as_str()));

    let s: &'static str = static_demo();
    println!("static: {s}");

    let chosen = longest_with_announcement("apple", "banana-tree", "新提示");
    println!("chosen = {chosen}");

    dangle_demo(); // 只是为了让函数被引用、避免警告
}

// ============================================================
// 本课小结
//   - 生命周期是"引用必须有效"的编译期保证，不会影响运行时。
//   - 当函数签名里"输入引用和输出引用的关系不明确"时，你必须显式写 'a。
//   - struct 里存引用时，必须给 struct 加生命周期参数。
//   - 'static 表示"整个程序运行期"，常见于字符串字面量。
//
// 常见陷阱
//   - 看见 'a 就害怕：先观察省略规则——多数情况下根本不用写。
//   - 想让引用"活久一点"硬加 'static → 实际数据并不存在那么久 → 编译错误。
//   - struct 字段里存了引用，但忘了给 struct 加 <'a> → 编译错。
//   - 不能在函数里返回"指向局部变量的引用"——返回拥有所有权的值，或者让调用方传入。
//
// 最佳实践
//   - 优先用拥有所有权的类型 (String、Vec) 简化签名；只有性能 / API 风格强烈要求时才存引用。
//   - 不要无脑给所有东西标 'a；让编译器先报错，根据它的提示再加最少的标注。
//   - 把生命周期标注当作"和编译器对话的注释"，越短越好。
//   - 慎用 &'static：通常是把"应该用 Arc<String>"的场景错误简化的信号。
//
// 思考题
//   1. fn first<'a>(v: &'a [&'a str]) -> &'a str 这种"重复 'a"是必须的吗？
//   2. struct Wrapper { s: &'static str } 和 struct Wrapper<'a> { s: &'a str } 各适合什么场景？
//   3. 为什么 fn longest(x: &str, y: &str) -> &str 不能直接编译？怎么改？
// ============================================================
