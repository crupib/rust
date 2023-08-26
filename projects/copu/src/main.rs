fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    let x = 5;
    println!("{}",x);
    makes_copy(x);
    println!("Hello, world!");
    println!("Hello, world!");
    println!("{}",x);
    
}
fn takes_ownership(some_string: String){
	println!("{}", some_string)
}
fn makes_copy(some_integer: i32) {
   println!("{}", some_integer)
}
