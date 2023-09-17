fn main() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
    let mut s3 = String::from("fuck off and die");
    let len = calculate_length_ref(&s3);
    println!("The length of '{}' is {}.",s3,len);
    update_string(&mut s3);
    println!("Updated string {}",s3);
}
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_ref(s: &String) -> usize {
    s.len()
 }// len() return


fn update_string(some_string: &mut String) {
    some_string.push_str(", asshole");
}