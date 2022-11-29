fn main() {
    let s = String::from("hello world");
    // let mut s = String::from("hello world");
    
    let first = first_word(&s);

    // s.clear()
    println!("{first}");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }

    return &s[..];
}