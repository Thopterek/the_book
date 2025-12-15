fn main() {
    println!("M E T H O D S");
    let rec = Rectangle {width: 30, height: 30,};
    println!("Area of rectangle {}", rec.area());
    let to_cmp: u32 = 1000;
    println!("Is area of rectangle bigger than {}? {}", to_cmp, rec.bigger_than_area(to_cmp));
    let small_rec = Rectangle {width:10, height: 2,};
    let bigger_rec = Rectangle {width:50, height: 35};
    println!("Can you rectangle hold smaller inside? {}", rec.holds_inside(&small_rec));
    println!("Can you rectangle hold bigger inside? {}", rec.holds_inside(&bigger_rec));
    let square = Rectangle::square(10);
    println!("Area of square size 10 -> {}", square.area());
}

/*
 * Methods are part of the structs
 * allowing to give special functionality
 * first param is always self
*/
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

/*
 * Using impl to give a functionality
 * its going to be a part of struct
 * to use [variable].[method]
*/
impl Rectangle {
    // all of them are associated to type impl [type]
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // we can just add more functions here
    fn holds_inside(&self, other: &Rectangle) -> bool {
        self.area() > other.area()
    }
    // using a new style methods through ::
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

/*
 * Way to call method inside the method
 * through usage of self.[method]
 * When calling methods rust adds:
 * & / &mut / * rather than C ->
*/
impl Rectangle {
    fn bigger_than_area(&self, area_to_compare: u32) -> bool {
        if area_to_compare > self.area() {
            return false;
        }
        true
    }
}

