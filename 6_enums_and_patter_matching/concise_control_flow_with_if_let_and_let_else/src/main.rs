fn main() {
    println!("--------------");
    println!("Hello, if let!");
    println!("--------------");
    /*
     * Handling one particular pattern
     * if not ignoring the rest
     * if let [pattern(value)] = [to_be_checked] {
     * binding to whats there to value
     * [code]
     * }
    */
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The max is configured to be {max}");
    }
    if let None = config_max {
        println!("It was none");
    } else {
        println!("Its something");
    }
    // match version to compare to
    match config_max {
        Some(max) => println!("The max is configured to be {max}"),
        _ => (),
    }
}
