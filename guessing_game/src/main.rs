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
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}