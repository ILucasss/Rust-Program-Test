use std::collections::HashMap;

pub fn hash_map() {
    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    for(key,value) in &scores {
        println!("{} {}",key,value);
    }


    //使用 entry 方法只在键没有对应一个值时插入
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    // 会打印出 {"Yellow": 50, "Blue": 10}。
    // 第一个 entry 调用会插入黄队的键和值 50，因为黄队并没有一个值。
    // 第二个 entry 调用不会改变哈希 map 因为蓝队已经有了值 10。
    println!("{:?}", scores);




    //or_insert 方法事实上会返回这个键的值的一个可变引用（&mut V）。
    // 这里我们将这个可变引用储存在 count 变量中，所以为了赋值必须首先使用星号（*）解引用 count。
    // 这个可变引用在 for 循环的结尾离开作用域，这样所有这些改变都是安全的并符合借用规则。
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

