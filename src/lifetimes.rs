struct User<'a> {
    name: &'a str,
}
fn main() {
    let a = String::from("dp");
    let user = User { name: &a };
    println!("{}", user.name);
}
