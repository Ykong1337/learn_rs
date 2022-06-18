fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                options: vec![],
            }),
        ],
    };
    screen.run();
}

pub trait Draw {
    fn draw(&self);
}

pub trait Clone {
    fn clone(&self) -> Self;
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in &self.components {
            component.draw();
        }
    }
}

pub struct Button {
    width: usize,
    height: usize,
    options: Vec<String>,
}

impl Draw for Button {
    fn draw(&self) {}
}

struct SelectBox {
    width: usize,
    height: usize,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {}
}