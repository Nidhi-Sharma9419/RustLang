use std::io;
fn main(){
    println!("Enter Number");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");

    let n: u32 = match n.trim().parse(){
        Ok(num) => num,
        Err(_) =>{
            println!("Invalid input. Please enter a valid year.");
            return;
        }
    };
    println!("You number is: {}",n);
    if is_prime(n){
        println!("{} number is prime",n);
    }
    else{
        println!("{} number is not prime",n);
    }
}

fn is_prime(n: u32)-> bool{
    let mut i = 2;
    let mut f= false;
    while i<n {
        if n%i == 0{
            return false;
           
        }
        else{
            f= true
        }
        i=i+1;
    }
    return f;
}