// 本文件演示模块系统。真正的包管理通常由 Cargo.toml 管理。
// 可运行：rustc 10_modules_and_packages.rs

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("加入等位列表");
        }

        pub fn seat_at_table() {
            println!("安排入座");
        }
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }

        pub fn describe(&self) {
            println!("早餐: toast={}, fruit={}", self.toast, self.seasonal_fruit);
        }
    }
}

use crate::front_of_house::hosting;

fn main() {
    // crate 是当前编译单元的根模块。
    crate::front_of_house::hosting::add_to_waitlist();

    // use 可把路径引入当前作用域。
    hosting::seat_at_table();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    meal.describe();

    // Cargo 常用命令：
    // cargo new project_name    创建包
    // cargo build               构建
    // cargo run                 构建并运行
    // cargo test                运行测试
    // cargo add crate_name      添加依赖（需要安装 cargo-edit 或新版 Cargo 支持）
}
