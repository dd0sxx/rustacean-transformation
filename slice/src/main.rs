fn main() {
    let s = String::from("hello world");
    let string_literal = "Hello, world!"; //The type of s here is &str: it’s a slice pointing to that specific point of the binary. This is also why string literals are immutable; &str is an immutable reference.

    let hello = &s[0..5]; // or &s[..5] is the same thing
    let world = &s[6..11]; // or &s[6..] is the same thing


        // `first_word` works on slices of `String`s, whether partial or whole
        let word = first_word(&s[0..6]);
        let word = first_word(&s[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
        let word = first_word(string_literal);

    { // There are more than just string slices, array slices work too!
        let a = [1, 2, 3, 4, 5];

        let slice = &a[1..3];

        assert_eq!(slice, &[2, 3]);
    }
}

// using the &String type is unnessesary since we can use &str
// fn first_word(s: &String) -> &str { 
fn first_word(s: &str) -> &str {

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..] // the whole string
}