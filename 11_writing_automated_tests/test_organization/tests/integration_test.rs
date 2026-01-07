use how_to_write_tests::add_one;

// same name as directory
mod not_shown;

#[test]
fn integrated_test() {
    // calling the function by name
    not_shown::something_hidden();
    let result = add_one(1);
    assert_eq!(result, 2);
}
