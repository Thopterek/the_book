use std::env;

fn main() {
    /*
     * usage of the std::env::args -> returns an iterator to passed args
     * function panics if there is invalid unicode, for it args_os and OsString
     * Iterator, produces the series of values
     * collect method turns them into collection
    */
    let ag: Vec<String> = env::args().collect();
    // C style arguments 0 is binary itself
    // dbg!(&ag);
    let query = &ag[1];
    let file = &ag[2];
    println!("{query} looking for {file}");
}
