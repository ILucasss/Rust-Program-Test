pub fn data_types() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    //元组

    let five_hundred = x.0;
    //解构元组

    let six_point_four = x.1;

    let one = x.2;
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    //数组
    let a = [3; 5];
    //等同于   let a = [3, 3, 3, 3, 3];
}
