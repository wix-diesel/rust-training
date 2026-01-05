pub mod tweet;
use tweet::Tweet;
use crate::tweet::Summary;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = tweet::NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        content: String::from("The Pittsburgh Penguins once again are the best \
                              hockey team in the NHL."),
        author: String::from("John Doe"),
        publication_date: String::from("2023-04-15"),
    };

    println!("New article available!: {}", article.summarize());
}
