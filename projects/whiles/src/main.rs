fn main() {
   let mut number = 10;
   while number != 0 {
      println!("{}!",number);
      number = number - 1;
   }
   println!("LIFTOFF!!!");
   let mut index = 0;
   let a = [10, 20, 30, 40, 50];
   while index < 5 {
     println!("the value is: {}",a[index]);
     index = index + 1;
   }
}
