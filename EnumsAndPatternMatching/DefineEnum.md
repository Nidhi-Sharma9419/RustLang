 Enums allow you to define a type by enumerating its possible 
values.

Defining an Enum
----------------

Creating enumeration `IpAddrKind`
where V4 and V6 are called variants

enum IpAddrKind{
    V4,
    V6,
}

IpAddrKind is custon data type

Enum Values
-----------

Crreating instance of each of two variants of IpAddrKind
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;

enum are namespaced under its identifier , using double colon to separate the

/*
In a general programming context, a namespace is a container that holds a collection of names or identifiers. Namespaces are used to organize and manage identifiers in a way that avoids naming conflicts.

An identifier, on the other hand, is a name given to a program element, such as a variable, function, class, or module. Identifiers are used to uniquely identify and refer to these elements within a program.
*/

# Defining a function that takes any IpAddrKind

fn route(ip_type: IpAddrKind) { }

Now we can call this function with either variant
route(IpAddrKind::V4);
route(IpAddrKind::V6);

Way to store actual Ip Address data using structs , given below are examples:
`
enum IpAddrKind {
 V4,
 V6,
}
struct IpAddr {
  kind: IpAddrKind,
  address: String,
}
let home = IpAddr {
 kind: IpAddrKind::V4,
 address: String::from("127.0.0.1"),
};
let loopback = IpAddr {
 kind: IpAddrKind::V6,
 address: String::from("::1"),
};
`
We’ve used a struct to bundle the kind and address values together, so now the variant is associated with the value.


Doing same thing without using struct
`
enum IpAddr {
 V4(String),
 V6(String),
}
let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));
`
Attaching data to each variant of the enum directly
ADVANTAGE OF USING ENUM OVER STRUCT
Version four type IP addresses will always have four numeric components that will have values between 0 and 255. If we wanted to store V4 addresses as four u8 values but still express V6 addresses as one String value, we wouldn’t be able to with a struct. Enums handle this case with ease.

`
enum IpAddr {
 V4(u8, u8, u8, u8),
 V6(String),
}
let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
`

the standard library 
defines IpAddr: it has the exact enum and variants that we’ve defined and used, but it embeds the address data inside the variants in the form of two different structs, which are defined differently for each variant:
`
struct Ipv4Addr {
 // --snip--
}
struct Ipv6Addr {
 // --snip--
}
enum IpAddr {
 V4(Ipv4Addr),
 V6(Ipv6Addr),
}
`

When wide variety of types embedded in its variants
`
enum Message {
 Quit,                     // It has no data associated with it at all
 Move { x: i32, y: i32 },//Move includes an anonymous struct inside it
 Write(String),        //Write includes an anonymous struct inside it
 ChangeColor(i32, i32, i32), //ChangeColor includes three i32 values
}
`

 The following structs could hold the same data that the preceding enum variants hold:
 `
 struct QuitMessage; // unit struct
struct MoveMessage {
 x: i32,
 y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
 `

 Definining a function to take any type of these message is difficult when using struct but with enum it's not.

Here’s a method named call that we could define on our Message enum:
 `
 impl Message {
 fn call(&self) {
 // method body would be defined here
 }
}
let m = Message::Write(String::from("hello"));
m.call();
 `
The body of the method would use self to get the value that we called 
the method on.
Value Message::Write(String::from("hello")) is what self will be in 
the body of the call method  when m.call() runs.

