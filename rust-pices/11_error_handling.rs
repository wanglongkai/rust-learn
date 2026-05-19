use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    // panic! 表示不可恢复错误，会终止当前线程。
    // panic!("发生严重错误");

    // Result<T, E> 用于可恢复错误。
    let greeting_file = File::open("hello.txt");

    let _file = match greeting_file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(created) => created,
                Err(create_error) => panic!("创建文件失败: {:?}", create_error),
            },
            other_error => panic!("打开文件失败: {:?}", other_error),
        },
    };

    match read_username_from_file("hello.txt") {
        Ok(name) if !name.trim().is_empty() => println!("用户名: {name}"),
        Ok(_) => println!("文件存在，但内容为空"),
        Err(error) => println!("读取失败: {error}"),
    }
}

fn read_username_from_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}
