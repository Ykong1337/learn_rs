fn main() {
    println!("Hello, world!");

    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
}

#[test]
fn test() {
    let sum: i32 = Counter::new()
        .zip(Counter::new()
            .skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18,sum);
}

struct Counter {
    count: i32,
}

impl Counter {
    fn new() -> Counter {
        Counter {
            count: 0
        }
    }
}

impl Iterator for Counter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}