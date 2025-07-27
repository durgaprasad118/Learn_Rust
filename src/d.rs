use std::num::ParseIntError;

fn parse_temp(s: &str) -> Result<i32, ParseIntError> {
    s.parse::<i32>()
}
fn main() {
    let t = "25";
    let t2 = "cold";
    // let converted_temp = parse_temp(&t2);
    // match converted_temp {
    //     Ok(temp) => println!("{}", temp),
    //     Err(e) => println!("Error is {}", e),
    // }
    if let Ok(x) = parse_temp(&t) {
        println!("Temp is : {}", x);
    } else {
        println!("Not  a valid temperature");
    }
}
