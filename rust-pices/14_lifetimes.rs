// 生命周期标注描述引用之间的有效期关系，不会改变真实生命周期。
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("请注意: {announcement}");
        self.part
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("较长字符串是 {result}");

    let novel = String::from("很久很久以前。这里是第二句。");
    let first_sentence = novel
        .split('。')
        .next()
        .expect("应该能找到第一句");

    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };
    println!("摘录: {}", excerpt.announce_and_return_part("生命周期示例"));

    // 'static 表示引用在整个程序运行期间都有效。
    let static_text: &'static str = "我存储在程序二进制中";
    println!("{static_text}");
}
