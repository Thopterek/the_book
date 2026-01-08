# Change to the way the directory is structured

* as this chapter is project everything is going to two cargos
    * first one being the basic with all the debugging
    * second one after refactoring the code properly
* the new concepts and subchapters are gonna be mentioned in this README
* still most of the information will be a source code and its comments

# Accepting Command Line Arguments

* there are libraries to do so but this time it will be fully recreated
* our program should have the following options -> cargo run -- searchstring example-filename.txt

# Reading a File

* usage of std::fs for file reading

# Refactoring to Improve Modularity and Error Handling

* Rust style rules for main
    * Calling the command line parsing logic with args
    * Setting up any other configs
    * Calling a run function in lib.rs
    * Handling the error if run returns it

# Adding Functionality with Test-Driven Development

* Steps for the TDD
    * Write a test that fails and run it to make sure it fails for the reason you expect
    * Write or modify just enough code to make the new test pass
    * Refactor the code added or changed and make sure the tests still pass
    * Repeat from the first step
