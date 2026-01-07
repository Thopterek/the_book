# Building upon again

adding the categorizations for the tests

# Unit tests

* smaller and more focused, one module in isolation
* used also for private interfaces
    * bringed to be tested through -> use super::*
    * there is a school were the private shouldn't be part of tests
* annotate the module with cfg(tests) -> only compiled if caro test is run
    * cfg stands for configuration, magic behind the above
    * it includes any helper functions that might be within this module

# Integration tests

* entirely external to your library
* only public and potentially multiple modules per test
* made in the parent directory, annotated tests

adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs

* using it adds after running the test 3 sections
    * basic tests in src
    * tests in the directory of same name
    * Doc-tests
* only run after all of the basic tests pass
* each file in tests -> compiled as each own crate -> adds one more section to cargo test output
* files in subdirectoreis don't have a seperate section in the output

# Important > [!NOTE]
> if project is a binary crate (doesn't containt lib.rs), we can create create integration tests in test directory

