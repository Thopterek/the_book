/*
 * Library starts with function we can test
 * - tests set up any data or state
 * - run the particular code
 * - assert that the results are okay
*/
pub fn add(left: u64, right: u64) -> u64 {
    println!("We are printing something");
    left + right
}

/*
 * making it into test by #[test] attribute
 * used later through running cargo test
 * test result: ok -> everything passed
 * [count] passed; [count] failed;
*/
#[cfg(test)]
mod tests {
    // usage of a global
    use super::*;

    #[test]
    #[ignore]
    fn exploration() {
        let result = add(2, 2);
        // macro to assert the expected answer equals
        assert_eq!(result, 4);
    }
    /* each test is own thread
     * they fail if something inside panics
    */
    #[test]
    fn will_panic() {
        panic!("I make the test to fail");
    }
    #[test]
    fn larger_can_hold_smapller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 4,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
        // this doesn't add new test just grows one
        assert!(!smaller.can_hold(&larger));
    }
    #[test]
    fn smaller_cannot_fit() {
        let big = Rectangle {
            width: 10,
            height: 9,
        };
        let smol = Rectangle {
            width: 1,
            height: 1,
        };
        // negating as checking for true
        assert!(!smol.can_hold(&big));
    }
    #[test]
    fn it_adds_1() {
        let res = add_one(4);
        // or assert_ne! -> !=
        assert_eq!(res, 5);
    }
    /*
     * to make it more explicit you can add expected
     * cool thing checks for message of panic! if includes
    */
    #[test]
    #[should_panic(expected = "panic no matter what")]
    fn if_panics_passes_test() {
        panic!("DAAAMN WE PASSED IT, panic no matter what");
    }
    /*
     * different add with test on templates
     * check with asser through assert!(value.is_err())
    */
    #[test]
    fn template_test() -> Result<(), String> {
        let result = add(2, 2);
        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal 4"))
        }
    }
}

/*
 * added to write tests using assert! macro
 * by base checking for true as passed to it
*/
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn add_one(number: u64) -> u64 {
    number + 1
}

