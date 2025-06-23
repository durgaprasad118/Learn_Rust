

use std::io;
fn main(){

    let mut input:String = String::new();
    print!("Enter : ");
    io::stdin().read_line(&mut input).expect("Can't read line");
    let int_input:i64 = input.trim().parse().unwrap();
    println!("The value of the input is : {}",int_input);
}
