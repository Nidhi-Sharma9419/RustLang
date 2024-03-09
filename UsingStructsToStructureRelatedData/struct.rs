    // Define a struct named User
struct User {
    email: String,
    username: String,
    active: bool,
    sign_in_count: u64,
}
fn main(){
    // Create a new User instance by calling the build_user function
let user1 = build_user(String::from("example@example.com"), String::from("example_username"));

// Print out the values of the fields of the user1 instance
println!("User email: {}", user1.email);
println!("Username: {}", user1.username);
println!("Active: {}", user1.active);
println!("Sign-in count: {}", user1.sign_in_count);

}

fn build_user(email: String, username: String) -> User {
    // Construct a new User instance using the provided parameters and default values for active and sign_in_count
    User {
        email: email,           // Assign the provided email to the email field of the User struct
        username: username,     // Assign the provided username to the username field of the User struct
        active: true,           // Set the active field of the User struct to true
        sign_in_count: 1,       // Set the sign_in_count field of the User struct to 1
    }
}

/*
This will output:
User email: example@example.com
Username: example_username
Active: true
Sign-in count: 1
*/