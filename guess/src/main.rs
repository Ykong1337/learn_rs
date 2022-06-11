fn main() {
    println!("Hello, world!");

    loop {
        let guess = "32";

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        let guess = Guess::new(guess);
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("must be between 1 and 100");
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 { self.value }
}