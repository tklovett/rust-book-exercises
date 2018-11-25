fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5
    let word = first_word(word); // word will get the value 5

    s.clear(); // This empties the String, making it equal to ""

    println!("First word is: {}", word);
    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}

fn first_word(s: &str) -> &str {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}