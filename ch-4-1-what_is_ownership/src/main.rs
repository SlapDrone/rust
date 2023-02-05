fn main() {
    let mut s = String::from("hello");

    s.push_str(", babos!");

    println!("{}", s);

    let s2 = s;
    // assigning new pointer to s 
    // invalidates original reference 
    // ("shallow copy" => "move").
    // println!("{}", s); // error[E0382]: borrow of moved value: `s`
    println!("{s2}");

    // clone method is deep copy
    let s3 = s2.clone();
    println!("{s3}");

    test_ownership();
    test_ownership_again();
}

fn takes_ownership(s: String) { // s comes into scope
    println!("{}", s);
} // drop called when `s` goes out of scope, mem freed

fn makes_copy(i: i32) { // i comes into scope
    println!("{}", i); // i is copied since it's an integer
}

fn test_ownership() {
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{s}"); // error[E0382]: borrow of moved value: `s`
    let i = 5;
    makes_copy(i);
}


fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}


fn takes_and_returns_ownership(s: String) -> String {
    s
}

fn test_ownership_again() {
    let s1 = gives_ownership();
    // s1 out of scope now
    let s2 = String::from("hello");
    let s3 = takes_and_returns_ownership(s2);
    println!("s3: {s3}");
}