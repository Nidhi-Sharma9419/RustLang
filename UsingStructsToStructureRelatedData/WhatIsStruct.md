A struct, or structure, is a custom data type 
that lets you name and package together 
multiple related values that make up a 
meaningful group.

-Definind and Instantiating Structs

`struct` keyword is used to define the struct
E.g.

`
struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
`
Inside curly brackets names and types are called 'fields'

Now create 'instance'
-> stating name of struct
-> curly brackets containing 'key:value' pairs
E.g.

`
let user1 = user{
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
}
`
No need for fields to be in same order

How to get those value:  user1.email

If instance is mutatble we can change a value 
E.g.
`
let mut user1 = User {
 email: String::from("someone@example.com"),
 username: String::from("someusername123"),
 active: true,
 sign_in_count: 1,
};
user1.email = String::from("anotheremail@example.com");
`
NOTE: Entire instance must be mutable
It makes sense to name the function parameters with the same name as 
the struct fields.

#SEE struct.rs for implementation pov



## Using the `Field Init Shorthand` When Variables and Fields 
Have the Same Name
`
fn build_user(email: String, username: String) -> User {
 User {
 email,
 username,
 active: true,
 sign_in_count: 1,
 }
}
`

## Creating Instances from Other Instances with Struct Update Syntax\
`struct update syntax`
E.g.
Creating new instance
Without update syntax:
`
let user2 = User {
 email: String::from("another@example.com"),
 username: String::from("anotherusername567"),
 active: user1.active,
 sign_in_count: user1.sign_in_count,
};
`

Using struct update syntax
`
let user2 = User {
 email: String::from("another@example.com"),
 username: String::from("anotherusername567"),
 ..user1
};
`

## Using Tuple Structs Without Named Fields to Create Different Types
`tuple structs`

E.g. 
*struct struct name (types in tuple)*
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0,0,0);
let origin = Point(0,0,0);

e, a 
function that takes a parameter of type Color cannot take a Point as an argument, even though both types are made up of three i32 values. 
This is what differenntiate struct tuples from tuples.

However, tuple struct instances behave similarly to tuples in that you can destructure them into their individual pieces, access individual values using dot notation followed by the index, etc. For example:
`
let color = Color(255, 0, 0);
let Point(x, y, z) = origin;

println!("Red value: {}", color.0); // Accessing the first field of the color tuple struct
println!("X coordinate: {}", x);    // Accessing the first field of the point tuple struct after destructuring
`

## Unit-Like Structs Without Any Fields
Structs that don't have any fields: `unit-like structs`  (),the unit type

It’s possible for structs to store references to data owned by something 
else, but to do so requires the use of `lifetimes`

SEE code rectangles.rs

## Refactoring with Tuples
`
fn main() {
 let rect1 = (30, 50);
 println!(
 "The area of the rectangle is {} square pixels.",
  $area(rect1)
 );
}
fn area(dimensions: (u32, u32)) -> u32 {
 @ dimensions.0 * dimensions.1
}
`

Tuples let us add a bit of structure, and 
we’re now passing just one argument $. But in another way, this version is less 
clear: tuples don’t name their elements, so our calculation has become more 
confusing because we have to index into the parts of the tuple @.


## Refactoring with Structs: Adding More Meaning
area function is now defined with one parameter
rectangle, whose type is an immutable borrow of a struct Rectangle
instance
we want to borrow the struct rather than take ownership of it.
This way, main retains its ownership and can continue using rect1, which is the reason we use the & in the function signature 
and where we call the function.
The area function accesses the width and height fields of the Rectangle instance . 
it gives descriptive names to the values rather than using the tuple index values of 0 and 1.

This is a win for clarity.

struct Rectangle {
 width: u32,
 height: u32,
}
fn main() {
 let rect1 = Rectangle { width: 30, height: 50 };
 println!(
 "The area of the rectangle is {} square pixels.",
 area(&rect1)
 );
}
fn area(rectangle: &Rectangle) -> u32 {
 rectangle.width * rectangle.height
}

## Adding Useful Functionality with Derived Traits
`Debug`
 The println! macro call will now look like println!("rect1 is 
{:?}", rect1);. Putting the specifier :? inside the curly brackets tells println! we want to use an output format called Debug.



Functionality to print debugging information ,we add the annotation `#[derive(Debug)]` just before the struct definition


bit easier to read; in 
those cases, we can use `{:#?}` instead of {:?} in the println! string. 

It will output like :
`
rect1 is Rectangle {
 width: 30,
 height: 50
}
`