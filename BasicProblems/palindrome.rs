use std::io;

fn main() {
    // Taking input from user
    println!("Please input a number:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Convert input to integer
    let number: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return;
        }
    };

    // Check if it's a palindrome
    if is_palindrome(number) {
        println!("{} is a palindrome.", number);
    } else {
        println!("{} is not a palindrome.", number);
    }
}

// Function to check if a number is palindrome
fn is_palindrome(number: i32) -> bool {
    let mut n = number;
    let mut reversed = 0;

    while n > 0 {
        let digit = n % 10;
        reversed = reversed * 10 + digit;
        n /= 10;
    }

    number == reversed
}
