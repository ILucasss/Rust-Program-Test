
pub fn vector() {
    let v= vec![1,2,3,4,5];

    let third: &i32 = &v[2];    //使用 & 和 [] 返回一个引用
    println!("The third element is {}",third);

    match v.get(2) {    //使用 get 方法以索引作为参数来返回一个 Option<&T>
        Some(third) => println!("The third element is {}",third),
        None => println!("There is no third element."),     //当 get 方法被传递了一个数组外的索引时，它不会 panic 而是返回 None
    }

    // error!!
    // 为什么第一个元素的引用会关心 vector 结尾的变化？
    // 不能这么做的原因是由于 vector 的工作方式：
    // 在 vector 的结尾增加新元素时，在没有足够空间将所有所有元素依次相邻存放的情况下，可能会要求分配新内存并将老的元素拷贝到新的空间中。
    // 这时，第一个元素的引用就指向了被释放的内存。借用规则阻止程序陷入这种状况。
    // let first = &v[0];
    // v.push(6);

    let mut v = vec![100, 32, 57];

    v.push(40);     //add element
    let m = v.pop();    //delete element
    println!("add and delete:{:?}",m);

    for i in &mut v {
        *i += 50;
        println!("{}",i);
    }

}