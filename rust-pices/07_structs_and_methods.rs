// ============================================================
// 第 07 课：结构体与方法 (Structs & Methods)
// ------------------------------------------------------------
// 学习目标：
//   1. 掌握三种结构体：具名字段 / 元组结构体 / 单元结构体。
//   2. 学会用 impl 块为结构体定义方法和关联函数。
//   3. 理解 #[derive(...)] 自动派生常用 trait。
//   4. 学会用 newtype 给基础类型加业务语义。
// 前置知识：第 05、06 课所有权和借用。
// 本课覆盖：struct / impl / &self / 关联函数 / derive / newtype。
// 运行方式：
//   rustc 07_structs_and_methods.rs && ./07_structs_and_methods
// ============================================================

// ------------------------------------------------------------
// 1. 具名字段结构体：最常见的形式。
// #[derive(Debug)] 让我们能用 {:?} 打印它（不写就不能）。
// ------------------------------------------------------------
#[derive(Debug, Clone)]
#[allow(dead_code)] // 教学示例：字段定义出来主要为了演示，未必都读
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

// ------------------------------------------------------------
// 2. 元组结构体：字段没有名字，只有顺序。
// 常用于"轻量包装"——给基础类型加语义（newtype 模式）。
// ------------------------------------------------------------
#[derive(Debug, Clone, Copy)]
struct Point(f64, f64);

// newtype：让 UserId 和 OrderId 在类型系统里不能互换
#[derive(Debug, Clone, Copy)]
struct UserId(u64);
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
struct OrderId(u64);

// ------------------------------------------------------------
// 3. 单元结构体：没有任何字段，类似 ()。
// 常用于"只用作 trait 标记"的占位类型。
// ------------------------------------------------------------
struct AlwaysEqual;

// ------------------------------------------------------------
// 4. 方法 (Method) 和关联函数 (Associated Function)
// ------------------------------------------------------------
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 4.1 实例方法：第一个参数是 self / &self / &mut self。
    //     一般用 &self（只读访问），需要修改时用 &mut self，
    //     需要"消耗"实例时用 self（少见）。
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    // 4.2 &mut self：可以修改字段
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }

    // 4.3 关联函数：没有 self 参数。用 Type::name() 调用。
    //     习惯上，构造函数命名为 new；其它构造器自由命名（square、from_xxx）。
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
    fn square(size: u32) -> Self {
        Self::new(size, size)
    }
}

// impl 块可以分散在多处，编译期合并 —— 利于拆分大类型。
impl Rectangle {
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

fn main() {
    // ---------- 5. 创建实例 ----------
    let user = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        active: true,
        sign_in_count: 1,
    };
    println!("user = {:?}", user);
    println!("user.email = {}", user.email);

    // ---------- 6. 字段简写：变量名与字段名相同可省略 ----------
    let username = String::from("bob");
    let email = String::from("bob@example.com");
    let user_b = User {
        username,        // 等价于 username: username
        email,
        active: true,
        sign_in_count: 0,
    };
    println!("user_b = {:?}", user_b);

    // ---------- 7. 结构体更新语法 ..other ----------
    // 把没列出的字段从另一实例"复用"过来。
    // 注意：这是 move！能改的字段（如 String）会被 move 走。
    let user_c = User {
        email: String::from("alice2@example.com"),
        ..user.clone()
    };
    println!("user_c = {:?}", user_c);

    // ---------- 8. 元组结构体与 newtype ----------
    let p = Point(3.0, 4.0);
    println!("Point = ({}, {})", p.0, p.1);

    // 编译器现在阻止你把 UserId 当 OrderId
    let uid = UserId(1);
    let _oid = OrderId(1);
    // let _x: OrderId = uid; // 错：类型不匹配（这就是 newtype 的价值）
    print_user_id(uid);

    // ---------- 9. 调用方法与关联函数 ----------
    let rect1 = Rectangle::new(30, 50);
    let rect2 = Rectangle { width: 10, height: 40 };
    let mut square = Rectangle::square(20);

    println!("rect1 面积 = {}, 周长 = {}", rect1.area(), rect1.perimeter());
    println!("rect1 能装 rect2 吗？{}", rect1.can_hold(&rect2));

    square.scale(2);
    println!("放大后的 square = {:?}", square);

    let _ = AlwaysEqual; // 单元结构体只用作标记
}

fn print_user_id(id: UserId) {
    println!("UserId = {}", id.0);
}

// ============================================================
// 本课小结
//   - 三种 struct：具名 / 元组 / 单元，覆盖几乎所有数据建模需求。
//   - impl 块定义方法（带 self）和关联函数（无 self）。
//   - #[derive(Debug, Clone, Copy, ...)] 可自动实现常用 trait。
//   - newtype 模式让"看起来一样的基础类型"在类型系统里分开。
//
// 常见陷阱
//   - 没 derive Debug 就 println!("{:?}", x) → 编译错误。
//   - 结构体更新语法 `..other` 会 move（除非字段都是 Copy），原变量后续不能再用。
//   - 把字段全 pub → 破坏封装，后续重构成本高。
//
// 最佳实践
//   - 公共类型至少 derive Debug；能 derive Clone / PartialEq 就 derive。
//   - 用 newtype 包裹"看起来像 u64 但语义不同"的 ID、金额、距离。
//   - 字段默认私有，通过方法暴露读/写——便于将来加校验。
//   - 构造函数惯用 new；多种构造器用 from_xxx / with_xxx 命名。
//
// 思考题
//   1. Rectangle::new(...) 和 Rectangle { width, height } 有什么区别？
//   2. 在什么情况下你会写一个 fn into_xxx(self) -> Other 这种"消耗 self"的方法？
//   3. 把 UserId 改成 struct UserId { id: u64 } 和 struct UserId(u64) 各有什么优劣？
// ============================================================
