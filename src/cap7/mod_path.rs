pub fn mod_path() {
    println!("please goto cap7::mod_path for reading the comment line");
    // 如果我们在一个结构体定义的前面使用了 pub ，这个结构体会变成公有的，
    // 但是这个结构体的字段仍然是私有的。我们可以根据情况决定每个字段是否公有
    // mod back_of_house {
    //     pub struct Breakfast {
    //         pub toast: String,
    //         seasonal_fruit: String,
    //     }
    //
    //     impl Breakfast {
    //         pub fn summer(toast: &str) -> Breakfast {
    //             Breakfast {
    //                 toast: String::from(toast),
    //                 seasonal_fruit: String::from("peaches"), //certainly not
    //             }
    //         }
    //     }
    // }
    //
    // pub fn eat_at_restaurant() {
    //     // Order a breakfast in the summer with Rye toast
    //     let mut meal = back_of_house::Breakfast::summer("Rye");
    //     // Change our mind about what bread we'd like
    //     meal.toast = String::from("Wheat");
    //     println!("I'd like {} toast please", meal.toast);

        // The next line won't compile if we uncomment it; we're not allowed
        // to see or modify the seasonal fruit that comes with the meal
        // meal.seasonal_fruit = String::from("blueberries");
    // }


    //与之相反，如果我们将枚举设为公有，则它的所有成员都将变为公有。我们只需要在 enum 关键字前面加上 pub
    // mod back_of_house {
    //     pub enum Appetizer {
    //         Soup,
    //         Salad,
    //     }
    // }
    //
    // pub fn eat_at_restaurant() {
    //     let order1 = back_of_house::Appetizer::Soup;
    //     let order2 = back_of_house::Appetizer::Salad;
    // }
}

pub fn re_export() {
    println!("re_export success!");
}