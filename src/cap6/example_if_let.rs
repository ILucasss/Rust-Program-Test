
pub fn example_if_let(option_number:Option<u8>) {
    if let None = option_number{
        println!("{:?}",option_number);
    } else {
        println!("option_number is {:?}",option_number);
    }
}