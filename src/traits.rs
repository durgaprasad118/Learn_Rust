trait Summary {
    fn Summarize(&self) -> String;
}
struct NewsArticle {
    headline: String,
    content: String,
    author: String,
}
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}
fn main() {
    let tweet = Tweet {
        username: String::from("dp"),
        retweet: false,
        reply: false,
        content: String::from("Hey There"),
    };
    let newsart = NewsArticle {
        headline: String::from("asdf"),
        content: String::from(""),
        author: String::from("dp"),
    };
    news(tweet);
    news(newsart)
}
// this is the shared behaviour
impl Summary for Tweet {
    fn Summarize(&self) -> String {
        let content = format!("Tweet by :{}", self.username);
        content
    }
}
impl Summary for NewsArticle {
    fn Summarize(&self) -> String {
        let content = format!("News by :{}", self.author);
        content
    }
}
fn news(source: impl Summary) {
    println!("There is a news in the market");
    println!("{}", source.Summarize());
}
fn news_aggregator_tweet(tweet: Tweet) {
    println!("There is a news in market");
    println!(
        "The news is that {} and is publilshed by {}",
        tweet.content, tweet.username
    );
}

fn news_aggregator_news(news: NewsArticle) {
    println!("There is a news in market");
    println!(
        "The news is that {} and is publilshed by {}",
        news.content, news.author
    );
}
