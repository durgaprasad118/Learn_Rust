trait Summary {
    fn Summarize(&self) -> String {
        return String::from("Summarize");
    }
}

struct User {
    name: String,
    age: u32,
}
impl Summary for User {
    fn Summarize(&self) -> String {
        return format!("Nmae is {} and age is {}", self.name, self.age);
    }
}
pub fn notify(u: &impl Summary) {
    println!("{}", u.Summarize());
}
fn main() {
    let user1 = User {
        name: String::from("dp"),
        age: 22,
    };
    println!("{}", user1.Summarize());
    notify(&user1);
}
