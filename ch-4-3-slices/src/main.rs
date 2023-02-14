use std::io;

fn main() {
    println!("Enter some text with spaces:");

    let mut s = String::new();
    
    io::stdin()
        .read_line(&mut s)
        .expect("Failed to read");

    let furst_word = first_word(&s); //*

    //s.clear(); // error, mutable borrow after immutable*

    println!("first word is: {}", furst_word);

    // string slices, partial or whole
    let word = first_word(&s[0..10]); 
    let word = first_word(&s[..]);
    // string references == whole slices of strings
    // deref coercions, ch 15
    let word = first_word(&s);

    let s_lit = "hello i am a string literal";

    // slice of slice is a slice?
    let word = first_word(&s_lit[0..10]);
    let word = first_word(&s_lit[..]);
    // string literals are string slices already
    let word = first_word(s_lit);
}


fn first_word(s: &str) -> &str { // string slice
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}