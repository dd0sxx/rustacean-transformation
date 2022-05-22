fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5]; // or &s[..5] is the same thing
    let world = &s[6..11]; // or &s[6..] is the same thing

}


fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..] // the whole string
}