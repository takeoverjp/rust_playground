fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &c) in bytes.iter().enumerate() {
        if c == b' ' {
            return &s[..i];
        }
    }
    return s;
}

fn main() {
    let s = String::from("Hello, world!");
    let word = first_word(&s);
    println!("word = {}", word);
}
