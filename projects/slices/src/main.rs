fn main() {
    let mut s = String::from("blowme you fucking hello world");
    let word = first_word(&s);
    println!("{}",word);
    s.clear();
}
fn first_word(s: &String) -> &str {
	let bytes = s.as_bytes();
	for (i,&item) in bytes.iter().enumerate() {
		if item == b' ' {
		   return &s[0..i];
                }
	}
        &s[..]
}
