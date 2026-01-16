use crate::List::{Cons, Nil};

fn main() {
    /*
     * Boxes, to store data on heap, pointer to it on stack
     * - for type whose size can't be known at compile time
     * - large amount of data with ownership transfer, no copy
     * - owning a value and caring about type that implements specific trait
     * its being a smart pointer for implemetning Deref trait (treat like reference)
     * and the Drop, that cleans up after going out of scope
    */
    let b_heap: Box<i32> = Box::new(5);
    println!("b = {b_heap}");
    let cons_list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

/*
 * Recursive type eg. cons list
 * - having another value of the same type as part of it
 * - solved by not taking knowledge about value in compile time
 * cons list
 * - each item in the list contains current item and next
 * - last item in list contains only a value called Nil
 * - produced by recursively calling the cons function
 * - base case of the recursion is Nil
*/
enum List {
    Cons(i32, Box<List>),
    Nil,
}
