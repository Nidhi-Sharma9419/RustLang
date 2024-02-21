Constants and shadowing

Like immutable variables, constants are values that are bound to a name and are not allowed to change, but there are a few differences between constants and variables.

#Differences
Constants
1. Don't use mut with contsants . Constants are always immutable
2. Use const keyword instead of let while declaring constants
3. Constants may be set to a expression but not to a function call or any other value which can be computed at the runtime

Shadowing
1. Shadowing means you can declare a new variable with the same as previous variable.
2. E.g.   fn main()
              let x=5;
              let x=x+1;     //use previous value of x
              println!("The value is: {}", x);  // 6
3. It's different than marking a variable as mut (we will get compile time error if we do so)
4. One advantage is we are allowed to use same name with different data type whereas we can't so this with mut
It spares us from using space_str and space_num variable each time as per data type.
