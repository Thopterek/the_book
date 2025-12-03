/*
 * More in depth about type of language
 * as Rust is statically typed (compile)
*/
fn main() {
    println!("Data Types!");
    /*
     * without type annotation we get error
    */
    let guess: u32 = "42".parse().expect("Not a number");
    println!("guess - > {guess}");
    /*
     * using scalar types which are:
     * ints, floats, bools, chars
    */
    let small: i8 = 2;
    let floo: f32 = 3.3;
    let bully: bool = true;
    let charizard: char = 'a';
    println!("like int -> {small}, float -> {floo}, bool -> {bully}, char -> {charizard}");
    /*
     * Different types of ints are based on the possible size
     * setting from unsigned to standard by i or u prefix
     * 8, 16, 32, 64, 128 bit and till isize / usize
     * default goes to the i32
    */
    let eight: i8 = 127;
    let sixtee: i16 = 32767;
    println!("8 bit max is {eight} and then follows 16 bit -> {sixtee} till architecture max");
    let un_eight: u8 = 255;
    let un_sixtee: u16 = 65535;
    println!("for unsigned goes from 8 bit -> {un_eight}, then 16 bit -> {un_sixtee} and so on");
    /*
     * You can write numbers with _ as per 8_0_8
     * to have a visual seperation and takes in formts:
     * Decimal, Hex, Octal, Binary, Byte
    */
    let dec: i32 = 8_0_8;
    let hex: i32 = 0xff;
    println!("Dec is {dec}, Hex is {hex}, printed as decimals");
    /*
     * There ways to check for the overflows
     * as per handling of it rather than expecing wrap
     * checked fails with the explicit type
     * overflowing returns value and boolean
    */
    let new: i8 = eight.wrapping_add(1);
    println!("Overflow but wrapped_add -> {new}");
    {
        let test = eight.checked_add(1);
        println!("Now with checked_add is {:?}", test);
    }
    let (value, state) = eight.overflowing_add(1);
    println!("Value -> {} & State -> {}", value, state);
    {
        let test = eight.saturating_add(1);
        println!("Saturating add -> {test}");
    }
    /*
     * Floating point numbers are always signed
     * base one is f64 but there is also f32
     * same C++ standard IEEE754
    */
    let x = 2.0;
    let y: f32 = 3.0;
    println!("Different floats addition {}", x + y);
}
