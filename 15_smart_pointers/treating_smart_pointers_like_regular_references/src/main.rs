use std::ops::Deref;

fn main() {
    let x: i8 = 5;
    let y: MyBox<i8> = MyBox(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    let m = MyBox(String::from("Rust"));
    /*
     * Deref Coercion
     * converting reference that implements the Dereft trait,
     * it converts into a reference of another type:
     * eg. &String to &str
    */
    println!("Usage of Deref Coersion!");
    hello(&m);
    println!("Same as writing below:");
    hello(&(*m)[..]);
}

// wrapper type similiar to Box<T>, no heap
struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    /*
     * defines an associated type for the Deref to use
     * different than just a generic parameter
    */
    type Target = T;
    fn deref(&self) -> &Self::Target {
        /*
         * allows to return reference with *
         * similiar to Tuple .0 returning first element
         * behind the scenes run as *(y.deref())
        */
        &self.0
    }
    // deref possible below:
    // fn new(x: T) -> MyBox<T> {
    //    MyBox(x)
    // }
}

/*
 * to showcase Deref Coercion, done in those cases:
 * 1: From &T to &U when T: Deref<Target=U>
 * 2: From &mut T to &mut U when T: DerefMut<Target=U>
 * 3: From &mut T to &U when T: Deref<Target=U>
*/
fn hello(name: &str) {
    println!("Hello, {name}");
}
