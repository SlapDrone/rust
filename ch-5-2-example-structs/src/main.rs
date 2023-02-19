fn main_vars() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square px",
        area_vars(width1, height1)
    );
}

fn main_tuples() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square px",
        area_tuples(rect1)
    );    
}

fn main() {
    let scale = 2;
    let rectangle = Rectangle {
        width: dbg!(scale*30),
        height: 50
    };

    println!(
        "The area of the rectangle is {} square px",
        area_struct(&rectangle)
    );

    println!("{:?}", rectangle);
    dbg!(&rectangle);
}

fn area_vars(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuples(dims: (u32, u32)) -> u32 {
    dims.0 * dims.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn area_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}