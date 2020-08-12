
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
