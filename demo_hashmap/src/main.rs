use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let mut map = HashMap::new();

    map.insert("hello",10);
    map.insert("world!",20);
    map.insert("hello",30);

    println!("map = {:?}",map);

    // let teams = vec![String::from("Blue"),String::from("Green")];
    // let score = vec![10,50];
    //
    // let sc : HashMap<_,_> = teams.iter().zip(score.iter()).collect();
    //
    // println!("{:?}",sc);

    let hit = map.get("hello");

    match hit {
        Some(s) => println!("{}",s),
        _ => println!("no exist"),
    }

    for (k,v) in &map {
        println!("{},{}",k,v);
    }

    let text = "hello world my world !";

    let mut map1 = HashMap::new();

    for word in text.split_whitespace() {
        let count = map1.entry(word).or_insert(0);
        *count += 1;
    }

    //有# 纵向输出 ， 无# 横向输出
    println!("{:?}",map1);
    println!("{:#?}",map1);
}
