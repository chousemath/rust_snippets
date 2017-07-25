// bring the input/output library (from the standard library) into scope
use std::io;

// the main function is the entry point of the program
fn main() {
    println!("\x1b[31;1mWelcome to the number guessing game!\x1b[0m");
    println!("We are going to have so much \x1b[33;1mfun\x1b[0m ^^");
    // ask for user input
    println!("\x1b[36;1mPlease input a number between 1 and 100!\x1b[0m");

    // process that input
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line...");
    println!("You just entered: \x1b[36;1m{}\x1b[0m", guess);

    // check that the input is in the expected form
}
