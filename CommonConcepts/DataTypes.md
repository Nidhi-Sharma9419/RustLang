Rust has two data type subsets:
1. Scalar
2. Compound

Rust is statically typed language

#Scalar
Represents single value

1. integers
For signed integer it starts with i (when we need to sign it posotive or negative)
For unsigned integer it starts with u (when its only positive)
 Note that all number literals except the byte literal allow a type suffix, such as 57u8, and _ as a separator, such as 1_000

 Default integer type is i32 , its fastest even on 64-bit systems

2. floating-point numbers
f32 : single precision float
f64 : double precision float

Default type is f64 because on modern CPUs it's roughly the same speed as f32 but is capable of more precision

fn main() {
 let x = 2.0; // f64
 let y: f32 = 3.0; // f32
}

3. Booleans
true
false

fn main() {
 let t = true;
 let f: bool = false; // with explicit type annotation
}

4. Characters
Represents Unicode Scalar Value, it means more than just ASCII, Chinese, Japanese, Korean and Emoji
anges from U+0000 to U+D7FF and U+E000 to U+10FFFF 
char : use single quote ''
string : use double quote ""

fn main() {
 let c = 'z';
 let z = 'Ƶ';
 let heart_eyed_cat = '😻';
}


#Compound
Group multiple values in one type

1. tuples
fn main() {
 let tup: (i32, f64, u8) = (500, 6.4, 1);
}

fn main() {
 let tup = (500, 6.4, 1);
 let (x, y, z) = tup; //destructuring
 println!("The value of y is: {}", y);
}

we can access a tuple element directly by using a period (.) followed by the index of the value we want to access.

fn main() {
 let x: (i32, f64, u8) = (500, 6.4, 1);
 let five_hundred = x.0;
 let six_point_four = x.1;
 let one = x.2;
}

This program creates a tuple, x, and then makes new variables for each element by using their index. As with most programming languages, the first index in a tuple is 0.


2. arrays
Every element of the array must be of the same size.
arrays in Rust have a fixed length: once declared, they cannot grow or shrink in size.
fn main() {
 let a = [1, 2, 3, 4, 5];
}

  - Accessing Array Elements 
  An array is a single chunk of memory allocated on the stack. You can access 
elements of an array using indexing, like this:
fn main() {
 let a = [1, 2, 3, 4, 5];
 let first = a[0];
 let second = a[1];
}

For this code 
fn main() {
 let a = [1, 2, 3, 4, 5];
 let index = 10;
 let element = a[index];

 println!("The value of element is: {}", element);
}
It will generate runtime error
BENEFIT : Rust protects you against 
this kind of error by immediately exiting instead of allowing the memory 
access and continuing.


