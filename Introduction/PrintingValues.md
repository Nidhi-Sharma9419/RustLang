println!("You guessed: {guess}");
{} is a placeholder that holds the valuein place



When printing the result of evaluating an expression, place empty curly brackets in the format string, then follow the format string with a comma-separated list of expressions to print in each empty curly bracket placeholder in the same order. 
E.g.
let x = 5;
let y = 10;

println!("x = {x} and y + 2 = {}", y + 2);
