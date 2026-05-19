fn main() {
    // 标量类型：整数、浮点数、布尔值、字符。
    let age: u8 = 30;
    let temperature: f64 = 36.6;
    let is_active: bool = true;
    let heart: char = '爱';

    println!("age={age}, temperature={temperature}, is_active={is_active}, heart={heart}");

    // 整数字面量可使用后缀和分隔符。
    let decimal = 98_222;
    let hex = 0xff;
    let binary = 0b1111_0000;
    let byte = b'A';
    println!("decimal={decimal}, hex={hex}, binary={binary}, byte={byte}");

    // 元组 tuple：固定长度，可包含不同类型。
    let user: (&str, u32, bool) = ("Alice", 18, true);
    let (name, level, enabled) = user;
    println!("name={name}, level={level}, enabled={enabled}");
    println!("也可以按索引访问 tuple: {}", user.0);

    // 数组 array：固定长度，元素类型相同。
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("第一个数字 = {}", numbers[0]);

    // [初始值; 长度] 可创建重复元素数组。
    let zeroes = [0; 3];
    println!("zeroes = {:?}", zeroes);
}
