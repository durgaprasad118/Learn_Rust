// use std::io::{self, Read};
// struct User {
//     first_name: String,
//     last_name: String,
//     age: u32,
// }
// impl User {
//     fn print_username(&self) {
//         println!("The user name is : {},{}", self.first_name, self.last_name)
//     }
//     fn print_num(&self, num: i32) {
//         println!("The num is {num } for the user :{}", self.first_name);
//     }
//     fn just_sy_hello() {
//         println!("Hello there");
//     }
// }
enum Shape {
    Rectangle(f64, f64),
    Circle(f64),
}
fn main() {
    let my_shape = Shape::Rectangle(3.0, 4.0);
    let ar = area(my_shape);
    print!("{}", ar)
    // let user1 = User {
    //     first_name: String::from("durga"),
    //     last_name: String::from("prasad"),
    //     age: 22,
    // };
    // println!("The username is {:?}", user1.first_name);
    // user1.print_username();
    // user1.print_num(2);
    // User::just_sy_hello();
    // let mut str = String::new();
    // io::stdin()
    //     .read_line(&mut str)
    //     .expect("failed to read input");
    // let num: i64 = str.trim().parse().unwrap();
    // println!("The entered number {num} is , {}`", is_even(num));
    // println!("the 4th fibo is {}", nth_fibo(4));
    // let str = String::from("dp");
    // print!("The lenght of the string is : {}", getstr_len(&str));
}
fn area(shape: Shape) -> f64 {
    match shape {
        Shape::Rectangle(a, b) => a * b,
        Shape::Circle(a) => a * a,
    }
}

// fn is_even(a: i64) -> bool {
//     if a % 2 == 0 {
//         return true;
//     }
//     false
// }
//
// // find the nth fibonaci
// fn nth_fibo(a: u32) -> u32 {
//     let mut first = 0;
//     let mut second = 1;
//     if a == 0 {
//         return 0;
//     }
//     if a == 1 {
//         return 1;
//     }
//     for _ in 0..(a - 1) {
//         let temp = second;
//         second = second + first;
//         first = temp;
//     }
//     second
// }
//
// //gt string length
// fn getstr_len(str: &String) -> usize {
//     str.chars().count()
// }
