extern crate core;

use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};

fn main() {
    println!("Hello, world!");
    let result = read_username();
    println!("{:?}",result);

    // let file = File::open("hello.txt");
    //
    // let f = match file {
    //     Ok(f) => f,
    //     Err(e) => match e.kind(){
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(f) => f,
    //             Err(e) => panic!("Err"),
    //         },
    //         _ => panic!("Error"),
    //     },
    // };


    // let file = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Error: {:?}", error);
    //         })
    //     } else {
    //         panic!("Error : {:?}", error)
    //     }
    // });

    // let file = File::open("hello.txt").unwrap();
    // let file = File::open("hello.txt").expect("无法找到文件");
}

fn read_username() -> Result<String,io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)

    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };
    //
    // let mut s = String::new();
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }
}
