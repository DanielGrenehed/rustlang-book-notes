# 2: Programming a Guessing Game
Variables in rust are immutable by default, use "mut" to make them mutable.

"::" is used to indicated that the preceding type has an associated function with the name of what comes after the the prefix.

"&" indicates the variable is a reference, making the data accessible as the data and not a copy of the data when passing through function as parameters.

As variables references are also immutable by default so if you want to alter the data referenced, pass it as "&mut".

The method "expect" is the function called when a "Result" is an Err, it will cause the program to crash and display the passed message.

The standard library for Rust does not include a random number generator.

The version number in dependencies refers to the newest version of the api compatible with the version number written.

Once the api has been downloaded cargo will not automatically get new versions for you automatically, you have to tell it to update

    cargo update

This ignores the Cargo.lock file and get the latest version of the crates and write the updated versions to the Cargo.lock file.

To get the documentation of your installed crates run:

    cargo doc --open

You can specify variable type with "let name: String = ...". You can also shadow the previous values of values, ei. convert a string to an int without having to create two variables.
