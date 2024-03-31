# Another enum in standard library : Option
First one was IpAddr

Null is a value that means there is no value there. 

As such, Rust does not have nulls, but it does have an enum that 
can encode the concept of a value being present or absent. This enum is Option<T>, and it is defined by the standard library as follows:
`
enum Option<T> {
 Some(T),
 None,
}
`
you don’t need to bring it into scope explicitly.
 <T> means the Some variant of the Option enum can hold one piece of data of any type. 

 Some more examples:
 `
 let some_number = Some(5);
 let some_string = Some("a string");
 let absent_number: Option<i32> = None;
 `
 type i32 is explicitly defined while using None because Rust is a statically typed language, and the compiler needs to know the types of all variables at compile time. Explicitly specifying the type helps the compiler ensure type safety and prevents errors related to mismatched types.

 It is different from having Null as value because Option<T> and T (where T can be any type) are different 
types, the compiler won’t let us use an Option<T> value as if it were definitely a valid value. For example, this code won’t compile because it’s trying to add an i8 to an Option<i8>:

`
let x: i8 = 5;
let y: Option<i8> = Some(5);
let sum = x + y;
`
Above code will give error
In other words, you have to convert an Option<T> to a T before you can perform T operations with it. 
/* The Option<T> enum has a large 
number of methods that are useful in a variety of situations; you can check them out in its documentation. */

