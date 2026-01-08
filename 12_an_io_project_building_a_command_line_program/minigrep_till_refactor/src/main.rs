// env for env
use std::env;
// file handling
use std::fs;

fn main() {
    /*
     * usage of the std::env::args -> returns an iterator to passed args
     * function panics if there is invalid unicode, for it args_os and OsString
     * Iterator, produces the series of values
     * collect method turns them into collection
    */
    let ag: Vec<String> = env::args().collect();
    // C style arguments 0 is binary itself
    dbg!(&ag);
    let query = &ag[1];
    let file = &ag[2];
    println!("{query} looking for {file}");
    /*
     * taking a file path as argument to open it
     * returns the type std::io::Result<String> with content
    */
    let content = fs::read_to_string(file).expect("We should be able to read it");
    println!("content below:\n{content}");
}
