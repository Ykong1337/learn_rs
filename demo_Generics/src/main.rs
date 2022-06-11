fn main() {
    println!("Hello, world!");

    let integer = Point{x:5,y:10};
    let float = Point{x:1.0,y:3.0};

    println!("{:?},{:?}",integer.x,Point::get_x(&float));
}

struct Point<T>{
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn get_x(&self) -> &T{
        &self.x
    }
}
