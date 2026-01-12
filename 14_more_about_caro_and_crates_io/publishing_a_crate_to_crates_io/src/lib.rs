/// using the documentation comment
/// Adds one to the number given
///
/// # Examples
/// ```
/// let arg = 5;
/// let answer = publishing_a_crate_to_crates_io::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
/*
 * We can generate documentation comment by running cargo doc
 * it runs the rustdoc tool and puts generated HTML in target/doc
 * cargo doc --open will build thr HTML for current crate's docs and open in browser
*/
pub fn add_one(x: i32) -> i32 {
    x + 1
}
