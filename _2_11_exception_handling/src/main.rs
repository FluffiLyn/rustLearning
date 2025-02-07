use std::net::IpAddr;
fn main() {
    println!("Hello, world!");
    let home: IpAddr = "127.0.0.1".parse().unwrap();
    println!("home: {:?}", home);
}
