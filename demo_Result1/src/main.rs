use std::error::Error;
use std::fs::File;
use std::net::IpAddr;

fn main() -> Result<(),Box<dyn Error>> {
    println!("Hello, world!");

    let file = File::open("hello")?;

    let hom : IpAddr = "127.0.0.1".parse().unwrap();

    Ok(())
}
