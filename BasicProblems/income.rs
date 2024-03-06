fn main(){
    let mut s = String::from("hello");
    s.push_str(", world!"); //appends a literal to a string
    println!("{}",s); //this will print `hello, world!`
}