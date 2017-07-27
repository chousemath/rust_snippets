fn main() {
    println!("Hello, world!");
    let x = 5;
    println!("The value of {}", x);
    // variable shadowing for re-binding
    let x = "Hello Again";
    println!("The value of x: {}", x);

    // variables can be explicitly set as mutable with the `mut` keyword
    let mut y = 0;
    println!("y = {}", y);
    y = y + 1;
    println!("y = {}", y);
    y = y + 1;
    println!("y = {}", y);
}
