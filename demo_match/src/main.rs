
fn main() {
    println!("Hello, world!");

    let c = Coin::Quarter(UsState::Alaska);
    println!("{}",value_in_cents(c));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?},{:?},{:?}",five,six,none);


    let v = Some(0u8);
    match v {
        Some(3) =>println!("Three"),
        _ => println!("other")
    };

    if let Some(3) = v {
        println!("three");

    } else {
        ()
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1)
    }
}

#[derive(Debug)]
enum UsState {
    Alabaman,
    Alaska,
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8{
    match coin{
        Coin::Penny => {
            println!("Penny");
            1
        }
        Coin::Nickel => {5}
        Coin::Dime => {10}
        Coin::Quarter(state) => {
            println!("State Quarter from {:?}", state);
            25
        }
    }
}
