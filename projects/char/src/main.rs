fn main() {
    println!("Hello, Chars! and other stuff!");
    let c = 'z';
    let z = 'Z';
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
    println!("y = {}", y);
    println!("c = {}, z = {}, heart_eyed_cat = {}", c, z, heart_eyed_cat);
    println!("a = {}, b = {}, c = {}", a, b, d);
    println!("{}, {}, {}", tup.0,tup.1,tup.2);
    println!("{}", _five_hundred);
    println!("{} {} {}", arr[0], arr[1], arr[2]);
    println!("{}", months[0]);
    println!("{}", fifth);
    myprintfun(c,a);
}
fn myprintfun(c: char, a: i32)
{
    println!("Hello from myprintfun");
    println!("c = {} a = {}", c, a);
}
