pub fn string(){
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用
    println!("greeting:{}",s3);

    let s1 = String::from("Hello, ");
    //format! 与 println! 的工作原理相同,不过不同于将输出打印到屏幕上，它返回一个带有结果内容的 String。
    let s = format!("{}-{}",s1,s2);
    println!("format greeting:{}",s3);

    //对 “नमस्ते” 调用 chars 方法会将其分开并返回六个 char 类型的值，接着就可以遍历其结果来访问每一个元素了
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    //bytes 方法返回每一个原始字节，这可能会适合你的使用场景
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
// 总而言之，字符串还是很复杂的。
// 不同的语言选择了不同的向程序员展示其复杂性的方式。
// Rust 选择了以准确的方式处理 String 数据作为所有 Rust 程序的默认行为，
// 这意味着程序员们必须更多的思考如何预先处理 UTF-8 数据。
// 这种权衡取舍相比其他语言更多的暴露出了字符串的复杂性，
// 不过也使你在开发生命周期后期免于处理涉及非 ASCII 字符的错误。