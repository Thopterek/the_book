/*
 * Actual subject function
 * basic way to write
*/
fn example() {
    let width = 30;
    let height = 50;
    println!(
        "The area of the rectangle is {}",
        area(width, height)
        );
}

fn tuple_version() {
    let rectangle = (30, 50);
    println!(
        "The tuple_version area {}",
        tup_area(rectangle)
        );
}

fn tup_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn main() {
    println!("Calc the rectangle!");
    println!("-----");
    // not used not needed _
    for _ in 0..5 {
        println!("|   |");
    }
    println!("-----");
    example();
    tuple_version();
    struct_version();
    println!();
    println!("--------------DBGMACRO------------");
    println!();
    let scale = 2;
    let rec = DebugRectangle {
        // checking particular value
        width: dbg!(15 * scale),
        height: 50,
    };
    println!();
    println!("debug rectangle is {rec:?}");
    println!();
    println!("other print options is {rec:#?}");
    println!();
    // now using some of that dbg! magic
    // shows the exact line and call
    dbg!(&rec);
}

fn struct_version() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("Last but not least struct version {}",
        struct_area(&rect)
    );
}

// so we don't take ownership of struct
fn struct_area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug)]
struct DebugRectangle {
    width: u32,
    height: u32,
}

