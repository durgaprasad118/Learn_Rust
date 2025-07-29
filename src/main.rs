mod file;
use file::my_func;
mod greetings {
    fn english() {
        println!("Hello");
    }
    pub fn spanish() {
        println!("Hola");
    }
    pub mod formal {
        pub fn english_formal() {
            println!("Have a good day");
        }
    }
}
fn main() {
    // greetings::spanish();
    // greetings::formal::english_formal();
    my_func();
}
