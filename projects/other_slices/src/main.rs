fn main() {
    println!("Hello, world!");
    let a = [1,2,3,4,5];
    let slice = &a[0..5];
    println!("slice = {}", slice[0]);
    for item in slice {
        println!("{}",item);
    }
    }