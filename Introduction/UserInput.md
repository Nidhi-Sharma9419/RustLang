io::stdin()// This has been written because std::io; library has already been impoorted in main function. 
//If this wouldn't have been done then we get to write "std::io::Stdin"
        .read_line(&mut guess)   // This line calls the read_line method , argument written inside it is to tell what string to store the user input in(it appends not overwrite)
        string argument should be mutable so that the method can change the string's content.

        & indicates reference 
        like variables , references are immutable by default
        hen ce to make it mutable we write &mut instead of &guess


HANDLING POTENTIAL FAILURE
.expect("Failed to read line");

We can write this code as:
io::stdin().read_line(&mut guess).expect("Failed to read line");

Its good to break up long lines , hence we call method by
.method_name()
