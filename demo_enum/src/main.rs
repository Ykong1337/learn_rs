fn main() {
    println!("Hello, world!");

    let q = MessageType::Quit;

    let m = MessageType::Move {
        x: 6,
        y: 7,
    };

    let w = MessageType::Write(String::from("Hello, world!"));

    let c = MessageType::ChangeColor(0,255,255);

    let some : Option<i32> = None;
    m.call();

    println!("{:?}",MessageType::Quit);
}

#[derive(Debug)]
enum MessageType {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl MessageType {
    fn call(&self){
        println!("call method");
    }

}
