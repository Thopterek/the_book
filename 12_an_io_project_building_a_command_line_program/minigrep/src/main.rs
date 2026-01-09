use std::env;
use std::fs;
use std::process;
use std::error::Error;
// using our lib (bringing into scope)
use minigrep::{search, insensitive_search};

fn main() {
    let ag: Vec<String> = env::args().collect();
    /*
     * Usage of unwrap_or_else allows for custom error handling
     * if its not an error value it will work like unwrap
     * else its calling the code in the closure (anonymous fn)
    */
    let config = Config::build(&ag).unwrap_or_else(|err| {
        // printing on std err out
        eprintln!("Parsing error while setting up config: {err}");
        // C style exit with passing the error code
        process::exit(1);
    });
    // no unwrap because we don't return value on success
    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

/*
 * Handling the error case while keeping the unit type return ()
 * Box<dyn Error> when function will return that implements the Error trait
 * dyn -> is short for dynamic, as per possible Error types
 * handled together with usage of ? to return error value of fn
 * Ok(()) -> way to call it indicating that we care only about side effects
*/
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;
    // check for env variable
    let result = if config.ignore_case {
        insensitive_search(&config.query, &content)
    } else {
        search(&config.query, &content)
    };
    for line in result {
        println!("{line}");
    }
    Ok(())
}

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Error: cargo run [query] [file_path]");
        }
        // clone just for simplicity of not managing lifetime
        let query = args[1].clone();
        let file_path = args[2].clone();
        // using the environmental variable, if not set using normal search
        // idk that you could set envs like this -> IGNORE_CASE=1 cargo run to poem.txt
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config { query, file_path, ignore_case })
    }
}
