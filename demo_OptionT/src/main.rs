fn main() {
    println!("Hello, world!");

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = x + y.unwrap();
    println!("{}",sum);

    if let Some(5) = y {
        println!("yes");
    } else {
        println!("no");
    }
}
