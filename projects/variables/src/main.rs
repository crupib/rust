const MAX_POINTS: u32 = 100_000;
fn main() {
   let mut x = 5;
   println!("The value of x is: {}",x);
   x = 6;
   println!("The value of x is: {}",x);
   println!("The value of MAX_POINTS is: {}",MAX_POINTS);
   let y = 5;
   let y = y + 1;
   let y = y * 2;
   println!("The value of y is: {}",y); 
   let z = 2.0;
   let w: f32 = 3.0;
   println!("The value of z is: {}",z);
   println!("The value of w is: {}",w);
   let sum = 5 + 10;
   let difference = 95.5 - 4.3;
   let product = 4 * 30;
   let quotient = 56.7/32.2;
   let remainder = 43 % 5;
   println!("sum = {}",sum);
   println!("difference = {}",difference);
   println!("product = {}",product);
   println!("quotient = {}",quotient);
   println!("remainder = {}",remainder);
}
