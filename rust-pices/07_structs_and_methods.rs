#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 方法的第一个参数通常是 &self，表示借用当前实例。
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    // 关联函数没有 self，常用作构造函数。
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let user = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        active: true,
        sign_in_count: 1,
    };
    println!("user = {:?}", user);
    println!(
        "{} <{}>, active={}, sign_in_count={}",
        user.username, user.email, user.active, user.sign_in_count
    );

    // 结构体更新语法可复用其他实例的字段。
    let user2 = User {
        email: String::from("new@example.com"),
        ..user
    };
    println!("user2 = {:?}", user2);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let square = Rectangle::square(20);

    println!("rect1 面积 = {}", rect1.area());
    println!("rect1 能容纳 rect2 吗？{}", rect1.can_hold(&rect2));
    println!("square = {:?}", square);
}
