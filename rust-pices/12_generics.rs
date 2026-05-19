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

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let mixed = MixedPoint { x: 5, y: 4.0 };

    println!("integer = {:?}, x = {}", integer, integer.x());
    println!("float = {:?}, distance = {}", float, float.distance_from_origin());
    println!("mixed = {:?}", mixed);

    let list = vec![34, 50, 25, 100, 65];
    println!("最大值 = {}", largest(&list));
}

// 泛型函数可复用于多种类型。
// PartialOrd 支持比较，Copy 允许按值返回元素。
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
