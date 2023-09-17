fn main()
{
    let my_string = String::from("hello world");
    let word = first_word(&my_string[..]);
    let my_string_literal = "hello world";
    let word2 = first_word(&my_string_literal[..]);
    let word3 = first_word(my_string_literal);
    println!("All three {}, {}, {}", word, word2, word3);
}
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

