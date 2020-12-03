# 5: Using Structs to Structure Related Data

## 1. Defining and Instantiating Structs
You cannot define individual fields in a struct to be mutable, only the whole object. When constructing an instance of a struct, instead of writing the field name and variable name, if they are the same, just write the field name then a comma. You can use

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

## 2. An Example Program Using Structs
Using structs and tuples to specify related parameters to function. Preferably structs since the naming of variables makes it more obvious what is what.

We have to opt in to debugging information for structs, Rust won't create one for us. We can use #[derive(Debug)] to get default representation, then print with {:?} or {:#?} (nicer formatting).

## 3. Method Syntax
Methods are functions directly related to structs, they are defined in an "impl" block of a struct. Associated functions are functions related to structs but without a reference to self, often used as constructors to return a new instance of a struct.
