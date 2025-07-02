use chrono::{Local, Utc};
fn main() {
    let now = Utc::now();
    println!("Current time and date in UTC is: {}", now);
    let local = Local::now();
    println!("Local time is : {}", local);
}
