
fn main() {

    let data = "hello world!!";

    let mut s = data.to_string();

    let s1 = String::from("Hello Ykong!");

    s.push_str(&s1);

    let s3 = format!("{}-{}", s,s1);

    println!("{}",s3);
}
