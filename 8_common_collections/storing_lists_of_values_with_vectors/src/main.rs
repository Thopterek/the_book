fn main() {
    println!("Hello, vectors!");
    /*
     * basic C++ as per memory wise
     * creating a new one Vec<T>
     * version needed a type for None
     * and if there is Some() to be
    */
    let mut vector: Vec<i32> = Vec::new();
    let v_macro = vec![1, 2, 3];
     {
         // vector modify
         vector.push(0);
         /*
          * then have something to read
          * - indexing method (C style)
          * - get method (OOP style?)
          * C style don't push and use
         */
         // ref -> & [of some value] at index -> [];
         let third: &i32 = &v_macro[2];
         println!("The third element is {third}");
         let first: Option<&i32> = vector.get(0);
         match first {
             Some(first) => println!("The first element is {first}"),
             None => println!("There is no element 0"),
         }
     }
     {
         // getting immutable ref
         for i in &v_macro {
             print!("{i}");
         }
         println!(" <- done printing immutable");
         // getting modified one
         // dereferencing to apply changes
         for i in &mut vector {
             *i += 50;
         }
     }
     {
         println!("Working with vector of diff types");
         let mut row = vec![
             Spread::Int(3),
             Spread::Float(1.2),
             Spread::Text(String::from("Some String")),
         ];
         for i in &row {
             println!("{:?}", i);
         }
         row.pop();
     }
}

/*
 * handling of saving diff types as enums
 * would work with structs like in C also?
*/
#[derive(Debug)]
enum Spread {
    Int(i32),
    Float(f64),
    Text(String),
}
