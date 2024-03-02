//use rand::Rng;
//use std::cmp::Ordering;
// use std::io;
// fn main(){
//     //Taking input from user 
//     println!("Please input year");
//     let mut year = String::new();
//     io::stdin()
//     .read_line(&mut year)
//     .expect("Failed to read line");

//     // let year: u32 = match year.trim().parse() {
//     //     Ok(num) => num,
//     //     Err(_) => continue,
//     // }
//     println!("You guessed: {}", year);
// }
use std::io;

fn main() {
    // Taking input from user
    println!("Please input year");
    let mut year = String::new();
    io::stdin()
        .read_line(&mut year)
        .expect("Failed to read line");

    // Convert input to integer
    let year: u32 = match year.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid year.");
            return;
        }
    };

    // Check if it's a leap year
    if is_leap_year(year) {
        println!("{} is a leap year.", year);
    } else {
        println!("{} is not a leap year.", year);
    }
}

// Function to check leap year
fn is_leap_year(year: u32) -> bool {
    if year % 4 != 0 {
        return false;
    } else if year % 100 != 0 {
        return true;
    } else if year % 400 != 0 {
        return false;
    } else {
        return true;
    }
}
