## METHOD SYNTAX

In Rust, when defining methods for a struct, the first parameter of the method is always self. This self parameter represents the instance of the struct on which the method is called. It allows methods to access and modify the data contained within that particular instance of the struct.
`method syntax`:we add a dot followed by the method 
name, parentheses, and any arguments.
Let's illustrate this with an example:

```
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Method to calculate the area of the rectangle
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Method to resize the rectangle
    fn resize(&mut self, new_width: u32, new_height: u32) {
        self.width = new_width;
        self.height = new_height;
    }
}

fn main() {
    let mut my_rectangle = Rectangle { width: 5, height: 10 };

    // Accessing the area of the rectangle using a method
    println!("Area of the rectangle: {}", my_rectangle.area());
    //my_rectangle.area() this is method syntax
    // Resizing the rectangle using a method
    my_rectangle.resize(8, 12);

    println!("New width: {}", my_rectangle.width);
    println!("New height: {}", my_rectangle.height);
}
```
In this example, we define a Rectangle struct representing rectangles with width and height. We then implement two methods for this struct: area to calculate the area of the rectangle and resize to modify its dimensions.

Notice that both methods take self as their first parameter. When calling these methods, we don't need to explicitly pass the instance of the struct they are called on; Rust handles this automatically. This allows the methods to access the struct's data (width and height) using self, making it easy to manipulate the data associated with the struct instance.


// When you don't use self

If you don't use self as the first parameter in a method, it wouldn't be considered a method associated with a struct, but rather a regular function. In that case, you would need to pass the instance of the struct explicitly as a parameter to the function.

Here's an example demonstrating this:
`
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Method to calculate the area of the rectangle
    // Without self parameter
    fn area_without_self(rect: &Rectangle) -> u32 {
        rect.width * rect.height
    }

    // Method to resize the rectangle
    // Without self parameter
    fn resize_without_self(rect: &mut Rectangle, new_width: u32, new_height: u32) {
        rect.width = new_width;
        rect.height = new_height;
    }
}

fn main() {
    let mut my_rectangle = Rectangle { width: 5, height: 10 };

    // Accessing the area of the rectangle using a regular function
    println!("Area of the rectangle: {}", Rectangle::area_without_self(&my_rectangle));

    // Resizing the rectangle using a regular function
    Rectangle::resize_without_self(&mut my_rectangle, 8, 12);

    println!("New width: {}", my_rectangle.width);
    println!("New height: {}", my_rectangle.height);
}
`

In this modified example, area_without_self and resize_without_self are now regular functions instead of methods. They take the instance of the Rectangle struct as a parameter explicitly. When calling these functions, you need to pass the struct instance as an argument, unlike methods where you don't need to pass self explicitly.



Methods can take ownership of self, borrow self immutably as we’ve done here, or borrow self mutably, just as they 
can any other parameter.

## Automatic referencing and dereferencing
Rust has a feature called automatic referencing and dereferencing.
Here’s how it works: when you call a method with object.something(), 
Rust automatically adds in &, &mut, or * so object matches the signature of the method.

 In other words, the following are the same:
`p1.distance(&p2);`
`(&p1).distance(&p2);`
 Rust can figure out definitively whether the method is 
reading (&self), mutating (&mut self), or consuming (self).

FEATURES OF `impl`
## Methods with more parameters
 SEE canhold.rs

## Associated Functions
we’re allowed to define 
functions within impl blocks that don’t take self as a parameter.These are called `associated functions` because they’re associated with the struct. 
E.g `String::from` : associated function

Associated functions are often used for constructors that will return 
a new instance of the struct. 
E.g.
`
impl Rectangle {
 fn square(size: u32) -> Rectangle {
 Rectangle { width: size, height: size }
 }
}
`
To call this associated function, we use the :: syntax with the struct 
name; let sq = Rectangle::square(3); is an example.
This function is namespaced by the struct: the :: syntax is used for both associated functions and 
namespaces created by modules.

## Multiple impl Blocks
`
impl Rectangle {
 fn area(&self) -> u32 {
 self.width * self.height
 }
}
impl Rectangle {
 fn can_hold(&self, other: &Rectangle) -> bool {
 self.width > other.width && self.height > other.height
 }
}
`
