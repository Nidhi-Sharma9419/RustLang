use std::io;
// io stands for input/output library which comes from standard library
//Rust has a set of items defined in the standard library that it brings into the scope of every program. This set is called the prelude.
//If a type you want to use isn’t in the prelude, you have to bring that type into scope explicitly with a use statement.
//Exactly like I did here to bring "io"
fn main() {
//The fn syntax declares a new function;

    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        //read_line also returns a Result value. Result is "enumeration" (enum), it is a type that can be of multiple states, calling each possible state variant.
        //Results variants are Ok(Operation successful) and Err(Operation failed).
        //An instance of result has "expect" method
        //If you don’t call expect, the program will compile, but you’ll get a warning:
        //Just to give you warning of possible error that hasn't handled.
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}