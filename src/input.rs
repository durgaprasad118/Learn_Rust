
use std::io;
fn main(){
    let mut input = String::new();
    print!("Enter the string: ");
    // here we are passing refrence of changeable input 
    io::stdin().read_line(&mut input).expect("Failed to read line");
    println!("{}",input);
}
