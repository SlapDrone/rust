fn main() {
    println!("Hello, world!");
    another_function(44, 'K');

    let y   = {
        let x: u8 = 15;
        x + 3
    };
    println!("y: {y}");
    let z = five();
    println!("z: {z}");
    println!("{}", plus_one(z));
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn another_function(x: u32, unit_label: char){
    println!("measurement {x}{unit_label}");
}