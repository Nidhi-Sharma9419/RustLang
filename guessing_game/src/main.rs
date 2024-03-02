  /*use std::io;
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
}*/

/*use std::io;
use rand::Rng;

use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);//start..=end
    //Rng trait defines methods that random number generators implement, and this trait must be in scope for us to use those methods.
    //rand::thread_rng function that gives us the particular random number generator we’re going to use
    //gen_range method is defined by the Rng trait that we brought into scope with the use rand::Rng; statement.

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        //let guess: u32 = guess.trim().parse().expect("Please type a number!");  
        /*We bind this new variable to the expression guess.trim().parse(). The guess in the expression refers to the original guess variable that contained the input as a string. The trim method on a String instance will eliminate any whitespace at the beginning and end, which we must do to be able to compare the string to the u32, which can only contain numerical data. The user must press enter to satisfy read_line and input their guess, which adds a newline character to the string. For example, if the user types 5 and presses enter, guess looks like this: 5\n. The \n represents “newline.” (On Windows, pressing enter results in a carriage return and a newline, \r\n.) The trim method eliminates \n or \r\n, resulting in just 5.*/  
        //The parse method on strings converts a string to another type
        
        
        //Handling invalid input
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        //Ordering::Equal => println!("You win!"),
        Ordering::Equal => {
            println!("You win!");
            break;
        }
    }
}
*/
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}