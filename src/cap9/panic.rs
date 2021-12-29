use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;

pub fn panic() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    //如果调用这段代码时不存在 hello.txt 文件，
    // 我们将会看到一个 unwrap 调用 panic! 时提供的错误信息
    let f = File::open("hello.txt").unwrap();

    //还有另一个类似于 unwrap 的方法它还允许我们选择 panic! 的错误信息：expect.
    // 使用 expect 而不是 unwrap 并提供一个好的错误信息可以表明你的意图并更易于追踪 panic 的根源。
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

// 如果 Result 的值是 Ok，这个表达式将会返回 Ok 中的值而程序将继续执行。
// 如果值是 Err，? 运算符会提早返回整个函数并将一些 Err 值传播给调用者,
// Err 中的值将作为整个函数的返回值
pub fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}