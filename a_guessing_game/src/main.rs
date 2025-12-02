/*
 * without use (includes), we load prelude set
*/
use std::io;
/*
 * use is our header / import
 * std -> standard library
 * io <- input / output
*/
use rand::Rng;
/*
 * above line added through Cargo.toml dependency
 * run -> cargo build to get the dependency
 * you can explicitly update through changing the version
 * or command -> cargo update
 * to be reproduced with same one you use Cargo.lock
 */
use std::cmp::Ordering;

fn main() {
    /*
     * particular number generator -> current thread
     * passing the range of generated numbers to method
     * cargo doc --open -> to show methods and functions of dependencies
    */
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Guest the number!\nPlease input your guess.");
    /*
     * to make variable mutable you have to add mut
     * if not -> everything is being constant
     * [new_instance]::[associated_function]
     * in this instance creating empty str
     *
     * added the loop before hand to wrap it around
     * loop is just an infinite while loop (while(true))
    */
    loop {
        let mut guess = String::new();
        /*
         * possible rewrite (without use) std::io::stdin
         * create a new instance of std::io::Stdin
        */
        io::stdin()
            /*
            * appends the line into a string
            * uses references -> same  logic:
            *  - they have to be made mutable
            *  - if not things are constants
            */
            .read_line(&mut guess)
            /*
            * uses C like enum for errors
            * return (0) == Result Ok
            * return (23) == Result Err
            */
            .expect("Error: Failed to read line.");
        /*
         * shadowing the value of last guess -> used when converting values
         * u32 -> unsigned 32 bit number, i32 would be normal one
         * 
         * added invalid input handling through Ok and Err
         * expect line is being switched to the match
         * before -> let guess: u32 = guess.trim().parse().expect("Please type a number!");
        */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        /*
         * match expression allows to check enum what cmp returned
         * its made out of arms and checks the particular pattern
        */
        match guess.cmp(&secret_number) {
            /*
             * Ordering is another enum variant
             * cmp is a method to compare with referenced value
            */
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        println!("You guessed: {guess}");
    }
}

