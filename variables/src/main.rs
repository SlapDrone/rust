fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = x + 1;
    println!("The value of x is: {x}");
    // shadow x in inner scope
    {
        // with shadowing (let again) variable still immutable
        // but transformed explicitly in particular circumstances
        let x = x * 2;
        println!("The value of x is: {x}")
    }
    // "un"-shadowing
    println!("The value of x is: {x}");

    let spaces = "       ";
    // shadowing spaces can also change type and 
    // allow reusing same variable name
    let spaces = spaces.len();
    println!("{spaces} spaces");
}
