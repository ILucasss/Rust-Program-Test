pub fn borrowing() {
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用
    //若无创建作用域会发生编译报错,这个限制的好处是 Rust 可以在编译时就避免数据竞争。
    let r2 = &mut s;


    let mut s = String::from("hello");
    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);
    // 此位置之后 r1 和 r2 不再使用
    //我们 也 不能在拥有不可变引用的同时拥有可变引用。
    //不可变引用的用户可不希望在他们的眼皮底下值就被意外的改变了！
    //然而，多个不可变引用是可以的，因为没有哪个只能读取数据的人有能力影响其他人读取到的数据。
    let r3 = &mut s; // 没问题
    println!("{}", r3);
}