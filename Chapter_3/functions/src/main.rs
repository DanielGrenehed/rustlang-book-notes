fn main() {
    let x = plus_one(5);
    log_value_of_x(x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn log_value_of_x(x: i32) {
    println!("The value of x is: {}", x);
}
