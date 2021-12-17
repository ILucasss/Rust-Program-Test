//error:not found in this scope
//缺少此处结构体定义(先定义后赋值
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

//因为 email 字段与 email 参数有着相同的名称，则只需编写 email 而不是 email: email。
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

pub fn structs() {
    let user1 = build_user(String::from("yujunchengxxx@126.com"),String::from("Lucas"));
    println!("{}",user1.username);
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!("{}",user1.email);
    //结构体更新语法（struct update syntax）为移动数据,username由user1移动到user2,user1的username变量无法使用
    //println!("{}",user1.username);
}

