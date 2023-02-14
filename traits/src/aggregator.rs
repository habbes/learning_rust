use std::fmt::Debug;


pub trait Summary {
    // fn summarize(&self) -> String;

    fn summarize_author(&self) -> String;

    // default implementation
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // will use default implementation of summarize

    fn summarize_author(&self) -> String {
        return format!("{}", self.author);
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        return format!("@{}", self.username);
    }
}

// trait as function parameter
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize())
}


// trait bounds syntax
// pub fn notify<T: Summary>(item: &T) {
//     // ...
// }

// we can require the parameter to implement multiple traits
// pub fn notify<T: Summary + Display>(item: &T) {
//
// }

// trait bounds in where clause
// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {
// }

// trait as return type
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}