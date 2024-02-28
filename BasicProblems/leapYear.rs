//use rand::Rng;
//use std::cmp::Ordering;
use std::io;
fn main(){
    //Taking input from user 
    println!("Please input year");
    let mut year = String::new();
    io::stdin()
    .read_line(&mut year)
    .expect("Failed to read line");

    // let year: u32 = match year.trim().parse() {
    //     Ok(num) => num,
    //     Err(_) => continue,
    // }
    println!("You guessed: {}", year);
}