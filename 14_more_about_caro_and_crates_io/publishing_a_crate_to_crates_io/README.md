# Commonly Used Sections

* Just like before was used Examples and now others
* Panics, the situation when function can panic
* Errors, if the functions returns Result, describing what kind of errors
* Safety if the function is unsafe to caller there needs to be section explaining what caller needs to keep track of

# Documentation Comments as tests & Containted Item Comments

* running cargo test will run the code examples in documentation as tests
* adding //! adds documentation to the item that contains the comments -> purpose of crate

# Exporting a Convenient Public API

* using the pub use keyword, to re-export items to make a public structure
* the private structure might be different than the private one
* pub use self::<name_of_module>::<name_of_fn>;

# Setting up a Crates.io Account & Publishing

* connecting through GitHub and running cargo login
* after that passing the API key to the prompt
* package name is first-come, first-served, depending on Cargo.Toml name section
* also you need to setup the license (MIT OR Apache-2.0)
* if using the license thats not part of SPDX you have to use license-file and license key
* cargo publish is permanent, the version can neber be overwritten
* adding a new version of the existing crate through Cargo.toml and setting new version
* crate can be yanked so no new projects can use that crate -> cargo yank --vers <specific_version>
* you can also undo that yank through cargo yank --vers <specific_version> --undo

