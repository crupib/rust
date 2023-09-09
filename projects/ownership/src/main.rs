fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}",s);
    let s1 = String::from("hello eat my shorts");
    let s2 = s1;
    println!("{}",s2);
    let s3 = String::from("hello eat my shorts");
    let s4 = s3.clone();
    println!("s3 {} s4 {}",s3,s4);
    let s = String::from("hello");
    println!("{}", s);
    takes_ownership(s);
    let  x = 5;
    makes_copy(x);
    let x1 = gives_ownership();
    println!("{}",x1);
    let x2 = String::from("hello");
    let _x3 = takes_and_gives_back(x2);
    println!("{}", _x3);
}
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}
fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
fn gives_ownership() -> String {
  let some_string = String::from("hello");
    some_string
}
fn takes_and_gives_back(a_string: String) -> String{
    a_string
}