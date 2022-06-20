fn main() {
    let result = do_twice(add_one, 5);
    println!("result: {:?}", result);
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}