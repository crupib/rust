
#[derive(Debug)]

enum IpAddr {
    V4(u8,u8,u8,u8),
    V6(String),
}
/*
enum Message {
    Quit,
    Move {x: i32, y:i32 },
    Write(String),
    ChangeColor (i32, i32, i32),
}
 */

/*
enum Option<T> {
    /  None,
}
*/
fn main() {
    let home = IpAddr::V4(127,0,0,1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("Home {:?}",home);
    println!("Loopback {:?}",loopback);
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number : Option<i32> = None;
    println!("{:?}",some_number);
    println!("{:?}",some_string);
    println!("{:?}",absent_number);
}
