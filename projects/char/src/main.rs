fn main() {
    println!("Hello, Bill Chars! and other stuff!");
    println!("update");
    println!("give it up");
    let c = 'z';
    let z = 'Z';
    println!("{}",z);
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let heart_eyed_cat = '😹';
    let (a,b,d) = tup;
    let _five_hundred = tup.0;
    let arr = [1,2,3,4,5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    let fifth = arr[4];
    let y = {let j = 3;
                  j+1};
    println!("{}",y);
    let tup2 = (500,6.4,1);
    let (x,y,z) = tup2;
    println!("the value of xyz is: {} {} {}",x,y,z);
    println!("y = {}", y);
    println!("c = {}, z = {}, heart_eyed_cat = {}", c, z, heart_eyed_cat);
    println!("a = {}, b = {}, c = {}", a, b, d);
    println!("{}, {}, {}", tup.0,tup.1,tup.2);
    println!("{}", _five_hundred);
    println!("{} {} {}", arr[0], arr[1], arr[2]);
    println!("{}", months[0]);
    println!("{}", fifth);
    my_print_fun(c,a);
    let mut my_name: String = "Dave".to_string();
    my_name.push('!');
    println!("{}" ,my_name);
    let first_letter = 'A';
    println!("First Letter: {}", first_letter);
    let space = ' ';
    println!("space: {}", space);
    let other_language = 'Ꮔ';
    println!("other_language: {}", other_language);
    let cat_face = '😺';
    println!("cat: {}", cat_face);
    println!("Size of a char: {}", std::mem::size_of::<char>());
    println!("Size of string containing 'a': {}", "a".len());
    println!("Size of string containing 'ß': {}", "ß".len());
    println!("Size of string containing '国': {}", "国".len());
    println!("Size of string containing '𓅱': {}", "𓅱".len());
    let slice = "Hello!";
    println!("Slice is {} bytes.", slice.len());
    let slice2 = "안녕!";
    println!("Slice2 is {} by test.", slice2.len());
    println!("{:?}", "a".as_bytes());
    println!("{:?}", "ß".as_bytes());
    println!("{:?}", "国".as_bytes());
    println!("{:?}", "𓅱".as_bytes());
    let q = [1,2,3,4,5];
    let first = q[0];
    let second = q[1];
    println!("first = {} second = {}", first,second);
    another_function();

}
fn my_print_fun(c: char, a: i32)
{
    println!("Hello from my print fun");
    println!("c = {} a = {}", c, a);

}
fn another_function() {
    println!("Another function.");
}
