/*
 * Handling situations when you have duplicated code
 * but the functionality of it is just different type
*/
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
// same as above but for chars
fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
// putting it together with generic type (T -> template like)
// adding extra safety measure for only types with right trait
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    println!("Hello, Generic Data Types!");
    let num_list = vec![32, 231, 33, 1];
    let result = largest_i32(&num_list);
    println!("The largest num is {result}");
    let char_list = vec!['y', 'c', 'a'];
    let result = largest_char(&char_list);
    println!("The largest_char is {result}");
    let result_char = largest(&char_list);
    let result_num = largest(&num_list);
    println!("Getting results from generic {result_char}, {result_num}");
    let ints = Point { x: 5, y: 10 };
    let floats = Point { x: 4.9, y: 9.9 };
    println!("Using ints -> x: {}, y: {}", ints.x, ints.y);
    println!("and floats -> x: {}, y: {}", floats.x, floats.y);
    let mixed = MixedPoint { x: 10, y: 'c' };
    println!("Mixed generic struct -> x: {} y: {}", mixed.x, mixed.y);
    println!("Now lets use the methods");
    ints.show_x();
    let x = ints.return_x();
    println!("And returned x is {}", x);
    println!("Get that float only function -> {}", floats.distance());
}

/*
 * Usage of generics also through structs
 * also with mixed types but for that another version
*/
struct Point<T> {
    x: T,
    y: T,
}
// can also apply to methods
impl<T: std::fmt::Display> Point<T> {
    fn return_x(&self) -> &T {
        &self.x
    }
    fn show_x(&self) {
        println!("I show x in method {}", &self.x);
    }
}
// creating implementation that affects only one type
impl Point<f32> {
    fn distance(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct MixedPoint<T, U> {
    x: T,
    y: U,
}
