#[derive(Debug)]
pub struct Rectangle {
    length: usize,
    width: usize,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hold_smaller() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 5,
            width: 3,
        };

        assert!(larger.can_hold(&smaller));
        assert_eq!(smaller.can_hold(&larger), false);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("no"))
        }
    }
}


fn main() {
    println!("Hello, world!");
}
