fn main() {
    let bigger_num = largest_val(2, 44);
    let bigger_char = largest_val('a', 'b');
    println!("{}", bigger_num);
    println!("{}", bigger_char);
}
fn largest_val<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}
// fn largest_char(a: char, b: char) -> char {
//     if a > b {
//         a
//     } else {
//         b
//     }
// }
//
// fn largest_i32(a: i32, b: i32) -> i32 {
//     if a > b {
//         a
//     } else {
//         b
//     }
// }
