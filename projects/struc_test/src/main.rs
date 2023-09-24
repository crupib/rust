struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
fn main() {
    println!("Hello, world!");
    let mut user1 = build_user(String::from("bill.crupi@gmail.com"),String::from("crupib"));
    let black = Color(0,0,0);
    let origin = Point(0,0,0);
    println!("{}", user1.email);
    println!("{}", user1.username);
    println!("{}", user1.active);
    println!("{}", user1.sign_in_count);
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("fuck face"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,

    };
    println!("{}", user2.email);
    println!("{}", user2.username);
    println!("{}", user2.active);
    println!("{}", user2.sign_in_count);
    user1.email = String::from("crap@shit.com");
    println!("{}",user1.email);
    println!("black color = {}, {}, {}", black.0,black.1,black.2);
    println!("origin = {}, {}, {}", origin.0, origin.1,origin.2);
}
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
