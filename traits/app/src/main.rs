use std::fmt::{Debug, Display};

fn main() {
    let tweet = Tweet {
        username: String::from("john_doe"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summerize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
            hockey team in the NHL.",
        ),
    };
    println!("New article available! {}", article.summerize());
}

pub trait Summary {
    fn summerize_author(&self) -> String;

    fn summerize(&self) -> String {
        format!("(Read more from {}...)", self.summerize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summerize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summerize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summerize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// pub fn notify<T: Summary>(item: &T) {
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summerize());
}

// pub fn lot_trait_bound<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
pub fn lot_trait_bound<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    1
}

pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("Mary"),
        content: String::from("hello world"),
        reply: false,
        retweet: false,
    }
}
