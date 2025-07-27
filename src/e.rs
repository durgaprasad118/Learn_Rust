fn always_succeeds() -> Result<i32, String> {
    Ok(42)
}
fn sometime_fails(input: bool) -> Result<i32, String> {
    if input {
        Ok(100)
    } else {
        Err("No ".to_string())
    }
}
fn main() {
    // let v1 = always_succeeds().unwrap();
    // println!("{}", v1);

    let v2 = sometime_fails(false);
    if let Ok(x) = v2 {
        println!("{}", x);
    } else {
        println!("No value");
    }
}
