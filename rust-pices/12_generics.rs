// ============================================================
// 第 12 课：泛型 (Generics)
// ------------------------------------------------------------
// 学习目标：
//   1. 学会给函数、struct、enum、impl 加泛型参数。
//   2. 理解 trait bound（约束）的作用——为什么"裸 T"不能比较、不能 +。
//   3. 了解"单态化 (monomorphization)"：泛型的零成本抽象原理。
// 前置知识：第 07 课 struct，第 13 课 trait 会接续讲约束。
// 本课覆盖：<T> / 多参数泛型 / impl<T> / where / 单态化。
// 运行方式：
//   rustc 12_generics.rs && ./12_generics
// ============================================================

// ------------------------------------------------------------
// 1. 泛型函数：找出列表中的最大值
//    "T 必须能比较 (PartialOrd) 且能按值复制 (Copy)"
//    —— 没有约束的话，T 没有任何可用方法。
// ------------------------------------------------------------
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// 同样的函数，写成 where 子句——多约束时更易读
fn largest_v2<T>(list: &[T]) -> T
where
    T: PartialOrd + Copy,
{
    let mut max = list[0];
    for &x in list {
        if x > max {
            max = x;
        }
    }
    max
}

// ------------------------------------------------------------
// 2. 泛型结构体：同/异构泛型参数
// ------------------------------------------------------------
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct MixedPoint<T, U> {
    x: T,
    y: U,
}

// ------------------------------------------------------------
// 3. impl 块：所有 T 都可用的方法
// ------------------------------------------------------------
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 只对 T = f64 实现的方法 —— 编译器只允许 Point<f64> 调用 distance_from_origin
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// 多参数 impl：泛型方法也可以引入新的泛型
impl<T, U> MixedPoint<T, U> {
    fn mixup<V, W>(self, other: MixedPoint<V, W>) -> MixedPoint<T, W> {
        MixedPoint {
            x: self.x,
            y: other.y,
        }
    }
}

// ------------------------------------------------------------
// 4. 泛型枚举：Option<T> 和 Result<T, E> 就是最经典的例子。
// ------------------------------------------------------------
#[derive(Debug)]
#[allow(dead_code)]
enum MyOption<T> {
    Some(T),
    None,
}

fn main() {
    // ---- 5. 调用泛型函数 ----
    let nums = vec![34, 50, 25, 100, 65];
    let chars = vec!['y', 'm', 'a', 'q'];
    println!("最大数 = {}", largest(&nums));
    println!("最大字符 = {}", largest_v2(&chars));

    // ---- 6. 泛型结构体 ----
    let integer = Point { x: 5, y: 10 };
    let floating = Point { x: 1.0, y: 4.0 };
    println!("integer = {:?}, .x() = {}", integer, integer.x());
    println!(
        "floating = {:?}, distance = {}",
        floating,
        floating.distance_from_origin()
    );

    let mixed = MixedPoint { x: 1, y: "hello" };
    let other = MixedPoint { x: 2.5, y: 'z' };
    let mixed_up = mixed.mixup(other);
    println!("mixed_up = {:?}", mixed_up);

    let opt: MyOption<i32> = MyOption::Some(42);
    println!("opt = {:?}", opt);

    // ------------------------------------------------------------
    // 7. 单态化 (monomorphization)：泛型为何零成本
    // ------------------------------------------------------------
    //   编译器看到 largest::<i32>、largest::<char>，会"复制"出两份具体函数：
    //     fn largest_i32(list: &[i32]) -> i32 { ... }
    //     fn largest_char(list: &[char]) -> char { ... }
    //   运行时没有"泛型查找"开销，等价于你手写两份。
    //   代价：二进制体积会随泛型实例化的种类增大（叫"代码膨胀"）。
    // ------------------------------------------------------------
}

// ============================================================
// 本课小结
//   - 泛型让函数 / 类型可以"对一类类型工作"，类型安全且零运行期成本。
//   - 用 trait bound 给 T 加能力：能比较、能加、能打印……
//   - 单态化 = 编译期为每个具体类型生成一份代码，没有运行时开销。
//
// 常见陷阱
//   - 直接对裸 T 用 >、+、{} 打印 → 报错"trait not satisfied"，需要加 bound。
//   - 泛型实例化太多导致二进制膨胀 → 把热点抽象在内部，外部 API 用单一类型。
//   - 把"应该用 trait object (dyn Trait)"的场景全写成 <T: Trait> → 编译时间爆炸。
//
// 最佳实践
//   - 公共 API 的入参尽量泛型 (impl AsRef<str>, impl IntoIterator)，让调用方少 .to_string()。
//   - 多约束用 where 子句，可读性远好于一长串 <T: A + B + C, U: D + E>。
//   - 真的需要"同一函数处理多个完全不同的类型 + 在容器里混着存"时，
//     才考虑 trait object (Box<dyn Trait>)，下一课会讲。
//   - 给基础类型加业务语义优先用 newtype（第 07 课）而不是滥用泛型。
//
// 思考题
//   1. fn largest<T>(list: &[T]) -> T 直接这样写为什么编译不过？
//   2. 单态化和 C++ 模板、Java 泛型有什么相似/不同？
//   3. 下面两个 fn 哪个更可能让二进制变大，为什么？
//        fn run<T: Display>(x: T) { println!("{x}"); }
//        fn run(x: &dyn Display)  { println!("{x}"); }
// ============================================================
