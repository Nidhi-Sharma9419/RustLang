# The match control Flow operator

control flow operator : match
It allows you to compare a value against a series of patterns and then execute code based on which pattern matches.

Example:
Think of a match expression as being like a coin-sorting machine:
coins slide down a track with variously sized holes along it, and each coin falls through the first hole it encounters that it fits into. 

`
enum Coin {
 Penny,
 Nickel,
 Dime,
 Quarter,
}
fn value_in_cents(coin: Coin) -> u32 {
 match coin {    //The type of 'coin' is Coin enum 
 Coin::Penny => 1,
 Coin::Nickel => 5,
 Coin::Dime => 10,
 Coin::Quarter => 25,
 }
}
`
An arm has two parts: a pattern and some code. 
The first arm here has a pattern that is the value Coin::Penny and then the => operator that separates the pattern and the code to run
The code in this case is just the value 1.
Each arm is separated by comma.


Curly brackets typically aren’t used if the match arm code is short, 
as it is in Listing 6-3 where each arm just returns a value. If you want to run multiple lines of code in a match arm, you can use curly brackets. For example, the following code would print “Lucky penny!” every time the 
method was called with a Coin::Penny but would still return the last value of the block, 1:

`
fn value_in_cents(coin: Coin) -> u32 {
 match coin {
 Coin::Penny => {
 println!("Lucky penny!");
 1
 },
 Coin::Nickel => 5,
 Coin::Dime => 10,
 Coin::Quarter => 25,
 }
}
`

-> Patterns that bind to Values
Adding this information to our enum by changing the Quarter variant to include a UsState value stored inside it.
`
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
 Alabama,
 Alaska,
 // --snip--
}
enum Coin {
 Penny,
 Nickel,
 Dime,
 Quarter(UsState),
}
`

Let’s imagine that a friend of ours is trying to collect all 50 state quarters
`
fn value_in_cents(coin: Coin) -> u32 {
 match coin {
 Coin::Penny => 1,
 Coin::Nickel => 5,
 Coin::Dime => 10,
 Coin::Quarter(state) => {
 println!("State quarter from {:?}!", state);
 25
 },
 }
}
`
If we were to call value_in_cents(Coin::Quarter(UsState::Alaska)), 
coin would be Coin::Quarter(UsState::Alaska). When we compare that 
value with each of the match arms, none of them match until we reach
Coin::Quarter(state). At that point, the binding for state will be the value UsState::Alaska. We can then use that binding in the println! expression, thus getting the inner state value out of the Coin enum variant for Quarter.


-> Matching with Option<T>

We can also handle Option<T> using match as we did 
with the Coin enum! Instead of comparing coins, we’ll compare the variants of Option<T>, but the way that the match expression works remains the same.

Let’s say we want to write a function that takes an Option<i32> and, if there’s a value inside, adds 1 to that value. If there isn’t a value inside, the function should return the None value and not attempt to perform any operations.

`
fn plus_one(x: Option<i32>) -> Option<i32> {
 match x {
  None => None,
  Some(i) => Some(i + 1),
 }
}
let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
`

-> Matches are Exhaustive

`
fn plus_one(x: Option<i32>) -> Option<i32> {
 match x {
 Some(i) => Some(i + 1),
 }
}
`
We didn’t handle the None case, so this code will cause a bug.IT WILL GIVE ERROR.

Matches in Rust are exhaustive: we must exhaust every last possibility in order for the code to be valid.

-> The _ Placeholder

Rust also has a pattern we can use when we don’t want to list all possible values. For example, a u8 can have valid values of 0 through 255. If we only care about the values 1, 3, 5, and 7, we don’t want to have to list out 0, 2, 4, 6, 8, 9 all the way up to 255. Fortunately, we don’t have to: we can use the special pattern _ instead:
`
let some_u8_value = 0u8;
match some_u8_value {
 1 => println!("one"),
 3 => println!("three"),
 5 => println!("five"),
 7 => println!("seven"),
 _ => (),
}
`
The () is just the unit value, so nothing will happen in the _ case.

# Concise Control Flow with if let
The if let syntax lets you combine if and let into a less verbose way to handle values that match one pattern while ignoring the rest. 

Consider the 
program in Listing 6-6 that matches on an Option<u8> value but only wants to execute code if the value is 3.
`
let some_u8_value = Some(0u8);
match some_u8_value {
 Some(3) => println!("three"),
 _ => (),
}
`
We can write this in shorter way using 'if let'
`
if let Some(3) = some_u8_value {
 println!("three");
}
`
The syntax if let takes a pattern and an expression separated by an 
equal sign. It works the same way as a match, where the expression is given to the match and the pattern is its first arm.
but we are losing exhaustive checking.


Below two codes works same , but one has 'else' in it
`
let mut count = 0;
match coin {
 Coin::Quarter(state) => println!("State quarter from {:?}!", state),
 _ => count += 1,
}
`
`
let mut count = 0;
if let Coin::Quarter(state) = coin {
 println!("State quarter from {:?}!", state);
} else {
 count += 1;
}
`