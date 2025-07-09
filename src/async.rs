use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    userId: u32,
    id: u32,
    title: String,
    completed: bool,
}
#[tokio::main]
async fn main() {
    let _ = my_fn().await;
}

async fn my_fn() -> Result<(), Error> {
    let resp: Todo = reqwest::get("https://jsonplaceholder.typicode.com/todos/1")
        .await?
        .json()
        .await?;
    println!("{:?}", resp);
    let new_todo = Todo {
        userId: 1,
        id: 34,
        title: String::from("Hello there Bhaaaaai"),
        completed: false,
    };
    let client = reqwest::Client::new();
    let res = client
        .post("https://jsonplaceholder.typicode.com/todos")
        .json(&new_todo)
        .send()
        .await?
        .json::<Todo>()
        .await?;
    println!("{:?}", res);
    let client = reqwest::Client::new();
    let resother = client
        .put("https://jsonplaceholder.typicode.com/todos/1")
        .json(&new_todo)
        .send()
        .await?
        .json::<Todo>()
        .await?;
    println!("{:?}", resother);

    let client = reqwest::Client::new();
    let dlete_entry = client
        .delete("https://jsonplaceholder.typicode.com/todos/1")
        .send()
        .await?;
    println!("{:?}", dlete_entry.text().await?);
    Ok(())
}
