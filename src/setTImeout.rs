use tokio::time::{interval, sleep, Duration};
#[tokio::main]
async fn main() {
    println!("Hello ");
    loop {
        sleep(Duration::from_secs(1)).await;
        println!("Hello after two seconds");
    }
    // let mut ticker = interval(Duration::from_secs(1));
    // ticker.tick().await;
    // println!("Hello after ticking 1sec");
}
