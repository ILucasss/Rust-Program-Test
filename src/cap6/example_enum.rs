//Rust 并没有空值，不过它确实拥有一个可以编码存在或不存在概念的枚举。
// 这个枚举是 Option<T>，而且它定义于标准库中，如下:
// enum Option<T> {
//     Some(T),
//     None,
// }

pub fn option(){
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    //因为 Option<T> 和 T（这里 T 可以是任何类型）是不同的类型，编译器不允许像一个肯定有效的值那样使用 Option<T>。
    // 例如，这段代码不能编译，因为它尝试将 Option<i8> 与 i8 相加
    //  let x:i8 = 5;
    //  let sum = x+some_number;
}



//一个 Message 枚举，其每个成员都存储了不同数量和类型的值
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

//如下这些结构体可以包含与之前枚举成员中相同的数据
//不过，如果我们使用不同的结构体，由于它们都有不同的类型，我们将不能像使用以上定义的 Message 枚举那样，
//轻易的定义一个能够处理这些不同类型的结构体的函数，因为枚举是单独一个类型。
struct QuitMessage; // 类单元结构体
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // 元组结构体
struct ChangeColorMessage(i32, i32, i32); // 元组结构体

//结构体和枚举还有另一个相似点：就像可以使用 impl 来为结构体定义方法那样，也可以在枚举上定义方法。
// 这是一个定义于我们 Message 枚举上的叫做 call 的方法
impl Message {
    fn call(&self) {
        println!("Message call {:?}",self);
    }
}
pub fn impl_meun() {
    let m = Message::Write(String::from("hello"));
    m.call();
}
