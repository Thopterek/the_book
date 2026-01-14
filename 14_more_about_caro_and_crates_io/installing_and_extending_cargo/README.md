## Installing Binaries with cargo install

* cargo install -> allows to use binary crates locally
    * way to install tools that are shared on crates.io
    * it has to have a binary target, should mention in README
* every binary is stored in the roots bin folder -> $HOME/.cargo/bin
    * needs to be in the $PATH to run the program

## Extending Cargo with Custom Commands

* if a binary is in $PATH cargo-<name>
    * run by using of cargo <name>
    * customs commands are part of ones listed in cargo --list
