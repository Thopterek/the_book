fn main() {
    println!("Hello, Strings!");
    let mut mutable_string = String::new();
    let data = "initial literal str".to_string();
    let s = String::from("initial literal str");
    if data == s {
        println!("Same things");
    }
    /*
     * Updating the String
     * - push_str(slice)
     * - push(char)
    */
    mutable_string.push_str("こんにちは ");
    mutable_string.push('U');
    println!("{}", mutable_string);
    {
        // concatenating with +
        // usage of fn add(self, s: &str) -> String
        let first = String::from("Hello, ");
        let second = first + &String::from("world!");
        println!("{second}");
        // concatenating with format!
        // returns string with its content
        let tic = String::from("tic");
        let tac = String::from("tac");
        let toe = String::from("toe");
        let ttt = format!("{tic}-{tac}-{toe}");
        println!("{ttt}");
        // no matter what indexing is not possible
        // there is only slicing but it sill might panic
    }
    // iterating over Strings
    for c in mutable_string.chars() {
        print!("char -> {c}");
    }
    println!("\n or use the bytes values");
    for b in mutable_string.bytes() {
        print!("byte -> {b} ");
    }
    println!("\n like above");
    // grapheme clusters are from non std crates -> crates.io
}
