pub fn ownership() {
    //移动,此代码会报错,s2 = s1后rust认为s1失效(防止drop二次释放)
    // let s1 = String::from("hello");
    // let s2 = s1;
    //
    // println!("{}, world!", s1);

    //拷贝操作(会更消耗资源)
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    //本身属于Copy类型
    //所有整数类型，比如 u32。
    // 布尔类型，bool，它的值是 true 和 false。
    // 所有浮点数类型，比如 f64。
    // 字符类型，char。
    // 元组，当且仅当其包含的类型也都实现 Copy 的时候。比如，(i32, i32) 实现了 Copy，但 (i32, String) 就没有。
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}