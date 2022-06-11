
fn main() {

    struct User {
        id: i32,
        username: String,
    }

    let mut s1 = String::from("hello world");
    s1.push_str("now");
    let index = find_string(&s1);
    println!("{}", index);
    println!("{}", s1);

    let s = &s1[..];
    println!("{}", s);

    let mut user1 = User {
        id : 65,
        username : String::from("Ykong"),
    };

    let mut user2 = User{
        id: 0,
        ..user1
    };

    struct Color (u8,u8,u8);
    let black = Color(0,0,0);

    user1.id = 1;
    user1.username = String::from("Ykong");

    user2.id = 2;


    println!("{} , {}", user1.id, user1.username);
    println!("{}",user2.id);

    println!("{}",black.0);

}


fn find_string(string: &str) -> &str {
    let bytes = string.trim().as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &string[..i];
        }
    }
    return &string[..];
}
