fn main() {
    let number = 5;
    let number2 = 3;
    let number3 = 6;
    if number < 5 {
      println!("condition was true");
    } else {
      println!("condition was false");
    }
    if number2 != 0 {
        println!("number was not zero");
    } 
    if number3 % 4 == 0 {
        println!("number3 is divisble by 4");
    } else if number3 % 3 == 0 {
        println!("number3 is divisble by 3");
    } else if number3 % 2 == 0 {
        println!("number3 is divisble by 2");
    } else {
        println!("number3 is not divisble by 4,3,2");
    }
    let condition = true;
    let number4 = if condition {
        5;
        9
    } else {
             6
    };
    println!("The value of number is: {}", number4);
}
