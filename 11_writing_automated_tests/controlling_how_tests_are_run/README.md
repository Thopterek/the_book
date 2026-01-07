# Next sub chapter

building upon what was done previously, lib part was copied

# cargo test options

* --help -> displays the options you can use with cargo test
* -- --help -> displays the options you can use after the separatior
* documented in the Test section of rustc

# paraller and other default behaviour

* by default they run paraller to controll it more -> -- --test-threads=<number_of_threads>
* it also hides all of the printing inside the tests unless -> --show-output
* running a subset of tests by name -> cargo test <name_of_fn_or_partially_matched_per_regex>
* or adding ignore after the test in the same style to ignore, to run them -> -- --ignored
* running ignored and not ignored would be -> -- --include-ignored
