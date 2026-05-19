#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("处理消息: {:?}", self);
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("home={:?}, loopback={:?}", home, loopback);
    describe_ip(&home);
    describe_ip(&loopback);

    let messages = [
        Message::Quit,
        Message::Move { x: 3, y: 4 },
        Message::Write(String::from("hello")),
        Message::ChangeColor(255, 0, 0),
    ];

    for message in &messages {
        message.call();
        match message {
            Message::Quit => println!("退出"),
            Message::Move { x, y } => println!("移动到 ({x}, {y})"),
            Message::Write(text) => println!("写入文本: {text}"),
            Message::ChangeColor(r, g, b) => println!("颜色 RGB({r}, {g}, {b})"),
        }
    }

    // Option 用于表达“可能有值，也可能没有值”。
    let some_number = Some(5);
    let no_number: Option<i32> = None;
    println!("some + 1 = {:?}", plus_one(some_number));
    println!("none + 1 = {:?}", plus_one(no_number));

    // if let 适合只关心一种匹配情况。
    if let Some(value) = some_number {
        println!("if let 取到值: {value}");
    }
}

fn plus_one(value: Option<i32>) -> Option<i32> {
    match value {
        Some(x) => Some(x + 1),
        None => None,
    }
}

fn describe_ip(ip: &IpAddr) {
    match ip {
        IpAddr::V4(a, b, c, d) => println!("IPv4 地址: {a}.{b}.{c}.{d}"),
        IpAddr::V6(addr) => println!("IPv6 地址: {addr}"),
    }
}
