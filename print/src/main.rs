// * the main function is the first thing run for any executable Rust program
// * to run a Rust program, you must first compile it, generating a binary executable
// * Rust is an ahead-of-time compiled language
fn main() {
    // * Rust indents with 4 spaces, no tabs
    // * Rust does metaprogramming through `macros` (ending with !)
    println!("Hello world");
    // * string interpolation is easy!
    println!("Here are some arguments, {}, {}, {}", "arg1", "arg2", "arg3");
    // * constructing a string using std::fmt
    let statement_a = format!("Hello {} and {}\n", "Henry", "James");
    /*
        println! macro looks at string at compile time and verifies
        that the arguments and argument specifiers match in amount and type
    */
    /*
        this will not work -> `println!(statementA);` but the following will
        work (again, string interpolation)
    */
    println!("{}", statement_a);

    let x = 12;
    let y = 15;
    println!("x = {}, y = {}, x + y = {}", x, y, x + y);
}
