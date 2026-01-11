fn main() {
    let some_vector = vec![1, 2, 3];
    /*
     * iterators are lazy
     * no effects unless you call method to use it
     * it uses it up preventing from double usage
    */
    let iterator = some_vector.iter();
    /*
     * the loop hides making iterator mutable
     * normally to call next it would be mut
    */
    for value in iterator {
        println!("Got: {value}");
    }
    let vector: Vec<i32> = vec![1, 2, 3];
    // iterator adapters -> methods that produce diff iterator
    // need to call consuming adapter per first comment
    let version_two: Vec<_> = vector.iter().map(|x| x + 1).collect();
    assert_eq!(version_two, vec![2, 3, 4]);
}

