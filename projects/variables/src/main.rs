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
   let t = true;
   let f: bool = false;
   println!("t = {}", t); 
   println!("f = {}", f); 
   let c = 'z';
   let z = 'Z';
   let heart_eyed_cat = "é";
   println!("{} {} {}",c,z,heart_eyed_cat);
   let tup: (i32, f64, u8)= (500, 6.4, 1);
   let (x, y, z) = tup;
   println!("{} {} {}",x,y,z);
   let xx: (i32, f64, u8) = (500, 6.4,1);
   let five_hundred = xx.0;
   let six_point_four = xx.1;
   let one = xx.2;
   println!("{} {} {}",five_hundred, six_point_four, one);
   let a = [1,2,3,4,5];
   let first = a[0];
   let second = a[1];
   println!("{} {}",first,second);
}
