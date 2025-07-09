use colored_json::ToColoredJson;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Serialize, Deserialize)]
struct Todo {
    userId: u32,
    id: u32,
    title: String,
    completed: bool,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://jsonplaceholder.typicode.com/todos/1";
    let resp = reqwest::get(url).await?.json::<Value>().await?;
    let pretty = serde_json::to_string_pretty(&resp)?;
    println!("{}", pretty.to_colored_json_auto()?);
    //post request
    let client = reqwest::Client::new();
    let todo = Todo {
        userId: 32,
        id: 2,
        title: String::from("dp"),
        completed: false,
    };
    let post_response = client
        .post(url)
        .json(&todo)
        .send()
        .await?
        .json::<Todo>()
        .await?;
    let pretty_post = serde_json::to_string_pretty(&post_response)?;
    println!("{}", pretty_post.to_colored_json_auto()?);
    Ok(())
}
