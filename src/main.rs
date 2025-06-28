//use rand::prelude::*;
//fn main() {
//    let mut rng = rand::rng();
//    println!("Random die roll: {}", rng.random_range(1..=10));
//}
//
//fn add(a: u32, b: u32) {
//    println!("{}", a);
//}
//
use std::fs::File;
enum Result<T, E> {
    Ok(T),
    Err(E),
}
fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };
}
