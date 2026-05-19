trait Summary {
    fn summarize(&self) -> String;

    // trait 可以提供默认方法实现。
    fn author(&self) -> String {
        String::from("未知作者")
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}，来自 {}，作者 {}", self.headline, self.location, self.author)
    }

    fn author(&self) -> String {
        self.author.clone()
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Rust 学习路线"),
        location: String::from("Shanghai"),
        author: String::from("Alice"),
    };

    let tweet = Tweet {
        username: String::from("rustacean"),
        content: String::from("Ownership is powerful!"),
    };

    notify(&article);
    notify(&tweet);
    notify_pair(&article, &tweet);
}

// impl Trait 是 trait bound 的简洁写法。
fn notify(item: &impl Summary) {
    println!("通知: {}", item.summarize());
    println!("作者: {}", item.author());
}

// where 子句让复杂 trait bound 更易读。
fn notify_pair<T, U>(first: &T, second: &U)
where
    T: Summary,
    U: Summary,
{
    println!("第一条: {}", first.summarize());
    println!("第二条: {}", second.summarize());
}
