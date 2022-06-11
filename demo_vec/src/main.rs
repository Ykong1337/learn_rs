fn main() {

    let mut v1 = vec![1,3,5];

    v1.push(70);
    println!("{}",v1.len());

    for i in &mut v1 {
        *i += 50;
    }

    for i in &mut v1 {
        println!("{}",i);
    }

    println!("{:?}",v1);

}