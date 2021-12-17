pub fn statements() {
    let x = plus_one(5);
    println!("The value of x is:{}",x);
}
fn plus_one(x:i32)->i32 {
    x+1
    //语句与表达式的区别,语句以;结尾,不返回值,表达式存在返回值
}