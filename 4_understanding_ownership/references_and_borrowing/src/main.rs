/*
 * Damn no dangling pointers?
 * prevents from return &[value];
 * if the heaped value would be dropped
*/
fn no_dangle() -> String {
    let s = String::from("Hello");
    // if the s would be changed to &s it would dangle
    s
}

/*
 * Rules:
 * references must be always valid
 * - At any given time you can have one mutable reference
 *  OR
 *- any number of immutable references
*/
fn main() {
    let mut string: String = String::from("Working with reference");
    // doesn't take the ownership of the value
    // act of borrowing the values (THE PHRASE IS HERE)
    let len = calculate_len(&string);
    print!("Can use {string} because \ncalculate len -> {len}, didn't drop the var");
    mutable_ref(&mut string);
    print!("\n{}", string);
    {
        // mix of mutable and immutable only at certain assurence
        let mut test = String::from("Hello");
        // usage of immutable reference
        let another = &test;
        println!("\n{another} -> works just fine");
        let mutable = &mut test;
        println!("And {mutable} -> because they are after each other");
    }
    let s = no_dangle();
    println!("{s}");
}

/*
 * Can be borrowed mutable only once at a time
 * no data races to be worried about <3
*/
fn mutable_ref(string: &mut String) {
    string.push_str(", and mutable one!");
}

/*
 * polar opposite of the reference &
 * is the derefernce * (looking C like)
*/
fn calculate_len(string: &String) -> usize {
    string.len()
}
