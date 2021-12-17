#[derive(Debug)]
struct Rectangle {
    width: u32,
    height:u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self,other:&Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size:u32) -> Rectangle {
        Rectangle {
            width:size,
            height:size
        }
    }
}

pub fn example() {
    let rect1 = Rectangle {
        width:30,
        height:50
    };

    let rect2 = Rectangle::square(20);
    let rect3 = Rectangle::square(40);

    println!("rect1 is{:?}",rect1);
    println!("rect1 is{:#?}",rect2);
    dbg!(&rect3);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}",rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}",rect1.can_hold(&rect3));

}