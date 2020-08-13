/*
fn main() {

    //let s = "hello"; // stack allocated string literal, immutable
    //let s = String::from("hello"); // heap allocated string
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);


    let x = 5;
    let y = x; // copy of x value, stack allocated separat value

    let s1 = String::from("hello"); // stack pointer to heap
    let s2 = s1; // copy of the pointer to the heap. s1 moved into s2

    // using s1 here would result in an error since Rust considers it invalid.

    let s3 = s2.clone(); // deep copy of s2
    println!("s2  = {}, s3 = {}", s2, s3);

} // scope is over variables no longer valid.
*/

/*
fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    // s is no longer valid
    let x = 5;
    makes_copy(x);
    // can still use x
}

fn takes_ownership(some_string: String) { // string comes in to scope , pointer is pushed onto stack
    println!("{}", some_string);
} // string goes out of scope, drop is called

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // x goes out of scope, nothing special happens
*/

/*
fn main() {
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);
} // s3 and s1 are dropped, s2 was moved

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string // move out to caller
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
*/
/*
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
*/
/*
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The lenght of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
*/
/*
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
*/


fn main() {
    let my_string = String::from("hello world");

    let word = furst_word(&my_string[..]);

    let my_string_literal = "Hello world";

    let word = first_word(&my_string_literal[..]);

    let word = first_word(my_string_literal);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
