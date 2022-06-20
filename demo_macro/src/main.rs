use macro_define::HelloMacro;

fn main() {
    println!("Hello, world!");
    Pancakes::hello_macro();

}

trait HelloMacro{
    fn hello_macro();
}

#[derive(HelloMacro)]
struct Pancakes;

