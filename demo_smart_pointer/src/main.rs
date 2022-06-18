use std::ops::Deref;
use crate::List::{Nil, Node};

fn main() {
    println!("Hello, world!");

    let b = Box::new(5);

    println!("b = {}", b);

    let list = Node(1, Box::new(Node(2, Box::new(Node(3, Box::new(Nil))))));
    println!("{:?}", list);
}

#[derive(Debug)]
enum List {
    Node(i32, Box<List>),
    Nil,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    Color(i32, i32, i32),

}

struct My_box<T>(T);

impl<T> Deref for My_box<T>{
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> My_box<T> {
    fn new(x: T) -> My_box<T> {
        My_box(x)
    }
}

#[test]
fn test() {
    let x = 5;
    let y = My_box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}