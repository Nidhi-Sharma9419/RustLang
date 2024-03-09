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