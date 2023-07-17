create a variable to store the user input, like this:

    let mut guess = String::new();
    line has created a mutable variable that is currently bound to a new, empty instance of a String. 
 We use the let statement to create the variable. Here’s another example:

    let apples = 5;

In Rust, variables are immutable by default.    

let apples = 5; // immutable
let mut bananas = 5; // mutable

String::new, a function that returns a new instance of a String. 
The :: syntax in the ::new line indicates that new is an associated function of the String type.
An "associated" function is a function that’s implemented on a type, in this case String. 

This new function creates a new, empty string. 


