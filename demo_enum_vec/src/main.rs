fn main() {
    let row = vec![
        sheet::Int(7),
        sheet::Float(5.6),
        sheet::Text(String::from("Hello, world!")),
    ];

    println!("{:?}",row);
}

#[derive(Debug)]
enum sheet {
    Int(i32),
    Float(f64),
    Text(String),
}
