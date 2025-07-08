use std::thread;
use std::time::Duration;
fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the thread spawned");
            thread::sleep(Duration::from_millis(i));
        }
    });
    // handle.join().unwrap();
    for i in 1..5 {
        println!("hi number {i} from the main");
        thread::sleep(Duration::from_millis(i));
    }
}
