pub fn example_match() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    //想象我们正在玩一个游戏，
    //如果你掷出骰子的值为 3，角色不会移动，而是会得到一顶新奇的帽子。
    //如果你掷出了 7，你的角色将失去新奇的帽子。
    //对于其他的数值，你的角色会在棋盘上移动相应的格子。
    //other涵盖了所有其他可能的值,other 分支的代码通过将其传递给 move_player 函数来使用这个变量。
    //_是一个特殊的模式，可以匹配任意值而不绑定到该值。
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),//抛3和7以外移动一格
        _ => (),//当你掷出的值不是 3 或 7 的时候，无事发生,
        // 由于other分支存在! _现无效
    }

}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,//若无这行代码程序报错
        //Rust 中的匹配是 穷尽的（exhaustive）：必须穷举到最后的可能性来使代码有效。
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {
    println!("Congratulations on getting a fancy hat");
}
fn remove_fancy_hat() {
    println!("I'm sorry you lost a fancy hat");
}
fn move_player(num_spaces: u8) {
    println!("You can move {}",num_spaces);
}




