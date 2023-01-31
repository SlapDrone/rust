use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {

    println!("Raad het getal eens!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Voer uw gok in:");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("Jij hebt {guess} geraden.");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("te klein!"),
            Ordering::Greater => println!("te groot!"),
            Ordering::Equal => {
                println!("Jij wint!");
                break;
            }
        }
    }
    
}
