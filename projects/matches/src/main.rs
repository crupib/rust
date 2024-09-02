enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn plus_one(x : Option<i32>) -> Option<i32>{
    match x {
        None => None,
        Some(i) => Some(i+1)
    }
}
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
fn main() {
    println!("Hello, world!");
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}",six);
    println!("{:?}",none);
    let some_u8_value = 5u8;

    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    let coins = value_in_cents(Coin::Penny);
    println!("{}",coins);


}
