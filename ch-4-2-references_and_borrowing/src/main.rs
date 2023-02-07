fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    //change(&s1);
    change_mut(&mut s1);

    println!("{}", s1);

    let r1 = &mut s1;
    //let r2 = &mut s1; // error[E0499]: cannot borrow `s1` as mutable more than once at a time
    // can only have one mutable, or multiple immutable
    //let r3 = &s1; // error[E0502]: cannot borrow `s1` as immutable because it is also borrowed as mutable
    println!("{}", r1);

    // dangling pointer prevented by compiler
    //let reference_to_nothing = dangle();
}


fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope, but doesn't own the object it 
// refers to, so the underlying string memory isn't deallocated


// doesn't compile, can't change borrowed object!
// error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
// fn change(s: &String) {
//     s.push_str(", world!");
// }


fn change_mut(s: &mut String) {
    s.push_str(", world!");
}

// error[E0106]: missing lifetime specifier
// this function's return type contains a borrowed value,  
// but there is no value for it to be borrowed from
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }