# Lifetime ensures that references are valid as long as needed

* Every reference has a lifetime (scope in which is valid)
* Tries to fix a problem of dangling references
* Borrow checker has to know when to use (can't return a & and pass in fn 2x &)

# Lifetime Annotation Syntax

* &i32 is a reference
* &'[lifetime_param] i32 is a reference with explicit lifetime
* &'lifetime_param mut i32 mutable one with explicit lifetime
* Allow for borrow checker to know what will be returned
* passed in functions like -> fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {}

# Relationships

* through explicit marking of return param with lifetime + args
* compilers knows what we need to return

# Struct definition

* to allow for holding references we need lifetime params
* each item needs to have its own lifetime param

# lifetime elision rules

* times when before there had to be lifetime_param
* specific cases when compiler can add them for yourself eg. passing &str and returns &str

* RULES:
* 1: compiler assigns one lifetime_param to each reference (different ones)
* 2: exactly one reference passed and returned (giving same one)
* 3: if there is self as input its also becoming as part of return (same lifetime_param)

# static

* &'static -> is for the whole program
* probably its more needed to fix code logic rather than use it
