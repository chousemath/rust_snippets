// bring the input/output library (from the standard library) into scope
use std::io;

// the main function is the entry point of the program
fn main() {
    println!("\x1b[31;1mWelcome to the number guessing game!\x1b[0m");
    println!("We are going to have so much \x1b[33;1mfun\x1b[0m ^^");
    // ask for user input
    println!("\x1b[36;1mPlease input a number between 1 and 100!\x1b[0m");

    // process that input
    // the let statement is used to create a new variable
    // variables are mutable by default, `mut` allows variable to mutate
    // String::new() returns a new instance of a String (standard library)
    // :: indicates that new is an associated function of the String type (static method)
    // the new instance of String is an empty string
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line...");
    println!("You just entered: \x1b[36;1m{}\x1b[0m", guess);

    // check that the input is in the expected form
}
