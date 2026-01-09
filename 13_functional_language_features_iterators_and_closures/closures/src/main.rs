use std::thread;
use std::time;

/*
 * closures are anonymous functions
 * saving vars or passing the args to other fn
 * capturing values from scope which they are defined in
*/

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
 }

struct Inventory {
    shirts: Vec<ShirtColor>,
 }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
 }

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        /*
         * if Some variant is there it returns its value
         * if none it calls the closure and gets its value to return
         * || marks the closure expression passed as argument to unwrap_or_else
        */
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;
        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
 }

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };
    let first_user_pref = Some(ShirtColor::Red);
    let giveaway_to_first = store.giveaway(first_user_pref);
    println!("The user with preference {:?} gets {:?}", first_user_pref, giveaway_to_first);
    let num: u32 = 25;
    println!("Checking up the number {num}");
    // usage of closure with annotation
    let expensive_closure = |x: u32| -> u32 {
        println!("calculating the closure...");
        thread::sleep(time::Duration::from_secs(2));
        x
    };
    println!("no worries no calculation needed its still {}", expensive_closure(num));
    /* 
     * same closure different way to write
     * it takes the type of the first thing passed to it
    */
    let cheap_closure = |x| {
        println!("using another closure");
        thread::sleep(time::Duration::from_secs(1));
        x
    };
    println!("same thing less writing -> {}", cheap_closure(num));
    // or even less
    let x_and_print = |x| {println!("amazing one liner closure"); x};
    println!("EVEN less writing -> {}", x_and_print(num));
    let just_x = |x| x;
    println!("SUPER SMALL -> {}", just_x(num));
    let second_user_pref = None;
    let giveaway_to_second = store.giveaway(second_user_pref);
    println!("The user with preference {:?} gets {:?}", second_user_pref, giveaway_to_second);
    /*
     * Closure decides what to depending on the body of the function
     * same three ways in which fn can take param
     * - borrowing immutably
     * - borrowing mutably
     * - taking ownership
    */
    let list = vec![1, 2, 3];
    println!("Before closure: {list:?}");
    let only_borrows = || println!("printing from closure: {list:?}");
    println!("Before calling the closure: {list:?}");
    only_borrows();
    println!("Plus another call after the call {list:?}");
    // version with capturing mutable reference
    let mut another_list = vec![10, 9, 8];
    println!("Before defining closure: {another_list:?}");
    let mut borrow_mutably = || another_list.push(7);
    // immutable borrow print is not allowed anymore it has to use move
    // println!("After definition of closure {another_list:?}");
    borrow_mutably();
    println!("After calling closure with mut borrow: {another_list:?}");
    // spawning thread and using the move for it in closure
    let some_list = vec![32, 16, 8];
    println!("Before defining closure: {some_list:?}");
    thread::spawn(move || println!("Printing from thread: {some_list:?}"))
        .join()
        .unwrap();
    /*
     * a close body can do any of the following
     * - move a captured value out of closure
     * - mutate the captured value 
     * - neither move nor mutate the value 
     * - capture nothing from environment
     * The way closure captures handles values changes its trait
     * they can be one, two or all three from the following Fn traits
     * - FnOnce applies to closure that can be called once,
     * if it moves the values its the only trait that it implements
     * - FnMut applies to the one that mutate the value,
     * can be called more than once
     * - Fn applies to closures that don't mutate or move, capture nothing,
     * can be called mutliple times concurrently
    */
    let mut to_check = [
        Rectangle {width:10, height:1},
        Rectangle {width:3, height:5},
        Rectangle {width:7, height:12},
    ];
    let mut num_sort_operations = 0;
    to_check.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{to_check:#?}, sorted in {num_sort_operations} operations");
}

