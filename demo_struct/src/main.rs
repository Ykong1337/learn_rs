#[derive(Debug)]
struct Rectangle {
    width: usize,
    height: usize,
}

impl Rectangle {
    fn area(&self) -> usize {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size : usize) -> Rectangle{
        Rectangle{
            width: size,
            height: size
        }
    }
}


fn main() {
    println!("Hello, world!");

    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 35,
        height: 55,
    };

    let s = Rectangle::square(20);
    let area = rect2.area();

    println!("{}", rect.can_hold(&rect2));
    println!("{}", rect.can_hold(&rect3));
    println!("{}", rect.area());
    println!("{:#?}", rect);
    println!("{}",s.area());
    println!("{}",area);
}
