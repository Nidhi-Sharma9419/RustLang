"fn" allows you to declare new functions
"main" entry point of programs
E.g.
fn main() {
 println!("Hello, world!");
 another_function();
}

FUNCTION PARAMETERS
Concrete values are called arguments
Providing concrete values for parameters

fn main() {
 another_function(5);
}
fn another_function(x: i32) {
 println!("The value of x is: {}", x);
}