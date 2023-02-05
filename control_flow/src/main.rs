use std::io;

fn main() {

    let mut number = String::new();
    
    println!("Enter a number:");

    io::stdin()
        .read_line(&mut number)
        .expect("failed to read number");

    let number : u8 = number
        .trim()
        .parse()
        .expect("failed to parse input as num");

    if number < 5 {
        println!("condition true");
    } else {
        println!("condition false");
    }

    if number % 4 == 0 {
        println!("divisible by 4");
    } else if number % 3 == 0 {
        println!("divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("not divisible by 4, 3 or 2.");
    }

    // if is an expression => can be used with let assignment
    let condition = true;
    let nombre : u8 = if condition {5} else {6};
    println!("el nombre: {nombre}");
    

    // break with a value returns from loop
    let mut cnt : u8 = 0;
    let result : u8 = loop {
        cnt += 1;
        if cnt == 10 {
            break cnt*2;
        }
    };
    println!("result {result}");
    // run demo functions
    labelled_loop();
    blast_off();
    blast_off_for();
}

fn blast_off() {
    let mut number = 3;

    // demo while loop
    while number != 0 {
        println!("{number}...");
        number -= 1;
    }
    println!("BLASTOFF!");
}

fn blast_off_for() {
    for number in (1..4).rev() {
        println!("{number}.....");
    }
    println!("BLASTOFF!!!");
}

fn labelled_loop() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            // plain break: inner loop
            if remaining == 9 {
                break;
            }
            // break outer loop using label
            if count == 2 {
                break 'counting_up
            }
            remaining -= 1;

        }
        count += 1;
    }
    println!("End count = {count}");
}
