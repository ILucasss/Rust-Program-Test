#[derive(Debug)]
enum IpAddr {
    v4(String),
    v6(String),
}

pub fn define_enum() {
    let  home = IpAddr::v4(String::from("127.0.0.1"));
    let lockback = IpAddr::v6(String::from("::1"));
    println!("Ipv4 is {:?}",home);
    println!("Ipv6 is {:?}",lockback);
}