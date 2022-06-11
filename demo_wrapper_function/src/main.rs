fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    println!("Hello, world!");

    let char_list = vec!['a','t','h','p'];
    let result1 = largest(&char_list);
    println!("the largest char is {}",&result1);

    let number_list = vec![1,3,5,8,110,1,4,7,22];
    let result = largest(&number_list);
    println!("The largest number is {}",&result);
}

#[test]
fn test(){
    let char_list = vec!['a','t','h','p'];
    let result1 = largest(&char_list);
    // println!("the largest char is {}",&result1);
    assert!("{}","{}",'t');
}

#[test]
fn test1(){
    panic!("error");
}
