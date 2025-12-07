/*
 * ownership as a set of rules that compiler checks
 * - each value in Rust has an owner
 * - There can only be one owner at a time
 * - When the owner goes out of scope, the value will drop
 * stack -> push / pop (asm style) -> known size
 * heap -> c style allocating and getting a pointer
*/
fn main() {
    // before declaration
    {
        // stack string
        let _s: &str = "Hello";
        // in the scope
    }
    // out of scope
    {
        // using :: to get particular method in [name]::[method]
        let mut heap_string: String = String::from("Hello");
        heap_string.push_str(", world!");
        print!("{}", heap_string);
        // calling drop (C style free at the end of scope)
    }
    // out of scope
    {
        /*
         * Pointing to the same index
         * Rust is invalidating the orignal
         * but not changing the heap (c++ move)
        */
        let og_heap = String::from("Test");
        // you can add .clone() to create deep copy
        let invalidating = og_heap;
        println!("\n{invalidating}");
        // drop called ones on -> invalidating
    }
    // calling drop automatically, while re assinging
    let mut s = String::from("This will be dropped");
    s = String::from("this one replaced it");
    println!("{s}");
    /*
     * as for integers we know their compile time size
     * and there is no performance drop between deep / shallow
     * They have a Copy trait as well as other stack allocated
     * so its always making a full copy as per:
    */
    let x = 5;
    let y = x;
    println!("Look at that x -> {x} & y -> {y}");
    {
        let new = String::from("Comes into scope");
        takes_ownership(new);
        // new -> is no longer valid
        let x = 5;
        uses_copy_trai(x);
        // x -> is still valid
    }
    /*
     * Returning values can transfer ownership
    */
    {
        let get_ownership = gives_ownership();
        println!("Got this -> {get_ownership}");
        let takes_over = takes_and_gives_back(get_ownership);
        println!("Take over took -> {takes_over}");
        // takes over drops
    }
}

fn takes_and_gives_back(a_string: String) -> String {
    // comes into scope
    a_string
    // gets moved into the calling function
}

fn gives_ownership() -> String {
    let new_string = String::from("Given");
    new_string
}

fn takes_ownership(string_to_drop: String) {
    println!("{string_to_drop}");
    // gets dropped here
}

fn uses_copy_trai(copied_int: i32) {
    println!("{copied_int}");
    // copy ends here
}

